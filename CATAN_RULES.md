# Catan Rules Reference

*Based on the official Catan rules by Klaus Teuber (5th Edition).*

## Overview

Catan is played on a variable game board made up of 19 terrain hexes surrounded by ocean. Players settle the island by building roads, settlements, and cities, competing to be the first to reach **10 victory points**.

## Game Components

- 19 terrain hexes
- 6 sea frame pieces
- 9 harbor pieces
- 18 circular number tokens (2-12, no 7)
- 95 resource cards (brick, grain, lumber, ore, wool)
- 25 development cards (14 knight, 6 progress, 5 victory point)
- 4 building costs cards
- 2 special cards: Longest Road and Largest Army
- 16 cities (4 per color)
- 20 settlements (5 per color)
- 60 roads (15 per color)
- 2 dice, 1 robber

## Terrain Types and Resources

| Terrain    | Resource |
|------------|----------|
| Hills      | Brick    |
| Forest     | Lumber   |
| Mountains  | Ore      |
| Fields     | Grain    |
| Pasture    | Wool     |
| Desert     | Nothing  |

## Board Geometry

- **Intersections**: Points where 3 hexes meet. Settlements and cities are placed on intersections.
- **Paths**: Edges between two hexes (or a hex and the frame). Roads are placed on paths. Each path connects two intersections.
- **Coast**: Where a terrain hex borders the sea frame.

## Setup Phase

### Pieces Per Player

Each player takes: 5 settlements, 4 cities, 15 roads, 1 building costs card.

### Initial Placement (Snake Draft)

**Round One**: Starting player places 1 settlement + 1 adjacent road. Continues clockwise. Each player places 1 settlement and 1 road.

**Round Two**: The *last* player from round one goes first. Continues *counterclockwise* (reverse order). Each player places their second settlement + 1 adjacent road.

After placing the second settlement, each player receives 1 resource card for each terrain hex adjacent to that second settlement.

The starting player (who placed last in round two) takes the first turn.

### Placement Rules

- The **Distance Rule** always applies: a settlement may only be placed on an intersection if all 3 adjacent intersections are vacant (no settlements or cities, even your own).
- Each settlement must connect to at least 1 of your own roads (except during initial placement).

## Turn Structure

On your turn, execute these phases in order:

### 1. Roll for Resource Production

Roll both dice. The sum determines which terrain hexes produce resources.

