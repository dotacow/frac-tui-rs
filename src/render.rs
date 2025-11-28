use ratatui::widgets::canvas::Context;
use ratatui::layout::Rect;
use rayon::prelude::*;
use crate::color::{Palette, get_palette_colors};
use crate::utils::{calculate_mandelbrot, calculate_burning_ship};
use crate::hooks::FractalType;

/// Draws the fractal onto the canvas context
pub fn draw_fractal(
    ctx: &mut Context,
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    palette: Palette,
    fractal_type: FractalType,
    max_iters: u32,
    area: Rect,
) {
    let width = right - left;
    let height = top - bottom;

    // DYNAMIC RESOLUTION FIX:
    let density_x = (area.width as u32 * 2).max(10);
    let density_y = (area.height as u32 * 4).max(10);

    let colors = get_palette_colors(palette);
    let palette_len = colors.len();

    // Use Rayon to calculate points in parallel.
    // We use a fold/reduce pattern:
    // 1. fold: Each thread calculates points for a subset of columns and stores them in its own local vector.
    // 2. reduce: The thread-local vectors are merged together into the final batch result.
    // This avoids the need for Mutexes (which are slow) or huge single-vector allocations.
    let batches = (0..density_x).into_par_iter()
        .fold(
            || vec![Vec::new(); palette_len], // Initialize thread-local batches
            |mut local_batches, i| {
                for j in 0..density_y {
                    let x = left + (i as f64 / density_x as f64) * width;
                    let y = bottom + (j as f64 / density_y as f64) * height;

                    // Switch algorithm based on type
                    let iterations = match fractal_type {
                        FractalType::Mandelbrot => calculate_mandelbrot(x, y, max_iters),
                        FractalType::BurningShip => calculate_burning_ship(x, y, max_iters),
                    };

                    if iterations < max_iters {
                        let color_index = (iterations as usize) % palette_len;
                        local_batches[color_index].push((x, y));
                    }
                }
                local_batches
            }
        )
        .reduce(
            || vec![Vec::new(); palette_len], // Identity for reduction
            |mut global_batches, thread_batches| {
                // Merge thread_batches into global_batches
                for (i, points) in thread_batches.into_iter().enumerate() {
                    global_batches[i].extend(points);
                }
                global_batches
            }
        );

    // Draw the batched points to the Ratatui Context (Main Thread)
    // Context drawing must happen sequentially as it is not thread-safe.
    for (i, points) in batches.iter().enumerate() {
        if !points.is_empty() {
            ctx.draw(&ratatui::widgets::canvas::Points {
                coords: points,
                color: colors[i],
            });
        }
    }
}