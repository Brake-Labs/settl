//! Integration tests: generate board images to disk for visual inspection.
//!
//! Run with: cargo test --test board_image_test -- --nocapture
//! Then open target/test_board*.png to verify.

use settl::game::board::{Board, HexCoord, VertexCoord, VertexDirection};
use settl::game::rules;
use settl::game::state::{Building, GameState};
use settl::ui::board_image::BoardImageRenderer;

/// Helper: create a game state with some buildings placed.
fn test_state_with_buildings() -> GameState {
    let board = Board::default_board();
    let mut state = GameState::new(board, 4);

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

    state
}

#[test]
fn generate_test_board_image() {
    let state = test_state_with_buildings();
    let renderer = BoardImageRenderer::new_test();
    let img = renderer.generate_test_image(&state);

    std::fs::create_dir_all("target").ok();
    img.save("target/test_board.png").expect("save PNG");
    eprintln!("Board image saved to target/test_board.png");
}

#[test]
fn generate_board_with_settlement_cursors() {
    let board = Board::default_board();
    let state = GameState::new(board, 4);

    // Get all legal settlement positions for player 0 (setup phase -- all vertices).
    let legal = rules::legal_setup_vertices(&state);

    let renderer = BoardImageRenderer::new_test();
    let img = renderer.generate_test_image_with_cursors(&state, &legal, 0);

    std::fs::create_dir_all("target").ok();
    img.save("target/test_board_cursors.png").expect("save PNG");
    eprintln!(
        "Board with {} cursor positions saved to target/test_board_cursors.png",
        legal.len()
    );
}

#[test]
fn generate_board_with_cell_grid() {
    let state = test_state_with_buildings();
    let renderer = BoardImageRenderer::new_test();

    // Simulate a typical terminal: font 16x34, board area ~107x34 cells.
    let img = renderer.generate_test_image_with_grid(&state, (16, 34), (107, 34));

    std::fs::create_dir_all("target").ok();
    img.save("target/test_board_grid.png").expect("save PNG");
    eprintln!("Board with cell grid overlay saved to target/test_board_grid.png");
}
