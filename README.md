
# Oxide

A Roguelike on Rust made from scratch, while knowing nothing about game development and nothing about Rust




## Aims

This project currently aims at teaching me Rust (and maybe some gamedev) in a more intuitive and self-motivating way, also is a way to sharpen my coding and problem solving skills

## The basic functionality needed

- Implement a grid tile/map system where every position of the grid is a tile that can be assigned different properties
- Will use ASCII Characters to represent tiles like the old school games until i can be bothered implementing visuals if ever
- Procedurally generated levels (i'm not 100% certain on what is the way that i think is best to implement this, ideally i would want to create an algorithm that can generate tiles based on defined room types and what they need to have, with maybe a couple fixed ones for special encounters, the super lazy approach is just to manually create the tilesets myself and just use weightings and a distribution algorithm to allocate the rooms on the grid)
- Concept of floors, new floors must match the stairs of the previous floor as a start point for their generation(for example if you have floor 1 [3,5] has a stair down on it, by consequence when the next floor is generated there must be a connecting stair at floor 2 [3,5]), potential complex problem to solve might be is when you have multiple stairs down you need to guarantee you generate a room with a stair at each correspoding floor grid position but you must also guarantee that all the generated rooms can access eachother (might be a problem if i do limited generation instead of just generating the entire floor then selecting the tiles after i'm not entirely sure)
- Implement a player character (only basic functionality)
- Implement entities (starting as basic enemies, only the most basic functionality, use Bresnaham Line of sight algo, search how to implement a pathfinding algo, potentially implement behavior but leave that after basic functionality is complete)
- Add items/collectables in the map (start with a simple victory condition type of thing, then probably implement an inventory feature and start doing more advanced logic around items)
- Implement a turn-based way of running the game (potentially huge issue here is how do you define priority of action? My initial brainstormed idea is to implement an action speed modifier to every entity in the game that can take action, sort the entity list by said action speed, then compute them on that order, i'm not sure how much of a terrible idea this is but it has to be explored)

## Technical ToDo list
### This is a collection of things i realize might need to change later to accomodate more advanced functionality

- Make it so that we can accept non-square room shapes and implement an algorithm to rotate the room arrays in order to make more interesting room shapes that can be rotated (long corridor room type for example)


## Ideas for (probably much) later
- Create a noise system that also is going to be used alongside the basic LoS algorithm for entities to detect your presence
- Inventory system
- Interaction trees for every action, item and entity (this could get severely complicated but should be easy to implement with small number of items and not having to make a decision tree that's 300+ cases long)
- Races and classes
- Alignment system
- Pantheon
- ✨ Magic ✨
- Alchemy