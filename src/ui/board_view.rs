//! Renders the Catan hex board with shaped hex cells, buildings, roads, and cursor.

use std::collections::HashMap;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

use crate::game::board::{self, EdgeCoord, EdgeDirection, HexCoord, Terrain, VertexCoord};
use crate::game::state::{Building, GameState};

use super::{CursorKind, InputMode, PLAYER_COLORS};

// ── Layout constants ──────────────────────────────────────────────────

/// Horizontal distance between hex centers in the same row.
const HEX_COL_Q: i16 = 12;
/// Horizontal offset per r-row (half of HEX_COL_Q).
const HEX_COL_R: i16 = 6;
/// Vertical distance between hex row centers.
const HEX_ROW: i16 = 5;
/// Vertical offset from center to North/South vertex.
const VERT_OFF: i16 = 3;

// ── Terrain colors ───────────────────────────────────────────────────

fn terrain_color(t: Terrain) -> Color {
    match t {
        Terrain::Forest => Color::Rgb(34, 120, 34),
        Terrain::Hills => Color::Rgb(178, 102, 51),
        Terrain::Pasture => Color::Rgb(80, 180, 60),
        Terrain::Fields => Color::Rgb(200, 170, 50),
        Terrain::Mountains => Color::Rgb(140, 140, 150),
        Terrain::Desert => Color::Rgb(180, 160, 120),
    }
}

fn terrain_fg(_t: Terrain) -> Color {
    Color::White
}

// ── Probability dots ────────────────────────────────────────────────

fn probability_dots(number: u8) -> &'static str {
    match number {
        2 | 12 => "\u{00b7}",
        3 | 11 => "\u{00b7}\u{00b7}",
        4 | 10 => "\u{00b7}\u{00b7}\u{00b7}",
        5 | 9 => "\u{00b7}\u{00b7}\u{00b7}\u{00b7}",
        6 | 8 => "\u{00b7}\u{00b7}\u{00b7}\u{00b7}\u{00b7}",
        _ => "",
    }
}

// ── HexGrid ─────────────────────────────────────────────────────────

/// Precomputed screen positions for all hex elements.
pub struct HexGrid {
    /// Hex center positions (col, row) in board-local coordinates.
    hex_centers: HashMap<HexCoord, (i16, i16)>,
    /// Vertex screen positions.
    vertex_pos: HashMap<VertexCoord, (i16, i16)>,
    /// Edge midpoint screen positions (for cursor targeting).
    edge_pos: HashMap<EdgeCoord, (i16, i16)>,
    /// Board-local bounding box.
    pub width: u16,
    pub height: u16,
}

