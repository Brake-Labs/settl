mod game;
mod player;
mod replay;
mod trading;
mod ui;

use clap::Parser;
use tokio::sync::mpsc;

/// Terminal Settlers of Catan with LLM Players
#[derive(Parser)]
#[command(name = "settl", about = "Play Settlers of Catan in your terminal with AI opponents")]
struct Cli {
    /// Number of players (2-4)
    #[arg(short, long, default_value = "4")]
    players: usize,

    /// Run a demo game with random AI players (no API keys needed)
    #[arg(long)]
    demo: bool,

    /// Model for LLM players (e.g. "claude-sonnet-4-6", "gpt-4o-mini")
    #[arg(short, long, default_value = "claude-sonnet-4-6")]
    model: String,

    /// Maximum turns before ending the game
    #[arg(long, default_value = "500")]
    max_turns: u32,

    /// Launch the TUI spectator mode
    #[arg(long)]
    tui: bool,

    /// Per-player models, comma-separated (e.g. "claude-sonnet-4-6,gpt-4o-mini,claude-sonnet-4-6,gpt-4o")
    #[arg(long)]
    models: Option<String>,

    /// Path to a TOML personality file to use for all LLM players
    #[arg(long)]
    personality: Option<String>,

    /// Replay a saved game (JSON replay or JSONL event log)
    #[arg(long)]
    replay: Option<String>,

    /// Random seed for reproducible games
    #[arg(long)]
    seed: Option<u64>,