- Every player with a settlement adjacent to a hex with the rolled number receives **1 resource card** of that hex's type.
- Every player with a city adjacent to such a hex receives **2 resource cards**.
- If there are not enough of a resource to fulfill all production, **no one** receives that resource (unless the shortage affects only 1 player, in which case they receive what's available).
- The robber blocks production: hexes containing the robber produce nothing.

### 2. Trade

You may trade resources using either or both methods:

#### Domestic Trade

- Trade resource cards with any other player on mutually agreed terms.
- Only the active player (whose turn it is) may participate in trades. Other players may not trade among themselves.
- You may not give away cards for free.
- You may not trade identical resources (e.g., 2 wool for 1 wool).

#### Maritime Trade

Trade with the bank (no other player needed):

| Condition | Rate |
|-----------|------|
| Default (always available) | 4:1 -- trade 4 identical resources for any 1 |
| Generic harbor (3:1) | 3:1 -- trade 3 identical resources for any 1 |
| Special harbor (2:1) | 2:1 -- trade 2 of the *specific* resource shown for any 1 |

You must have a settlement or city on the harbor intersection to use its rate.

### 3. Build

Pay the required resources to build. You may build as many things as you can afford in a single turn.

## Building Costs

| Structure       | Cost                              |
|-----------------|-----------------------------------|
| Road            | 1 Brick + 1 Lumber                |
| Settlement      | 1 Brick + 1 Lumber + 1 Wool + 1 Grain |
| City (upgrade)  | 3 Ore + 2 Grain                   |
| Development Card| 1 Ore + 1 Wool + 1 Grain          |

### Supply Limits

You can never build more pieces than your supply holds: **5 settlements, 4 cities, 15 roads**. When you upgrade a settlement to a city, the settlement returns to your supply.

## Building Rules

### Roads

- Must connect to one of your existing roads, settlements, or cities.
- Only 1 road per path.
- Can be built along the coast.

### Settlements

- Must be built on an unoccupied intersection.
- Must observe the **Distance Rule**: all 3 adjacent intersections must be vacant.
- Must connect to at least 1 of your own roads.
- Worth **1 victory point**.

### Cities

- You cannot build a city directly; you must **upgrade** an existing settlement.
- Replace the settlement piece with a city piece on the same intersection. The settlement returns to your supply.
- Cities produce **2 resource cards** instead of 1 when the adjacent hex's number is rolled.
- Worth **2 victory points**.

### Development Cards

- Draw the top card from the development card deck.
- Keep development cards hidden until played.
- Development cards cannot be traded or given away.
- There are 3 types: Knight, Progress, and Victory Point.

## Rolling a 7 and the Robber

When a 7 is rolled:

1. **Discard**: Every player holding **more than 7 resource cards** must discard half (rounded down).
2. **Move the robber**: The player who rolled must move the robber to any *other* terrain hex's number token (or the desert). The robber may not stay where it is.
3. **Steal**: The player who moved the robber steals 1 random resource card from any one opponent who has a settlement or city adjacent to the robber's new hex. If multiple opponents qualify, the player chooses which to steal from.

While the robber occupies a hex, that hex produces **no resources** when its number is rolled.

## Development Cards

You may play **1 development card** per turn (knight or progress). You may play it at any time during your turn, including before rolling. You may **not** play a card you bought on the same turn.

**Exception**: Victory point cards may be revealed on the turn they are purchased (only to win).

### Knight Cards (14 total)

When played, you must immediately move the robber (same rules as rolling a 7, steps 2-3). Played knights stay face up in front of you.

### Progress Cards (6 total, 2 of each)

**Road Building**: Immediately place 2 free roads (following normal rules).

**Year of Plenty**: Immediately take any 2 resource cards from the supply.

**Monopoly**: Name 1 resource type. All other players must give you all cards of that type from their hand.

Progress cards are removed from the game after use.

### Victory Point Cards (5 total)

Each is worth 1 VP. Keep them hidden. Reveal only on your turn when you have enough points to win (or at game end).

Named cards: Library, Market, Chapel, Great Hall, University.

## Special Cards

### Longest Road (2 VP)

Awarded to the first player who builds a continuous road of **at least 5 segments**. Only the single longest branch counts (forks are ignored).

If another player builds a longer continuous road, they immediately take the card.

**Road breaking**: Building a settlement on an unoccupied intersection along an opponent's road breaks that road into two segments for longest road purposes.

**Ties**: If the holder's road is broken and they tie with another player, they keep the card. If no single player has the longest road (tie or no one has 5+), the card is set aside until one player has a clear longest road of 5+.

### Largest Army (2 VP)

Awarded to the first player to play **3 knight cards**. If another player plays more knights, they immediately take the card.

## Victory Points

| Source | VP |
|--------|-----|
| Settlement | 1 |
| City | 2 |
| Longest Road card | 2 |
| Largest Army card | 2 |
| Victory Point card | 1 each |

Each player starts with 2 settlements = **2 VP**. You need **8 more** to win.

## Ending the Game

The first player to reach **10 or more victory points during their own turn** wins immediately. If you reach 10 VP during another player's turn, you must wait until your turn to claim victory.

## Dice Roll Probabilities

| Roll | Probability |
|------|-------------|
| 2, 12 | ~3% (1 way each) |
| 3, 11 | ~6% (2 ways each) |
| 4, 10 | ~8% (3 ways each) |
| 5, 9 | ~11% (4 ways each) |
| 6, 8 | ~14% (5 ways each) |
| 7 | ~17% (6 ways) |

## Tactical Tips

- **Brick and lumber** are critical early for roads and settlements. Prioritize at least 1 starting settlement on a good hills or forest hex.
- **Harbors** are powerful. A player producing lots of one resource should aim for that resource's 2:1 harbor.
- **Leave room to expand** when placing initial settlements. Avoid getting surrounded or cut off from the coast.
- **Trade aggressively.** The more you trade, the better your chances.