impl Default for HexGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl HexGrid {
    pub fn new() -> Self {
        let coords = board::board_hex_coords();
        let mut hex_centers = HashMap::new();
        let mut vertex_pos: HashMap<VertexCoord, (i16, i16)> = HashMap::new();
        let mut edge_pos: HashMap<EdgeCoord, (i16, i16)> = HashMap::new();

        // Compute hex centers. The base offset ensures all coords are positive.
        // Min q=-2, min r=-2. With the formula col = q*HEX_COL_Q + r*HEX_COL_R,
        // min col = -2*12 + (-2)*6 = -36. So base_col = 38 to add margin.
        // Min row = -2*5 = -10. N vertex extends VERT_OFF(3) above. So base_row = 14.
        let base_col: i16 = 38;
        let base_row: i16 = 14;

        for &c in &coords {
            let cx = c.q as i16 * HEX_COL_Q + c.r as i16 * HEX_COL_R + base_col;
            let cy = c.r as i16 * HEX_ROW + base_row;
            hex_centers.insert(c, (cx, cy));
        }

        // Compute ALL 6 vertex positions per hex. This is critical for border
        // hexes whose side vertices (NE/SE/SW/NW) reference off-board hex
        // coordinates that would otherwise never be added.
        //
        // Vertex offsets from hex center (cx, cy):
        //   v0 N:  ( 0,           -VERT_OFF)
        //   v1 NE: (+HEX_COL_R,  -side_dy)     where side_dy = HEX_ROW - VERT_OFF
        //   v2 SE: (+HEX_COL_R,  +side_dy)
        //   v3 S:  ( 0,           +VERT_OFF)
        //   v4 SW: (-HEX_COL_R,  +side_dy)
        //   v5 NW: (-HEX_COL_R,  -side_dy)
        let side_dy = HEX_ROW - VERT_OFF;
        for &c in &coords {
            let (cx, cy) = hex_centers[&c];
            let verts = board::hex_vertices(c);
            vertex_pos.entry(verts[0]).or_insert((cx, cy - VERT_OFF));
            vertex_pos
                .entry(verts[1])
                .or_insert((cx + HEX_COL_R, cy - side_dy));
            vertex_pos
                .entry(verts[2])
                .or_insert((cx + HEX_COL_R, cy + side_dy));
            vertex_pos.entry(verts[3]).or_insert((cx, cy + VERT_OFF));
            vertex_pos
                .entry(verts[4])
                .or_insert((cx - HEX_COL_R, cy + side_dy));
            vertex_pos
                .entry(verts[5])
                .or_insert((cx - HEX_COL_R, cy - side_dy));
        }

        // Compute ALL 6 edge midpoint positions per hex. Same border issue:
        // the SW/W/NW edges canonicalize to off-board hex coordinates.
        //
        // Edge midpoint offsets from hex center (cx, cy):
        //   e0 NE: (+half_r, -edge_dy)    where half_r = HEX_COL_R/2, edge_dy = VERT_OFF-1
        //   e1 E:  (+HEX_COL_R, 0)
        //   e2 SE: (+half_r, +edge_dy)
        //   e3 SW: (-half_r, +edge_dy)
        //   e4 W:  (-HEX_COL_R, 0)
        //   e5 NW: (-half_r, -edge_dy)
        let half_r = HEX_COL_R / 2;
        let edge_dy = VERT_OFF - 1;
        for &c in &coords {
            let (cx, cy) = hex_centers[&c];
            let edges = board::hex_edges(c);
            edge_pos
                .entry(edges[0])
                .or_insert((cx + half_r, cy - edge_dy));
            edge_pos.entry(edges[1]).or_insert((cx + HEX_COL_R, cy));
            edge_pos
                .entry(edges[2])
                .or_insert((cx + half_r, cy + edge_dy));
            edge_pos
                .entry(edges[3])
                .or_insert((cx - half_r, cy + edge_dy));
            edge_pos.entry(edges[4]).or_insert((cx - HEX_COL_R, cy));
            edge_pos
                .entry(edges[5])
                .or_insert((cx - half_r, cy - edge_dy));
        }

        // Bounding box: hex extends cx-6..cx+6 horizontally, cy-3..cy+3 vertically.
        let max_col = hex_centers.values().map(|(c, _)| c + 7).max().unwrap_or(0);
        let max_row = hex_centers
            .values()
            .map(|(_, r)| r + VERT_OFF + 1)
            .max()
            .unwrap_or(0);

        HexGrid {
            hex_centers,
            vertex_pos,
            edge_pos,
            width: (max_col + 2) as u16,
            height: (max_row + 2) as u16,
        }
    }

    /// Get screen position of a vertex (board-local).
    pub fn vertex_screen_pos(&self, v: &VertexCoord) -> Option<(u16, u16)> {
        self.vertex_pos
            .get(v)
            .map(|&(c, r)| (c.max(0) as u16, r.max(0) as u16))
    }

    /// Get screen position of an edge midpoint (board-local).
    pub fn edge_screen_pos(&self, e: &EdgeCoord) -> Option<(u16, u16)> {
        self.edge_pos
            .get(e)
            .map(|&(c, r)| (c.max(0) as u16, r.max(0) as u16))
    }

