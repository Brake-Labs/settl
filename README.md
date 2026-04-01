# settl

A terminal-based Settlers of Catan game where LLMs play against each other (or you).
Watch Claude, GPT, and Gemini negotiate trades, form grudges, and compete for longest road — all in your terminal.

```
        [Fo 6] [Pa 3] [Hi 8]
      [Fi 2] [Mo 5] [Fo 4] [Pa 9]
    [Hi10] [Fi 6] [De --] [Fo 3] [Mo11]
      [Pa 5] [Mo 8] [Hi 4] [Fi 9]
        [Fo10] [Pa11] [Mo 3]
```

## Features

- **Full base Catan rules** — settlements, cities, roads, robber, development cards, Longest Road, Largest Army, bank trading, player-to-player trading
- **Multi-provider LLM players** — Claude, GPT, Gemini, and any provider supported by the [genai](https://crates.io/crates/genai) crate
- **Visible AI reasoning** — every decision comes with a strategic explanation
- **Personality system** — aggressive traders, grudge holders, cautious builders, chaos agents
- **TUI spectator mode** — watch AI games with a live hex board, resource panels, and reasoning traces
- **Game replays** — JSONL event logs and structured JSON replays with full reasoning traces
- **Save/resume** — save a game in progress and resume later
- **Reproducible games** — seed the RNG for deterministic board generation

## Quick Start

```bash
# Run a demo game with random AI players (no API keys needed)
cargo run -- --demo

# Run with TUI spectator mode
cargo run -- --demo --tui

# Run with LLM players (requires API key)
ANTHROPIC_API_KEY=sk-... cargo run -- --model claude-sonnet-4-6

# Different models per player
ANTHROPIC_API_KEY=sk-... OPENAI_API_KEY=sk-... cargo run -- \
  --models "claude-sonnet-4-6,gpt-4o-mini,claude-haiku-4-5-20251001"

# Reproducible board with a seed
cargo run -- --demo --seed 42
```

## CLI Options

| Flag | Description | Default |
|------|-------------|---------|
| `--demo` | Random AI players, no API keys needed | off |
| `--tui` | TUI spectator mode with live board | off |
| `-p, --players N` | Number of players (2-4) | 4 |
| `-m, --model MODEL` | Default LLM model for all players | claude-sonnet-4-6 |
| `--models M1,M2,...` | Per-player model assignment | — |
| `--personality FILE` | TOML personality file | built-in |
| `--max-turns N` | Turn limit before declaring stuck | 500 |
| `--seed N` | RNG seed for reproducible boards | random |
| `--replay FILE` | Replay a saved game (.json or .jsonl) | — |
| `--resume FILE` | Resume a saved game | — |

## TUI Controls

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `Space` | Pause/unpause |
| `+` / `-` | Adjust speed |
| `j` / `k` | Scroll log |

## Personalities

Create a TOML file to define custom AI personalities:

```toml
[personality]
name = "The Grudge Holder"
style = "remembers every slight, refuses trades with players who wronged them"
aggression = 0.3
cooperation = 0.2
catchphrases = ["I haven't forgotten turn 7.", "You'll have to do better than that."]
```

Built-in personalities: Default Strategist, Aggressive Trader, Grudge Holder, Cautious Builder, Chaos Agent.

## Replays

Games automatically save two replay formats:

- `game_log.jsonl` — one JSON event per line, lightweight
- `game_replay.json` — structured replay with VP tracking and event descriptions

View a replay:

```bash
# Structured replay with VP tracking
cargo run -- --replay game_replay.json

# Raw event log
cargo run -- --replay game_log.jsonl
```

## Save/Resume

If a game gets stuck (hits max turns), progress is automatically saved to `game_save.json`.
Resume with:

```bash
cargo run -- --resume game_save.json
```

## Architecture

```
src/
├── main.rs                    # CLI entry, game setup
├── game/
│   ├── board.rs               # Hex grid (axial coords), terrain, ports
│   ├── state.rs               # GameState, PlayerState, buildings, roads
│   ├── rules.rs               # Legal moves, placement, dev cards
│   ├── actions.rs             # Action/DevCard/TradeOffer types
│   ├── dice.rs                # Dice rolls, resource distribution
│   └── orchestrator.rs        # Game loop, player interaction
├── player/
│   ├── mod.rs                 # Player trait (async)
│   ├── llm.rs                 # LLM player (genai + tool use)
│   ├── random.rs              # Random AI player (for testing)
│   ├── human.rs               # Human player (TUI input)
│   ├── personality.rs         # Personality system (TOML)
│   └── prompt.rs              # Board/state → LLM prompt serialization
├── trading/
│   ├── negotiation.rs         # Trade protocol, validation, execution
│   └── offers.rs              # Offer validation, resource checks
├── replay/
│   ├── event.rs               # GameEvent enum, JSONL log
│   ├── recorder.rs            # GameReplay with state snapshots
│   └── save.rs                # Save/resume game state
└── ui/
    ├── mod.rs                 # TUI app state, event loop
    ├── board_view.rs          # Hex board rendering (ratatui)
    ├── resource_bar.rs        # Player resource/VP panel
    ├── chat_panel.rs          # AI reasoning display
    ├── game_log.rs            # Scrollable event log
    └── layout.rs              # TUI layout composition
```

## API Keys

Set environment variables for LLM providers:

```bash
export ANTHROPIC_API_KEY=sk-ant-...   # Claude
export OPENAI_API_KEY=sk-...          # GPT
export GOOGLE_API_KEY=...             # Gemini
```

## Development

```bash
# Run all tests
cargo test

# Run a quick demo game
cargo run -- --demo --max-turns 100

# Run with verbose output
RUST_LOG=debug cargo run -- --demo
```

## License

MIT
