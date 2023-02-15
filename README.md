# Conway's Game of Life

Conway's Game of Life implemented in Rust using SDL2.

Reference: [domoritz/gameoflife-rust](https://github.com/domoritz/gameoflife-rust)

![Video GIF](https://thumbs.gfycat.com/LastingMindlessIndiancow-size_restricted.gif)

## Controls

### Camera
- W: Move camera up
- A: Move camera left
- S: Move camera down
- D: Move camera right

### Cells
- LMB: Make cell alive
- RMB: Make cell dead

### Game state
- SPACE: Toggle generation stepping
- DEL: Kill all alive cells (reset)

## Building

    cargo build --release

**Make sure the SDL2.dll is in the current working directory!**