---
title: How to Play
description: Game rules, resources, building, trading, and winning
order: 2
---

# How to Play

settl is a hex-based resource trading and building game for 2-4 players. Collect resources, build settlements and roads, trade with opponents, and race to 10 victory points.

## The Board

The board is a grid of 19 hexagonal tiles arranged in a 3-4-5-4-3 pattern. Each hex has a terrain type that produces a specific resource, and a number token (2-12, except 7).

| Terrain | Produces |
|---------|----------|
| Forest | Wood |
| Hills | Brick |
| Pasture | Sheep |
| Fields | Wheat |
| Mountains | Ore |
| Desert | Nothing |

Hexes meet at **vertices** (where you build settlements and cities) and share **edges** (where you build roads). The board is surrounded by ocean, with **ports** at certain coastal positions that give better trade rates.

## Setup Phase

Each game begins with a snake draft. In order (Player 1, 2, 3, 4, 4, 3, 2, 1), each player places one settlement and one road. Choose your starting positions carefully:

- Place settlements at vertices where multiple productive hexes meet
- Consider which numbers are on adjacent hexes (6 and 8 are rolled most often)
- Your second settlement's adjacent hexes give you starting resources

## Turns

On your turn:

1. **Roll dice** -- happens automatically. The sum determines which hexes produce resources. Every player with a settlement or city on a producing hex collects resources.
2. **Take actions** -- build, trade, or play development cards (in any order, as many as you can afford).
3. **End turn** -- press `e` when done.

## Building

| Structure | Cost | Victory Points | Effect |
|-----------|------|---------------|--------|
| Road | 1 Wood + 1 Brick | 0 | Extends your road network |
| Settlement | 1 Wood + 1 Brick + 1 Sheep + 1 Wheat | 1 | Collects 1 resource from adjacent hexes |
| City | 2 Wheat + 3 Ore | 2 (replaces settlement's 1) | Collects 2 resources from adjacent hexes |
| Dev Card | 1 Wheat + 1 Sheep + 1 Ore | varies | Draw from the deck (see below) |

**Placement rules:**
- Settlements must be on an empty vertex connected to one of your roads (except during setup)
- The **distance rule**: no settlement can be within one edge of another settlement or city
- Roads must connect to one of your existing roads, settlements, or cities
- Cities upgrade an existing settlement you own

**Piece limits:** 5 settlements, 4 cities, 15 roads per player.

## The Robber

When a **7** is rolled:

1. Any player with more than 7 resource cards must **discard** half (rounded down)
2. The rolling player **moves the robber** to any hex (except where it already is)
3. The rolling player **steals** one random resource from a player with a building on that hex

The robber blocks production: the hex it sits on produces nothing until the robber is moved again. Playing a Knight development card also lets you move the robber.

## Trading

There are two types of trades:

### Player Trading
Propose a trade to other players. They can accept, reject, or counter-offer. Both sides must have the resources they're offering. You cannot trade with yourself.

### Bank Trading
Trade resources directly with the bank at fixed rates:

| Access | Rate | How to Get |
|--------|------|------------|
| Default | 4:1 | Always available |
| Generic port | 3:1 | Settlement/city on a 3:1 port |
| Specific port | 2:1 | Settlement/city on a matching resource port |

## Development Cards

Buy a development card for 1 Wheat + 1 Sheep + 1 Ore. Cards go to your hand and can be played on a later turn (not the turn you bought them). You may play one dev card per turn.

| Card | Count | Effect |
|------|-------|--------|
| Knight | 14 | Move the robber and steal a resource |
| Victory Point | 5 | Worth 1 VP (revealed at game end) |
| Road Building | 2 | Place 2 roads for free |
| Year of Plenty | 2 | Take any 2 resources from the bank |
| Monopoly | 2 | Name a resource; all other players give you all of theirs |

## Special Awards

| Award | Requirement | Victory Points |
|-------|-------------|---------------|
| Longest Road | 5+ connected road segments, more than any other player | 2 |
| Largest Army | 3+ Knights played, more than any other player | 2 |

These awards transfer if another player surpasses the current holder.

## Winning

The first player to reach **10 victory points** on their turn wins. Points come from:

- Settlements: 1 VP each
- Cities: 2 VP each
- Victory Point dev cards: 1 VP each
- Longest Road: 2 VP
- Largest Army: 2 VP
