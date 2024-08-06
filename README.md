# Tetris
#### Video Demo: [Video](https://youtu.be/fgJEqse7zlU)
#### Description:
A classic tetris game that increases fall speed as scores get higher. Developed on my favorite language Rust, using an ECS framework called Bevy.

[Github repository](https://github.com/dgsantana/cs50_finalproject.git)

## FILES
- `main.rs`: Main entry point of the game. Contains the main bevy app setup.
- `common.rs`: Contains constants used throughout the game.
- `grid.rs`: Contains the setup code to draw the "cup" where the tetris blocks fall.
- `ui.rs`: Contains the code to draw the menus (main menu, game over menu and pause menu).
- `state.rs`: Contains the enums that define the game states. There are 2 enums, one for the app state and another for the game state as a sub-state of the app state.
- `stats.rs`: Contains code to show the player's stats: score, lines and high-score.
- `piece.rs`: This is the main module that setups all the game logic. It includes sub-modules for components, resources and systems.
    - `components.rs`: Contains the components that make up the tetris blocks.
    - `resources.rs`: Contains the resources that are used throughout the game.
    - `systems.rs`: Contains the systems that update the game state, checking input, collisions and game over condition.


Choosing ECS for this project was a nice challenge, since this could be easily realized with some other paradigms.
Each tetris piece is constructed from multiple blocks, where each block is a separate entity. While the piece is falling and didn't collide we keep a component `PieceType` that defines the piece type (I, J, L, O, S, T, Z). When the collision happens, we remove the PieceType component from all the blocks of that Piece, making it static. This allows to easily use the ECS query system to check for collisions, lines and game over conditions.
Using SystemSet's also allows us to enforce the order between the different systems, making sure the code is more readable and maintainable.

So the main systems in order of execution are:

### setup_game
Prepares a new game, by inserting the first 7 pieces into a queue as a resource which updates at every piece taken. We also insert the fall timer that is reduce each 500 points by 0.1s. And finally the input timer that allows the user to press the keys without making the piece move too fast.

#### add_piece
This system uses a query to check if there is any `PieceType` component in the world. If there isn't any, we then take from the `PiecesQueue` one piece (adding a new one to the end of the queue) and spawn the blocks of that piece, which are just sprites with a `Block` and `PieceType` component.

### rotate_piece
This system handles the piece rotation. It first checks if the user as pressed the rotate key, and then checks if the rotation is possible. We do this by first doing the rotation in a temporary `Piece` helper, and checking if all the blocks are in valid places inside the grid and not collinding with other blocks. If everything is ok, we update the piece blocks positions.

### move_piece
This systems does the main input logic as well as the "drop" logic. First checks if any of the timers has elapsed, if so we check what are the valid movements that a piece can do, by checking each block of the piece and the grid position as well as the other blocks (static) in the world. If we can move down and the auto move timer was the one that elapsed, we move the piece down and update the collision information. Then we check if the use did any input, and if so we check if the movement is possible and update the position of the piece, but for the down movement we only allow if the auto move didn't occur, this way the piece doesn't move down twice in the same frame.

### check_collisions
In this system we check if the piece is colliding with either the grid below (y=0) or if there is any block below colliding. If so, we remove the `PieceType` component making the piece static.

### remove_lines
This system handles score and line removal. We build a list of all the lines with the count of each block in that line. If any count is equal to 10 (the line width) we remove all the blocks and store the line number. Then in another loop we move the blocks above the removed line down. The score and line count is updated using the Bevy event system, that is listened by one of the stats systems.

### game_over_check
Finally we check if any block is above the grid, and if so we change the game state to `GameOver`.

## How to run the project
Since this project is done in Rust, you need to have Rust installed in your machine. You can install it by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).
After Rust is installed running `cargo r -r` should compile and run the project in release mode. For those that want to run in debug mode, you can run `cargo r` instead, and there is a crate included to help debug the project called `bevy-inspector-egui`. It's also possible to build and run for the Web, but requires some extra steps that I leave for the brave to try.

## Conclusion
This was a fun project, with some challenges, mostly related to the ECS paradigm, as well as the fact of Bevy is still new and in development, make for example handling the UI a bit more difficult. Overall I'm very happy with the result and I hope other enjoy playing it as much as I enjoyed developing it.