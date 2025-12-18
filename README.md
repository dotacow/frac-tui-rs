# **Ratatui Fractal Renderer**

A high-performance, multithreaded TUI (Terminal User Interface) fractal explorer written in Rust using \[Ratatui\]. It functions as a tiling window manager for fractal simulations, allowing you to run multiple independent renders of the Mandelbrot and Burning Ship sets simultaneously.

![coolandsickfractaltui](frac-tui.png)

![secondcoolfractal](ex2.png) ![ex3](ex3.png)
## **Features**

* **Tiling Window Manager**: Split the screen horizontally or vertically (Tmux-style) to view multiple fractals at once.
* **High resolution**: Uses Braille markers (2×4 dots per character) for detailed rendering.
* **Multithreaded**: Uses rayon to compute the fractal in parallel across CPU cores.
* **Independent Simulations**: Each pane has its own state (zoom, position, palette, fractal type, iteration limit).
* **Interactive controls**: Smooth panning and zooming (keyboard \+ mouse) with mouse focus support.
* **Dynamic resolution**: Adjusts calculation density automatically when the terminal or pane resizes.
* **Aspect ratio correction**: Maintains a 1:1 mathematical aspect ratio regardless of pane shape.
* **Multiple fractals**: Toggle between Mandelbrot and Burning Ship.
* **Color palettes**: Classic, Rainbow, and Magma themes.

## **Installation**

Ensure you have Rust and Cargo installed. For best performance build and run in release mode:

```bash
git clone https://github.com/dotacow/frac-tui-rs
cd frac-tui-rs
```
run as any standard rust project:
```bash
cargo run --release
```
## **Controls**

### **Window Management**

* **Shift \+ U**: Split active pane **Up**
* **Shift \+ D**: Split active pane **Down**
* **Shift \+ L**: Split active pane **Left**
* **Shift \+ R**: Split active pane **Right**
* **Shift \+ X**: Close active pane
* **Tab**: Cycle focus between panes
* **1-9**: Instantly switch focus to pane \#1–9
* **Mouse Click**: Focus specific pane

### **Fractal Navigation (Active Pane)**

* **Arrow keys**: Pan the view
* **Mouse wheel**: Zoom in/out relative to cursor
* **\+ / \-**: Zoom in/out (center-focused)
* **r**: Reset view to default

### **Simulation Settings**

* **Space**: Cycle color palettes (Classic → Rainbow → Magma)
* **b**: Switch fractal mode (Mandelbrot ↔ Burning Ship)
* **d**: Increase max iterations (more detail)
* **s**: Decrease max iterations (faster rendering)

### **Application**

* **q / Esc**: Open Quit Confirmation
* **y / n**: Confirm or Cancel Quit

## **Technical Details**

### **Architecture**

The application uses a **recursive tree structure** (PaneNode) to manage layout. Panes can be leaves (rendering a fractal) or splits (containing children). This allows for infinite nesting of vertical and horizontal splits, similar to multiplexers like Tmux or i3.

### **Rendering strategy**

The app uses the Ratatui Canvas widget and renders using Braille characters. To optimize performance, the renderer calculates the required density based on the specific Rect area of the active pane:

* Width density: pane\_width \* 2
* Height density: pane\_height \* 4

### **Parallelization**

rayon is used to parallelize the pixel calculation. Each worker computes a subset of the coordinate grid into thread-local buffers using a map-reduce pattern. These buffers are merged into a final batch for drawing, ensuring the UI remains responsive even at high iteration depths.

## **License**

Copyright (c) yousef K [ykitaneh22@gmail.com](mailto:ykitaneh22@gmail.com)

This project is licensed under the MIT license. See LICENSE for details.

## **Links & Credits**

* Ratatui: https://ratatui.rs
* Simple template used as a starting point: https://github.com/ratatui/templates/tree/main/simple