    /// Get screen position of a hex center (board-local).
    pub fn hex_center_pos(&self, h: &HexCoord) -> (u16, u16) {
        self.hex_centers
            .get(h)
            .map(|&(c, r)| (c.max(0) as u16, r.max(0) as u16))
            .unwrap_or((0, 0))
    }
}

// ── Rendering ───────────────────────────────────────────────────────

/// Render the hex board with terrain, buildings, roads, robber, and cursor.
pub fn render_board(
    state: &GameState,
    grid: &HexGrid,
    area: Rect,
    buf: &mut Buffer,
    input_mode: &InputMode,
) {
    // Draw border.
    let block = Block::default()
        .title(" Board ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    if inner.width < 10 || inner.height < 5 {
        return;
    }

    // Offset to center the board in the available area.
    let board_w = grid.width;
    let board_h = grid.height;
    let off_col = inner.x + inner.width.saturating_sub(board_w) / 2;
    let off_row = inner.y + inner.height.saturating_sub(board_h) / 2;

    // Layer 1: Draw hex cells (terrain + number).
    for hex in &state.board.hexes {
        if let Some(&(cx, cy)) = grid.hex_centers.get(&hex.coord) {
            let scr_col = off_col as i16 + cx;
            let scr_row = off_row as i16 + cy;
            draw_hex_cell(hex, state, scr_col, scr_row, inner, buf);
        }
    }

    // Layer 2: Draw roads.
    for (&edge, &player_id) in &state.roads {
        if let Some(&(ex, ey)) = grid.edge_pos.get(&edge) {
            let scr_col = off_col as i16 + ex;
            let scr_row = off_row as i16 + ey;
            let color = PLAYER_COLORS
                .get(player_id)
                .copied()
                .unwrap_or(Color::White);
            draw_road_segment(edge.dir, scr_col, scr_row, color, inner, buf);
        }
    }

    // Layer 3: Draw buildings.
    for (vertex, building) in &state.buildings {
        if let Some(&(vx, vy)) = grid.vertex_pos.get(vertex) {
            let scr_col = off_col as i16 + vx;
            let scr_row = off_row as i16 + vy;
            let (player_id, ch) = match building {
                Building::Settlement(p) => (*p, '\u{25b2}'), // ▲
                Building::City(p) => (*p, '\u{25a0}'),       // ■
            };
            let color = PLAYER_COLORS
                .get(player_id)
                .copied()
                .unwrap_or(Color::White);
            set_cell(
                scr_col,
                scr_row,
                ch,
                Style::default().fg(color).bold(),
                inner,
                buf,
            );
        }
    }

    // Layer 4: Draw ports.
    for port in &state.board.ports {
        for v in [&port.vertices.0, &port.vertices.1] {
            if let Some(&(vx, vy)) = grid.vertex_pos.get(v) {
                let scr_col = off_col as i16 + vx;
                let scr_row = off_row as i16 + vy;
                // Only draw port marker if no building is there.
                if !state.buildings.contains_key(v) {
                    set_cell(
                        scr_col,
                        scr_row,
                        '*',
                        Style::default().fg(Color::Yellow),
                        inner,
                        buf,
                    );
                }
            }
        }
    }

    // Layer 5: Draw cursor overlay (legal positions + selected).
    draw_cursor_overlay(grid, off_col, off_row, inner, buf, input_mode);
}

