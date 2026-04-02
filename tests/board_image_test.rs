//! Integration test: generate a board image to disk for visual inspection.

use std::collections::HashMap;

use settl::game::board::{Board, HexCoord, VertexCoord, VertexDirection};
use settl::game::state::{Building, GameState};
use settl::ui::board_image::BoardImageRenderer;

#[test]
fn generate_test_board_image() {
    let board = Board::default_board();
    let mut state = GameState::new(board, 4);

    // Add some buildings and roads for visual testing.
    state.buildings.insert(
        VertexCoord::new(HexCoord::new(0, 0), VertexDirection::North),
        Building::Settlement(0),
    );
    state.buildings.insert(
        VertexCoord::new(HexCoord::new(1, 0), VertexDirection::South),
        Building::City(1),
    );
    state.buildings.insert(
        VertexCoord::new(HexCoord::new(-1, 1), VertexDirection::North),
        Building::Settlement(2),
    );
    state.buildings.insert(
        VertexCoord::new(HexCoord::new(0, -1), VertexDirection::South),
        Building::Settlement(3),
    );

    let renderer = BoardImageRenderer::new_test();
    let img = renderer.generate_test_image(&state);

    // Save to target/ so it doesn't clutter the project root.
    std::fs::create_dir_all("target").ok();
    img.save("target/test_board.png").expect("save PNG");
    eprintln!("Board image saved to target/test_board.png");
}
