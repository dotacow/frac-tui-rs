use ratatui::{
    prelude::*,
    symbols::Marker,
    widgets::canvas::Canvas,
    widgets::{Block, Borders, Paragraph},
};
use crate::hooks::App;
use crate::render::draw_fractal;

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    // 1. Use the full available area (No more forced square/black bars)
    let area = chunks[1];
    app.canvas_area = area;

    // 2. Calculate Aspect Ratio Correction
    // Terminal characters are roughly 2x taller than they are wide.
    // To make a circle look like a circle, we need to adjust the mathematical width.
    // Aspect = Width / (Height * 2.0)
    let aspect_ratio = area.width as f64 / (area.height as f64 * 2.0).max(1.0);

    // 3. Define Bounds
    // app.scale now represents the HEIGHT of the view in the complex plane.
    // The width is derived from the aspect ratio.
    let height = app.scale;
    let width = app.scale * aspect_ratio;

    let mut x_left = app.center_x - width / 2.0;
    let mut x_right = app.center_x + width / 2.0;
    let mut y_bottom = app.center_y - height / 2.0;
    let mut y_top = app.center_y + height / 2.0;

    // Safety Guard for NaN/Infinity
    if !x_left.is_finite() || !x_right.is_finite() || !y_bottom.is_finite() || !y_top.is_finite() {
        app.center_x = -0.75;
        app.center_y = 0.0;
        app.scale = 3.0;

        // Reset to safe defaults
        let aspect = area.width as f64 / (area.height as f64 * 2.0).max(1.0);
        let w = 3.0 * aspect;
        x_left = -0.75 - w/2.0;
        x_right = -0.75 + w/2.0;
        y_bottom = -1.5;
        y_top = 1.5;
    }

    let header_text = format!(
        "Mode: {:?} | Iters: {} | Arrows: Pan | Scroll: Zoom | S/D: +/- Iters | Space: Palette | B: Switch | Q: Quit",
        app.fractal_type,
        app.max_iters
    );

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(header, chunks[0]);

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Fractal"))
        .marker(Marker::Braille)
        .x_bounds([x_left, x_right])
        .y_bounds([y_bottom, y_top])
        .paint(move |ctx| {
            draw_fractal(ctx, x_left, x_right, y_bottom, y_top, app.palette, app.fractal_type, app.max_iters, area);
        });

    f.render_widget(canvas, area);
}