/// Draw a single hex cell with outlines, terrain fill, and probability dots.
///
/// With HEX_COL_Q=12, HEX_ROW=5, VERT_OFF=3 the hex is a wide hexagon:
/// ```text
///          ·                cy-3: N vertex
///        ╱   ╲              cy-2: upper diags
///      ╱  Wo  6 ╲           cy-1: terrain + number
///     · ·······   ·         cy:   side verts + dots
///      ╲         ╱          cy+1: lower diags
///        ╲     ╱            cy+2: closing diags
///          ·                cy+3: S vertex
/// ```
fn draw_hex_cell(
    hex: &crate::game::board::Hex,
    state: &GameState,
    cx: i16,
    cy: i16,
    area: Rect,
    buf: &mut Buffer,
) {
    let bg = terrain_color(hex.terrain);
    let fg = terrain_fg(hex.terrain);
    let is_robber = state.robber_hex == hex.coord;
    let fill_bg = if is_robber { Color::Red } else { bg };
    let fill = Style::default().bg(fill_bg);
    let bg_fill = Style::default().bg(bg);

    let edge_style = Style::default().fg(Color::DarkGray);

    // Row cy-3: N vertex (left empty for building/port/cursor layers)

    // Row cy-2: upper diagonals with interior fill
    set_cell(cx - 3, cy - 2, '\u{2571}', edge_style, area, buf); // ╱
    for dx in -2..=2i16 {
        set_cell(cx + dx, cy - 2, ' ', fill, area, buf);
    }
    set_cell(cx + 3, cy - 2, '\u{2572}', edge_style, area, buf); // ╲

    // Row cy-1: wider diagonals + terrain label + number
    set_cell(cx - 5, cy - 1, '\u{2571}', edge_style, area, buf); // ╱
    for dx in -4..=4i16 {
        set_cell(cx + dx, cy - 1, ' ', fill, area, buf);
    }
    set_cell(cx + 5, cy - 1, '\u{2572}', edge_style, area, buf); // ╲

    // Terrain abbreviation + number
    let abbr = hex.terrain.abbr();
    let text_style = if is_robber {
        Style::default().fg(Color::White).bg(Color::Red).bold()
    } else {
        Style::default().fg(fg).bg(fill_bg)
    };

    if is_robber {
        set_cell(cx - 3, cy - 1, 'R', text_style, area, buf);
        for (i, ch) in abbr.chars().enumerate() {
            set_cell(cx - 1 + i as i16, cy - 1, ch, text_style, area, buf);
        }
    } else {
        for (i, ch) in abbr.chars().enumerate() {
            set_cell(cx - 2 + i as i16, cy - 1, ch, text_style, area, buf);
        }
    }

    if let Some(n) = hex.number_token {
        let num_str = format!("{:>2}", n);
        let is_hot = n == 6 || n == 8;
        let num_style = if is_hot {
            Style::default().fg(Color::Red).bg(fill_bg).bold()
        } else {
            Style::default().fg(fg).bg(fill_bg)
        };
        for (i, ch) in num_str.chars().enumerate() {
            set_cell(cx + 1 + i as i16, cy - 1, ch, num_style, area, buf);
        }
    } else if !is_robber {
        set_cell(cx + 1, cy - 1, '-', text_style, area, buf);
        set_cell(cx + 2, cy - 1, '-', text_style, area, buf);
    }

    // Row cy: widest row -- side vertices at cx±6, probability dots centered
    for dx in -5..=5i16 {
        set_cell(cx + dx, cy, ' ', bg_fill, area, buf);
    }

    if let Some(n) = hex.number_token {
        let dots = probability_dots(n);
        let is_hot = n == 6 || n == 8;
        let dots_style = if is_hot {
            Style::default().fg(Color::Red).bg(bg).bold()
        } else {
            Style::default().fg(Color::DarkGray).bg(bg)
        };
        let dot_start = cx - (dots.chars().count() as i16) / 2;
        for (i, ch) in dots.chars().enumerate() {
            set_cell(dot_start + i as i16, cy, ch, dots_style, area, buf);
        }
    }

    // Row cy+1: lower diagonals
    set_cell(cx - 5, cy + 1, '\u{2572}', edge_style, area, buf); // ╲
    for dx in -4..=4i16 {
        set_cell(cx + dx, cy + 1, ' ', bg_fill, area, buf);
    }
    set_cell(cx + 5, cy + 1, '\u{2571}', edge_style, area, buf); // ╱

    // Row cy+2: closing diagonals
    set_cell(cx - 3, cy + 2, '\u{2572}', edge_style, area, buf); // ╲
    for dx in -2..=2i16 {
        set_cell(cx + dx, cy + 2, ' ', bg_fill, area, buf);
    }
    set_cell(cx + 3, cy + 2, '\u{2571}', edge_style, area, buf); // ╱

    // Row cy+3: S vertex (left empty for building/port/cursor layers)
}

