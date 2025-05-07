<p align="center">
  <img src="https://avatars.githubusercontent.com/u/138057124?s=200&v=4" width="150" />
</p>
<h1 align="center">DOOM</h1>

<p align="center">This project demonstrates the core rendering technique used in early pseudo-3D games like Doom and Wolfenstein 3D.</p>

## Features

- Basic raycasting engine
- Colored walls
- Simple map
- Player movement and rotation
- Collision detection
- Floor and ceiling rendering

## Dependencies

- `minifb` crate for window management and graphics

## Installation

1.  Make sure you have Rust installed. If not, download it from [https://www.rust-lang.org/](https://www.rust-lang.org/)

2. Clone the repository
```bash
git clone https://github.com/WillKirkmanM/doom
```

3. Run DOOM
```bash
cargo run --release
```

## Usage

1.  Build and run the project:

    ```bash
    cargo run --release
    ```

2.  Use the following keys to control the player:

    -   `W`: Move forward
    -   `S`: Move backward
    -   `A`: Turn left
    -   `D`: Turn right
    -   `ESC`: Exit the game

## Code Structure

-   `src/main.rs`: Contains the main game loop and rendering logic.

    -   `WIDTH`: Width of the window.
    -   `HEIGHT`: Height of the window.
    -   `MAP_SIZE`: Size of the map (number of cells in each dimension).
    -   `get_map()`: Function that defines the game map. Walls are represented by `1`, and empty spaces by `0`. Different numbers represent different wall colors.
    -   `main()`:

        -   Initializes the window using `minifb`.
        -   Sets up player position, direction, and camera plane.
        -   Enters the main loop, handling player input and rendering.
        -   Renders the floor and ceiling.
        -   Implements the raycasting algorithm to draw the walls.

## Raycasting Algorithm

The raycasting algorithm works as follows:

1.  For each vertical column of pixels on the screen:

    -   Calculate the ray direction based on the player's direction and camera plane.
    -   Determine which map cell the ray starts in.
    -   Calculate the distances to the nearest grid lines in the x and y directions (`side_dist_x` and `side_dist_y`).
    -   Use Digital Differential Analysis (DDA) to step through the map until a wall is hit.
    -   Calculate the distance from the player to the hit wall.
    -   Determine the height of the wall on the screen based on the distance.
    -   Draw a vertical line representing the wall, with the color determined by the wall type in the map.

## Map Definition

The `get_map()` function defines the game map as a 2D array of `u8` values. Each value represents a different type of block:

-   `0`: Empty space
-   `1`: Red wall
-   `2`: Green wall
-   `3`: Blue wall
-   `4`: Yellow wall

## Movement and Rotation

-   The player's position is represented by `player_x` and `player_y`.
-   The player's direction is represented by a direction vector (`dir_x`, `dir_y`) and a camera plane vector (`plane_x`, `plane_y`).
-   The `W` and `S` keys move the player forward and backward, respectively.
-   The `A` and `D` keys rotate the player left and right, respectively.
-   Collision detection prevents the player from walking through walls.
