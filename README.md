# settl

A terminal-based hex settlement game where LLMs play against each other (or you).
Watch Claude, GPT, and Gemini negotiate trades, form grudges, and compete for longest road -- all in your terminal.

## Quick Start

```bash
git clone https://github.com/Brake-Labs/settl.git
cd settl
cargo run
```

This launches the TUI. Select **New Game** to configure AI opponents and start playing. The default AI backend is [llamafile](https://github.com/mozilla-ai/llamafile) -- no API keys needed.

For headless AI-vs-AI games:

```bash
cargo run -- --demo            # Random AI, no API keys
cargo run -- --headless        # LLM AI via llamafile
cargo run -- --demo --seed 42  # Reproducible board
```

## Features

- **Full game rules** -- settlements, cities, roads, robber, development cards, trading, Longest Road, Largest Army
- **Play or spectate** -- Player 1 is human in TUI mode; watch AI opponents play around you
- **Multi-provider LLM players** -- Claude, GPT, Gemini, or any provider via [genai](https://crates.io/crates/genai)
- **Local AI** -- runs entirely offline with llamafile, no API keys required
- **Personality system** -- aggressive traders, grudge holders, cautious builders, chaos agents
- **Visible AI reasoning** -- watch each AI's strategic thinking in real time

## Documentation

Full docs are available in the `docs/` directory and at the [docs site](https://settl.dev):

- [Getting Started](docs/getting-started.md) -- Installation, first game, CLI options
- [How to Play](docs/how-to-play.md) -- Game rules, building, trading, winning
- [Controls](docs/controls.md) -- Keyboard shortcuts and interaction patterns
- [AI Players](docs/ai-players.md) -- LLM providers, personalities, spectator mode
- [Development](docs/development.md) -- Contributing, architecture, testing

Docs are also accessible from the TUI via the **Docs** menu item.

## License

Apache 2.0
