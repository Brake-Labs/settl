pub mod board_view;
pub mod chat_panel;
pub mod game_log;
pub mod layout;
pub mod resource_bar;

use std::io;
use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use tokio::sync::mpsc;

use crate::game::state::GameState;
use crate::replay::event::GameEvent;

/// Player colors shared across all UI panels.
pub const PLAYER_COLORS: [Color; 4] = [
    Color::Red,
    Color::Blue,
    Color::Green,
    Color::Magenta,
];

/// Events sent from the game orchestrator to the TUI.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum UiEvent {
    /// A game state update with the latest event and a human-readable message.
    StateUpdate {
        state: Arc<GameState>,
        event: Option<GameEvent>,
        message: String,
    },
    /// AI reasoning trace from an LLM or random player.
    AiReasoning {
        player_id: usize,
        player_name: String,
        reasoning: String,
    },
    /// The game has ended.
    GameOver {
        winner: usize,
        message: String,
    },
}

/// The TUI application state.
pub struct App {
    /// Current game state snapshot (shared via Arc to avoid full clones).
    pub state: Option<Arc<GameState>>,
    /// Log of messages to display.
    pub messages: Vec<String>,
    /// AI reasoning / chat messages.
    pub chat_messages: Vec<chat_panel::ChatMessage>,
    /// Player names.
    pub player_names: Vec<String>,
    /// Whether the game is over.
    pub game_over: bool,
    /// Scroll offset for the log panel.
    pub log_scroll: u16,
    /// Scroll offset for the chat panel.
    pub chat_scroll: u16,
    /// Speed multiplier (delay between UI updates in ms).
    pub speed_ms: u64,
    /// Whether the game is paused.
    pub paused: bool,
}

impl App {
    pub fn new(player_names: Vec<String>) -> Self {
        Self {
            state: None,
            messages: vec!["Catan TUI — Spectator Mode".into(), "Press 'q' to quit, Space to pause, +/- to adjust speed".into()],
            chat_messages: Vec::new(),
            player_names,
            game_over: false,
            log_scroll: 0,
            chat_scroll: 0,
            speed_ms: 100,
            paused: false,
        }
    }

    /// Apply a UI event from the game engine.
    pub fn handle_game_event(&mut self, ui_event: UiEvent) {
        match ui_event {
            UiEvent::StateUpdate { state, event: _, message } => {
                self.state = Some(state);
                if !message.is_empty() {
                    self.messages.push(message);
                    // Auto-scroll to bottom.
                    let total = self.messages.len() as u16;
                    self.log_scroll = total.saturating_sub(1);
                }
            }
            UiEvent::AiReasoning { player_id, player_name, reasoning } => {
                self.chat_messages.push(chat_panel::ChatMessage {
                    player: player_name,
                    player_id,
                    text: reasoning,
                });
                // Auto-scroll chat to bottom.
                let total = self.chat_messages.len() as u16;
                self.chat_scroll = total.saturating_sub(1);
            }
            UiEvent::GameOver { winner, message } => {
                self.messages.push(format!("GAME OVER: Player {} ({}) wins!", winner, self.player_names.get(winner).unwrap_or(&"?".into())));
                self.messages.push(message);
                self.game_over = true;
                let total = self.messages.len() as u16;
                self.log_scroll = total.saturating_sub(1);
            }
        }
    }
}

/// Run the TUI event loop. Receives game events via the channel.
pub async fn run_tui(
    mut rx: mpsc::UnboundedReceiver<UiEvent>,
    player_names: Vec<String>,
) -> io::Result<()> {
    // Setup terminal.
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(player_names);

    loop {
        // Draw UI.
        terminal.draw(|f| layout::draw(f, &app))?;

        // Poll for crossterm events (keyboard) with a short timeout.
        let timeout = Duration::from_millis(if app.paused { 50 } else { app.speed_ms.min(50) });
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app.speed_ms = app.speed_ms.saturating_sub(25).max(25);
                        }
                        KeyCode::Char('-') => {
                            app.speed_ms = (app.speed_ms + 25).min(500);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.log_scroll = app.log_scroll.saturating_sub(1);
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.log_scroll = app.log_scroll.saturating_add(1);
                        }
                        _ => {}
                    }
                }
            }
        }

        // Drain game events from the channel.
        if !app.paused {
            while let Ok(ui_event) = rx.try_recv() {
                app.handle_game_event(ui_event);
            }
        }

        if app.game_over {
            // Keep running so user can view final state, but stop receiving.
            continue;
        }
    }

    // Restore terminal.
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
