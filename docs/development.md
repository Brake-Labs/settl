---
title: Development
description: Contributing, architecture, and testing
order: 5
---

# Development

settl is built with Rust, using ratatui for the TUI and tokio for async orchestration.

## Build Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build (binary: target/release/settl)
cargo test                     # All tests
cargo test game::rules         # Tests in a specific module
cargo test test_name           # Single test by name
cargo run                      # Launch TUI
```

## Architecture

The codebase has four main modules:

### `game/` -- Core Engine

Stateless rules engine plus stateful game orchestrator.

- **board.rs** -- Hex grid using axial coordinates `(q, r)`. Vertices and edges are `(HexCoord, Direction)` pairs. Canonical edge storage prevents duplicates.
- **state.rs** -- `GameState` with board, player resources, buildings, roads, robber, dev card deck, and special award tracking.
- **rules.rs** -- Pure validation. Given a `GameState`, returns legal moves and validates actions.
- **event.rs** -- `GameEvent` enum for all game actions, with human-readable formatting.
- **orchestrator.rs** -- Drives the game loop: setup snake-draft, main turn loop, player interaction via async trait.
- **dice.rs** -- Dice rolling and resource distribution.

### `player/` -- Player Abstraction

Async `Player` trait with methods for each decision point.

- **llm.rs** -- LLM player using tool-calling (JSON schemas for structured responses).
- **random.rs** -- Random player for testing and demo mode.
- **tui_human.rs** -- TUI human player communicating via channels.
- **personality.rs** -- TOML personality configs injected into system prompts.

### `trading/` -- Trade Negotiation

Multi-round trade protocol: propose, respond (accept/reject/counter), execute.

### `ui/` -- TUI

Built with ratatui + crossterm. Async game engine runs in a background tokio task; TUI runs on the main thread with communication via `mpsc` channels.

- **mod.rs** -- App state machine, event loop, input dispatch.
- **screens.rs** -- Non-game screens (menu, about, settings, docs).
- **layout.rs** -- Game screen layout with context-sensitive bottom panel.
- **board_view.rs** -- Hex board rendering.

## Testing

Unit tests are in-module (`#[cfg(test)]`). The TUI has dedicated test infrastructure:

```bash
cargo test ui::input_tests      # Input handling (55+ tests)
cargo test ui::snapshot_tests   # Visual snapshot tests (15+ tests)
cargo insta review              # Review snapshot diffs
```

Tests must be deterministic. Use seeded RNG where randomness is needed.

## Debug Logging

Logging writes to `~/.settl/debug.log`:

```bash
cargo run                       # Debug build: logging on automatically
SETTL_DEBUG=0 cargo run         # Debug build: force logging off
SETTL_DEBUG=1 cargo run --release  # Release build: force logging on
```

Use `log::debug!()` / `log::info!()` from any module. Never use `println!` or `eprintln!` in TUI code.

## Coding Conventions

- Keep code `cargo fmt` and `cargo clippy` clean
- Never use `#[allow(dead_code)]` -- delete unused code instead
- Rust naming: `snake_case` for modules/functions, `CamelCase` for types
- Prefer editing existing files over creating new ones

## Commits

- Branch names: `feature/...`, `fix/...`, `docs/...`, `refactor/...`
- Commit messages: conventional prefixes (`feat:`, `fix:`, `docs:`, `refactor:`)