/// Draw a road segment at an edge midpoint.
fn draw_road_segment(
    dir: EdgeDirection,
    mx: i16,
    my: i16,
    color: Color,
    area: Rect,
    buf: &mut Buffer,
) {
    let style = Style::default().fg(color).bold();
    match dir {
        EdgeDirection::NorthEast | EdgeDirection::SouthEast => {
            // Horizontal-ish segment
            set_cell(mx - 1, my, '\u{2550}', style, area, buf); // ═
            set_cell(mx, my, '\u{2550}', style, area, buf);
            set_cell(mx + 1, my, '\u{2550}', style, area, buf);
        }
        EdgeDirection::East => {
            // Vertical segment
            set_cell(mx, my - 1, '\u{2551}', style, area, buf); // ║
            set_cell(mx, my, '\u{2551}', style, area, buf);
            set_cell(mx, my + 1, '\u{2551}', style, area, buf);
        }
    }
}

/// Draw cursor overlay highlighting legal positions and selected position.
fn draw_cursor_overlay(
    grid: &HexGrid,
    off_col: u16,
    off_row: u16,
    area: Rect,
    buf: &mut Buffer,
    input_mode: &InputMode,
) {
    if let InputMode::BoardCursor {
        kind,
        legal_vertices,
        legal_edges,
        legal_hexes,
        positions,
        selected,
    } = input_mode
    {
        let legal_style = Style::default().fg(Color::Yellow).bold();
        let cursor_style = Style::default().fg(Color::Black).bg(Color::Yellow).bold();

        match kind {
            CursorKind::Settlement => {
                for (i, v) in legal_vertices.iter().enumerate() {
                    if let Some(&(vx, vy)) = grid.vertex_pos.get(v) {
                        let sx = off_col as i16 + vx;
                        let sy = off_row as i16 + vy;
                        let style = if i == *selected {
                            cursor_style
                        } else {
                            legal_style
                        };
                        let ch = if i == *selected {
                            '\u{25c6}'
                        } else {
                            '\u{25c7}'
                        }; // ◆ / ◇
                        set_cell(sx, sy, ch, style, area, buf);
                    }
                }
            }
            CursorKind::Road => {
                for (i, e) in legal_edges.iter().enumerate() {
                    if let Some(&(ex, ey)) = grid.edge_pos.get(e) {
                        let sx = off_col as i16 + ex;
                        let sy = off_row as i16 + ey;
                        let style = if i == *selected {
                            cursor_style
                        } else {
                            legal_style
                        };
                        set_cell(sx - 1, sy, '=', style, area, buf);
                        set_cell(sx, sy, '=', style, area, buf);
                        set_cell(sx + 1, sy, '=', style, area, buf);
                    }
                }
            }
            CursorKind::Robber => {
                for (i, h) in legal_hexes.iter().enumerate() {
                    let (hx, hy) = grid.hex_center_pos(h);
                    let sx = off_col as i16 + hx as i16;
                    let sy = off_row as i16 + hy as i16;
                    let style = if i == *selected {
                        cursor_style
                    } else {
                        legal_style
                    };
                    set_cell(sx, sy, 'R', style, area, buf);
                }
            }
        }

        // Draw position description for the selected cursor position.
        let _ = (positions, selected); // used for screen positions in navigation
    }
}

/// Safe cell setter: only writes if within the given area.
fn set_cell(col: i16, row: i16, ch: char, style: Style, area: Rect, buf: &mut Buffer) {
    if col < 0 || row < 0 {
        return;
    }
    let col = col as u16;
    let row = row as u16;
    if col >= area.x && col < area.x + area.width && row >= area.y && row < area.y + area.height {
        if let Some(cell) = buf.cell_mut(ratatui::layout::Position::new(col, row)) {
            cell.set_char(ch);
            cell.set_style(style);
        }
    }
}
