---
title: AI Players
description: LLM providers, personalities, and spectator mode
order: 4
---

# AI Players

settl's AI players use tool-calling LLMs to make strategic decisions. Every choice comes with a reasoning explanation you can watch in real time.

## Llamafile (Default)

The default AI backend is [llamafile](https://github.com/mozilla-ai/llamafile), which runs LLMs locally with no API keys or internet connection. When you start a game with AI opponents, settl downloads and launches a llamafile server automatically.

Requirements:
- About 4GB of RAM for the default model
- First launch downloads the model file (one-time)

## Cloud Providers

For stronger AI play, use cloud LLM providers by setting API keys:

```bash
export ANTHROPIC_API_KEY=sk-ant-...   # Claude
export OPENAI_API_KEY=sk-...          # GPT
export GOOGLE_API_KEY=...             # Gemini
```

Then specify the model:

```bash
cargo run -- --headless --model claude-sonnet-4-6
```

Or assign different models to each player:

```bash
cargo run -- --models "claude-sonnet-4-6,gpt-4o-mini,claude-haiku-4-5-20251001"
```

## Personalities

AI players have configurable personalities that shape their play style through system prompt injection. Each personality has:

- **Aggression score** (0.0-1.0): how likely to make aggressive moves
- **Cooperation score** (0.0-1.0): how willing to accept trades
- **Style text**: describes the player's strategic approach
- **Catchphrases**: flavor text injected into reasoning

### Built-in Personalities

| Name | Style |
|------|-------|
| Default Strategist | Balanced play, adapts to the situation |
| Aggressive Trader | Pushes hard trades, builds fast |
| Grudge Holder | Remembers every slight, punishes betrayals |
| Cautious Builder | Slow and steady, avoids risk |
| Chaos Agent | Unpredictable, chaotic decisions |

### Custom Personalities

Create a TOML file:

```toml
[personality]
name = "The Diplomat"
style = "always seeks mutually beneficial trades, builds alliances"
aggression = 0.2
cooperation = 0.9
catchphrases = [
    "Let's find a deal that works for both of us.",
    "Cooperation is the path to victory."
]
```

Use it with:

```bash
cargo run -- --personality my_diplomat.toml
```

Or place TOML files in a `personalities/` directory for them to appear in the game setup screen.

## Spectator Mode

In TUI mode, Player 1 is always human. But you can watch AI-vs-AI games in headless mode:

```bash
cargo run -- --demo           # Random AI (fast, no API keys)
cargo run -- --headless       # LLM AI (requires keys or llamafile)
```

During gameplay, press `Tab` to toggle the AI reasoning panel, which shows each AI player's strategic thinking in real time. Use `Space` to pause/unpause, and `+`/`-` to adjust playback speed.
