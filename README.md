# Ratatui Fractal Renderer

![frac-tui in action.](image.png)
A high-performance, multithreaded TUI (Terminal User Interface) fractal explorer written in Rust using [Ratatui]. It renders the Mandelbrot and Burning Ship fractals using Braille characters (⣿) to achieve 8× the resolution of standard terminal blocks.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Controls](#controls)
- [Technical Details](#technical-details)
- [License](#license)
- [Links & Credits](#links--credits)

## Features

- **High resolution**: Uses Braille markers (2×4 dots per character) for detailed rendering.
- **Multithreaded**: Uses `rayon` to compute the fractal in parallel across CPU cores.
- **Interactive controls**: Smooth panning and zooming (keyboard + mouse).
- **Dynamic resolution**: Adjusts calculation density when the terminal resizes.
- **Aspect ratio correction**: Maintains a 1:1 mathematical aspect ratio regardless of terminal window shape.
- **Multiple fractals**: Toggle between Mandelbrot and Burning Ship.
- **Color palettes**: Classic, Rainbow, and Magma themes.

## Installation

Ensure you have Rust and Cargo installed. For best performance, build and run in release mode:

```bash
# Clone the repository
git clone <repo-url>
cd frac-tui-rs

# Build and run in release mode (recommended)

```

Note: Debug builds are significantly slower due to heavy numerical calculations.

## Controls

- Arrow keys: Pan the view (Left / Right / Up / Down)
- Mouse wheel: Zoom in/out relative to cursor position
- `+` / `-`: Zoom in/out (center-focused)
- `Space`: Cycle color palettes (Classic → Rainbow → Magma)
- `b`: Switch fractal mode (Mandelbrot ↔ Burning Ship)
- `d`: Increase max iterations (more detail)
- `s`: Decrease max iterations (faster rendering)
- `r`: Reset view to default
- `q` / `Esc`: Quit application

## Technical Details

### Rendering strategy

The app uses the Ratatui Canvas widget and renders using Braille characters to increase effective resolution. To avoid computing points that are not visible, the renderer calculates the required density from the terminal's physical size:

- Width density: `terminal_width * 2`
- Height density: `terminal_height * 4`

### Parallelization

`rayon` splits the render loop into parallel iterators. Each worker computes a subset of the coordinate grid into thread-local buffers. These buffers are then merged into a final batch to be drawn by the main thread, avoiding mutex contention inside the hot loop.

### Coordinate system and aspect ratio

Terminal characters are roughly rectangular (≈ 1:2 width:height). The code corrects the mathematical viewport to preserve a 1:1 aspect ratio for the fractal, preventing vertical/horizontal stretching.

## License

Copyright (c) yousef K <ykitaneh22@gmail.com>

This project is licensed under the MIT license. See `LICENSE` for details.

## Links & Credits

- Ratatui: https://ratatui.rs
- Simple template used as a starting point: https://github.com/ratatui/templates/tree/main/simple
