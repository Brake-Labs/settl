//! Scrollable game log panel.

use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Wrap};

/// Get the display color for a game log message.
pub fn message_color(msg: &str) -> Color {
    if msg.starts_with("GAME OVER") || msg.contains("wins") {
        Color::Yellow
    } else if msg.contains("Trade") || msg.contains("trade") {
        Color::Cyan
    } else if msg.contains("Rolled") {
        Color::White
    } else if msg.contains("Settlement") || msg.contains("City") || msg.contains("Road") {
        Color::Green
    } else if msg.contains("Robber") || msg.contains("Stole") {
        Color::Red
    } else if msg.contains("Setup") {
        Color::DarkGray
    } else {
        Color::Gray
    }
}

/// Build game log lines (shared helper).
fn build_log_lines(messages: &[String]) -> Vec<Line<'static>> {
    messages
        .iter()
        .map(|msg| {
            let mut style = Style::default().fg(message_color(msg));
            if msg.starts_with("GAME OVER") {
                style = style.bold();
            }
            Line::from(Span::styled(msg.clone(), style))
        })
        .collect()
}

/// Estimate visual line count after wrapping.
fn estimate_visual_lines(lines: &[Line], width: usize) -> usize {
    let mut total: usize = 0;
    for line in lines {
        let char_count: usize = line.spans.iter().map(|s| s.content.chars().count()).sum();
        total += if char_count == 0 {
            1
        } else {
            char_count.div_ceil(width)
        };
    }
    total
}

/// Render the game log without a border (for use inside a shared panel).
pub fn render_log_inner(messages: &[String], scroll: u16, area: Rect, buf: &mut Buffer) {
    let lines = build_log_lines(messages);
    let inner_width = area.width.max(1) as usize;
    let total_visual_lines = estimate_visual_lines(&lines, inner_width);
    let visible_height = area.height as usize;
    let max_scroll = total_visual_lines.saturating_sub(visible_height) as u16;
    let effective_scroll = scroll.min(max_scroll);

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((effective_scroll, 0));

    paragraph.render(area, buf);

    // Scroll indicators: show ^ at top-right when scrolled down, v at bottom-right
    // when more content exists below.
    if area.height >= 2 && !messages.is_empty() {
        let indicator_style = Style::default().fg(Color::DarkGray);
        if effective_scroll > 0 {
            buf.set_string(
                area.right().saturating_sub(2),
                area.top(),
                "^",
                indicator_style,
            );
        }
        if effective_scroll < max_scroll {
            buf.set_string(
                area.right().saturating_sub(2),
                area.bottom().saturating_sub(1),
                "v",
                indicator_style,
            );
        }
    }
}
