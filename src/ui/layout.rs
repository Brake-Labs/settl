//! TUI layout — splits the terminal into board, players, and log panels.

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use super::App;
use super::board_view;
use super::chat_panel;
use super::game_log;
use super::resource_bar;

/// Draw the full TUI layout.
///
/// ```text
/// +------------------------------------------+------------------+
/// |                                          |    PLAYERS       |
/// |           BOARD VIEW                     |    P0: W:2 B:3   |
/// |           (hex grid)                     |    P1: W:1 B:0   |
/// |                                          |    P2: W:0 B:1   |
/// |                                          |    P3: W:3 B:2   |
/// +------------------------------------------+------------------+
/// |                          |                                   |
/// |    GAME LOG (scrollable) |     AI REASONING (scrollable)     |
/// |                          |                                   |
/// +--------------------------------------------------------------+
/// |  Status bar: speed, paused, controls                         |
/// +--------------------------------------------------------------+
/// ```
pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Main vertical split: top (board + players) | bottom (log + chat) | status bar.
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(12),       // Board + players
            Constraint::Percentage(40), // Game log + AI chat
            Constraint::Length(1),      // Status bar
        ])
        .split(size);

    // Top horizontal split: board | players.
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(40),       // Board
            Constraint::Length(24),     // Players panel
        ])
        .split(main_chunks[0]);

    // Render board.
    if let Some(state) = &app.state {
        board_view::render_board(state, top_chunks[0], f.buffer_mut());
        resource_bar::render_players(state, &app.player_names, top_chunks[1], f.buffer_mut());
    } else {
        let waiting = Paragraph::new("Waiting for game to start...")
            .block(
                Block::default()
                    .title(" Board ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center);
        f.render_widget(waiting, top_chunks[0]);

        let no_players = Paragraph::new("")
            .block(
                Block::default()
                    .title(" Players ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            );
        f.render_widget(no_players, top_chunks[1]);
    }

    // Bottom horizontal split: game log | AI reasoning.
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Game log
            Constraint::Percentage(50), // AI reasoning
        ])
        .split(main_chunks[1]);

    // Render game log and AI chat.
    game_log::render_log(&app.messages, app.log_scroll, bottom_chunks[0], f.buffer_mut());
    chat_panel::render_chat(&app.chat_messages, app.chat_scroll, bottom_chunks[1], f.buffer_mut());

    // Status bar.
    let pause_indicator = if app.paused { " PAUSED " } else { "" };
    let status = Line::from(vec![
        Span::styled(
            format!(" Speed: {}ms ", app.speed_ms),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(
            pause_indicator,
            Style::default().fg(Color::Black).bg(Color::Yellow).bold(),
        ),
        Span::styled(
            " | q:quit  Space:pause  +/-:speed  j/k:scroll ",
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            if app.game_over { " GAME OVER " } else { "" },
            Style::default().fg(Color::Black).bg(Color::Green).bold(),
        ),
    ]);
    let status_paragraph = Paragraph::new(status);
    f.render_widget(status_paragraph, main_chunks[2]);
}