    /// Resume a saved game from a JSON save file
    #[arg(long)]
    resume: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Handle replay mode.
    if let Some(ref replay_path) = cli.replay {
        let path = std::path::Path::new(replay_path);

        // Try structured replay (JSON) first, fall back to JSONL event log.
        if replay_path.ends_with(".json") {
            match std::fs::read_to_string(path) {
                Ok(contents) => {
                    match serde_json::from_str::<replay::recorder::GameReplay>(&contents) {
                        Ok(replay) => {
                            println!("Replaying game: {} players", replay.num_players);
                            println!("Players: {}\n", replay.player_names.join(", "));
                            for (i, frame) in replay.frames.iter().enumerate() {
                                let vp: String = frame.victory_points.iter()
                                    .enumerate()
                                    .map(|(p, v)| format!("P{}:{}", p, v))
                                    .collect::<Vec<_>>()
                                    .join(" ");
                                println!("{:>4}. [T{:>3}] {} [{}]", i + 1, frame.turn, frame.description, vp);
                            }
                            println!("\n{}", replay.stats());
                        }
                        Err(e) => eprintln!("Failed to parse replay: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read replay file: {}", e),
            }
        } else {
            match replay::event::GameLog::read_jsonl(path) {
                Ok(log) => {
                    println!("Replaying game from: {}", replay_path);
                    println!("Total events: {}\n", log.events().len());
                    for (i, event) in log.events().iter().enumerate() {
                        println!("{:>4}. {:?}", i + 1, event);
                    }
                }
                Err(e) => eprintln!("Failed to read replay file: {}", e),
            }
        }
        return;
    }

    // Handle resume mode.
    if let Some(ref save_path) = cli.resume {
        let path = std::path::Path::new(save_path);
        match replay::save::SaveGame::load_from_file(path) {
            Ok(save) => {
                println!("Resuming game from: {}", save_path);
                println!("Players: {}", save.player_names.join(", "));
                println!("Turn: {}, Events: {}\n", save.state.turn_number, save.events.len());

                // Recreate players — use random players for demo/saved games.
                let players: Vec<Box<dyn player::Player>> = save.player_names.iter()
                    .enumerate()
                    .map(|(i, name)| {
                        let model = save.player_models.get(i)
                            .filter(|m| !m.is_empty())
                            .cloned();
                        if let Some(model_id) = model {
                            Box::new(player::llm::LlmPlayer::new(
                                name.clone(),
                                model_id,
                                player::personality::Personality::default(),
                            )) as Box<dyn player::Player>
                        } else {
                            Box::new(player::random::RandomPlayer::new(name.clone()))
                                as Box<dyn player::Player>
                        }
                    })
                    .collect();

                let log = save.recent_log();
                let mut orchestrator = game::orchestrator::GameOrchestrator::new(
                    save.state, players,
                );
                orchestrator.log = log;
                orchestrator.max_turns = cli.max_turns;

                match orchestrator.run().await {
                    Ok(winner) => {
                        println!(
                            "\nPlayer {} ({}) wins!",
                            winner, orchestrator.player_names[winner]
                        );
                        // Save log and replay.
                        let _ = orchestrator.log.write_jsonl(std::path::Path::new("game_log.jsonl"));
                        if let Ok(json) = serde_json::to_string_pretty(&orchestrator.replay) {
                            let _ = std::fs::write("game_replay.json", json);
                        }
                        println!("\n{}", orchestrator.replay.stats());
                    }
                    Err(e) => {
                        eprintln!("Game ended: {}", e);
                        // Save progress even on error.
                        let save = replay::save::SaveGame::new(
                            orchestrator.state.clone(),
                            &orchestrator.log,
                            orchestrator.player_names.clone(),
                            save.player_models.clone(),
                        );
                        if let Err(e) = save.save_to_file(std::path::Path::new("game_save.json")) {
                            eprintln!("Warning: failed to save game: {}", e);
                        } else {
                            println!("Game progress saved to game_save.json");
                        }
                    }
                }
            }
            Err(e) => eprintln!("Failed to load save file: {}", e),
        }
        return;
    }

    assert!(
        (2..=4).contains(&cli.players),
        "Player count must be 2-4, got {}",
        cli.players
    );

    // Create a board (seeded for reproducibility if requested).
    let board = if let Some(seed) = cli.seed {
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        game::board::Board::generate(&mut rng)
    } else {
        let mut rng = rand::thread_rng();
        game::board::Board::generate(&mut rng)
    };

    // Create game state.
    let state = game::state::GameState::new(board.clone(), cli.players);

    // Create players.
    let names: Vec<String>;
    let players: Vec<Box<dyn player::Player>> = if cli.demo || cli.tui {
        let name_list = ["Alice", "Bob", "Charlie", "Diana"];
        names = (0..cli.players).map(|i| name_list[i].into()).collect();
        (0..cli.players)
            .map(|i| {
                Box::new(player::random::RandomPlayer::new(name_list[i].into()))
                    as Box<dyn player::Player>
            })
            .collect()
    } else {
        // Parse per-player models if provided.
        let per_models: Vec<String> = if let Some(ref models_str) = cli.models {
            models_str.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            vec![cli.model.clone(); cli.players]
        };

        // Load custom personality or use built-in defaults.
        let custom_personality = cli.personality.as_ref().map(|path| {
            player::personality::Personality::from_toml_file(std::path::Path::new(path))
                .unwrap_or_else(|e| {
                    eprintln!("Warning: {}, using default personality", e);
                    player::personality::Personality::default()
                })
        });

        let default_personalities = [
            player::personality::Personality::default_personality(),
            player::personality::Personality::aggressive(),
            player::personality::Personality::grudge_holder(),
            player::personality::Personality::builder(),
        ];

        let name_list = ["Claude", "GPT", "Gemini", "Llama"];
        names = (0..cli.players).map(|i| name_list[i].into()).collect();
        (0..cli.players)
            .map(|i| {
                let model = per_models.get(i).cloned().unwrap_or_else(|| cli.model.clone());
                let personality = custom_personality.clone()
                    .unwrap_or_else(|| default_personalities[i].clone());
                Box::new(player::llm::LlmPlayer::new(
                    name_list[i].into(),
                    model,
                    personality,
                )) as Box<dyn player::Player>
            })
            .collect()
    };

    if cli.tui {
        // TUI spectator mode.
        let (tx, rx) = mpsc::unbounded_channel();
        let player_names = names.clone();

        // Spawn the game engine in a background task.
        let game_handle = tokio::spawn(async move {
            let mut orchestrator = game::orchestrator::GameOrchestrator::new(state, players);
            orchestrator.max_turns = cli.max_turns;
            orchestrator.ui_tx = Some(tx);

            // Add a small delay between actions so the TUI can keep up.
            let result = orchestrator.run().await;

            // Save game log and replay.
            let log_path = std::path::Path::new("game_log.jsonl");
            if let Err(e) = orchestrator.log.write_jsonl(log_path) {
                eprintln!("Warning: failed to write game log: {}", e);
            }
            let replay_path = std::path::Path::new("game_replay.json");
            if let Ok(json) = serde_json::to_string_pretty(&orchestrator.replay) {
                if let Err(e) = std::fs::write(replay_path, json) {
                    eprintln!("Warning: failed to write replay: {}", e);
                }
            }

            result
        });

        // Run the TUI on the main task.
        if let Err(e) = ui::run_tui(rx, player_names).await {
            eprintln!("TUI error: {}", e);
        }

        // Wait for the game to finish (or it may have already).
        let _ = game_handle.await;
    } else {
        // Classic text mode.
        println!("Catan - Terminal Edition with LLM Players");
        println!("==========================================\n");
        println!("{}\n", player::prompt::ascii_board(&board));

        if cli.demo {
            println!("Starting demo game with random AI players...\n");
        } else {
            println!("Starting game with LLM players (model: {})...\n", cli.model);
        }

        let mut orchestrator = game::orchestrator::GameOrchestrator::new(state, players);
        orchestrator.max_turns = cli.max_turns;

        match orchestrator.run().await {
            Ok(_winner) => {
                println!(
                    "\nFinal scores: {}",
                    (0..cli.players)
                        .map(|p| format!(
                            "Player {} ({}): {} VP",
                            p,
                            orchestrator.player_names[p],
                            orchestrator.state.victory_points(p)
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                let log_path = std::path::Path::new("game_log.jsonl");
                if let Err(e) = orchestrator.log.write_jsonl(log_path) {
                    eprintln!("Warning: failed to write game log: {}", e);
                } else {
                    println!("Game log saved to {}", log_path.display());
                }

                // Save structured replay.
                let replay_path = std::path::Path::new("game_replay.json");
                if let Ok(json) = serde_json::to_string_pretty(&orchestrator.replay) {
                    if let Err(e) = std::fs::write(replay_path, json) {
                        eprintln!("Warning: failed to write replay: {}", e);
                    } else {
                        println!("Replay saved to {}", replay_path.display());
                        println!("\n{}", orchestrator.replay.stats());
                    }
                }
            }
            Err(e) => {
                eprintln!("Game ended: {}", e);
                // Save progress on error so the game can be resumed.
                let model_ids: Vec<String> = if let Some(ref models_str) = cli.models {
                    models_str.split(',').map(|s| s.trim().to_string()).collect()
                } else if cli.demo {
                    vec!["".into(); cli.players]
                } else {
                    vec![cli.model.clone(); cli.players]
                };
                let save = replay::save::SaveGame::new(
                    orchestrator.state.clone(),
                    &orchestrator.log,
                    orchestrator.player_names.clone(),
                    model_ids,
                );
                if let Err(e) = save.save_to_file(std::path::Path::new("game_save.json")) {
                    eprintln!("Warning: failed to save game: {}", e);
                } else {
                    println!("Game progress saved to game_save.json — resume with --resume game_save.json");
                }
            }
        }
    }
}
