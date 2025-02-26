# Project DD2258

## Getting started

### Prerequisites

In order to compile the following is needed `rustc` (rust compiler) and `cargo` (rust package manager).

If you have these two components you can easily compile the project.

### Installation

#### Exe file

In case you do not have these dependencies (and dont want to download them), the program can be ran by running the provided `exe`. However, best practice is to inspect the code and compile it yourself. 

#### Compilation

To compile first `cd` into the `code` directory from the main folder.

```bash
cd code
```

Now to run the program simply run:
```bash
cargo run
```

Or if you would rather compile run (Output in the target folder):
```bash
cargo build --release
```

## Controls

Below are the controls for interacting with the project:

| Action        | Key / Button    | Description |
|--------------|---------------|-------------|
| Move Up      | `W`       | Moves the camera up|
| Move Down    | `S`       | Moves the camera down |
| Move Left    | `A`      | Moves the camera left |
| Move Right   | `D`     | Moves the camera right |
| Move Forward | `E`         | Moves the camera forward |
| Move Backward| `Q`         | Moves the camera backward |
| Quit         | `Esc`         | Quits the program |
| Toggle grass | `Backspace`    | Toggles the render of grass `on` or `off` |
| Increase grass density | `↑`       | Increases the amount of grass per quad |
| Decrease grass density   | `↓`       | Decreases the amount of grass per quad |
| Expand surface   | `→`       | Expands the spline surface |
| Shrink surface    | `←`       | Shrinks the spline surface|
| Spawn sphere | `Caps Lock`       | Spawns a falling sphere in the original quad|
| Select spline point | `LMB`       | Select a specific spline point **NOTE**: Is not entirely accurate|
| Move point up/down | `CTRL` + `Mouse up/down`       | Moves the selected spline point upward or downward (matches mouse movement)|
| Move point left/right| `CTRL` + `Mouse left/right`       | Moves the selected spline point left or right (matches mouse movement)|
| Move point forward/backward| `CTRL` + `Mouse wheel`       | Moves the selected spline point forward/backward (depending on scroll wheel direction)|

