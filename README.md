# Thales

Thales is an experimental Rust project exploring how much of the world can
be modelled using cellular automata.

The project sits somewhere between a simulation and a game. The idea is to
build a world from simple local rules and see what larger-scale behaviour
emerges. The player can interact with the world, while parts of the world
continue to evolve according to the rules of the simulation.

The long-term ambition is deliberately open-ended: to experiment with
cellular-automaton models of increasingly complex phenomena, including
settlement, urban growth, migration, economies, conflict, climate and
natural disasters.

## Current prototype

The current version is a terminal-based prototype.

It generates a procedural terrain map from a random height field and smooths
the field before classifying cells as water, mountains or grass. Cities can
then emerge from local interactions between neighbouring cells.

The player controls settler units and can:

- move settlers around the map;
- found cities;
- advance the simulation one step at a time.

As the simulation advances, cities appear and disappear according to their
local environment. When a city reaches a particular local configuration, it
can produce a new settler.

The current rules are intentionally simple. The purpose of the prototype is
to explore the underlying idea rather than to provide a realistic model of
urban development.

## Cellular automata

The world is represented as a grid of cells. Each cell has a state, and the
state of a cell can change according to the states of its neighbours.

This provides a simple mechanism for modelling systems in which large-scale
patterns emerge from local interactions.

Thales is an attempt to push this idea as far as possible: starting with
simple terrain and settlement rules and gradually experimenting with more
complex systems.

## Technology

- Rust
- `ratatui` — terminal user interface
- `crossterm` — terminal input and screen handling
- `rand` — procedural terrain generation

## Current architecture

The prototype is organised around three main concepts:

- **Map** — stores the grid and provides operations for accessing cells,
  checking boundaries and inspecting neighbouring cells.
- **Game** — manages the map, units and player interaction.
- **Simulation step** — applies the cellular-automaton rules to the current
  map and produces changes to the world.

Terrain is initially generated from a random height map. A simple smoothing
operation is applied repeatedly before the resulting values are converted
into terrain types.

## Controls

| Key | Action |
|-----|--------|
| Arrow keys | Move selected settler |
| `Tab` | Select next settler |
| `b` | Found a city |
| `n` | Advance the simulation |
| `q` | Quit |

## Project status

Thales is an early-stage experimental project.

The current implementation is a small prototype rather than a complete
simulation engine or game. The architecture, simulation rules and nature of
the player interaction are still being explored.

The project will evolve by gradually replacing simple rules with richer
models
and investigating which kinds of large-scale behaviour can emerge from
local interactions.
