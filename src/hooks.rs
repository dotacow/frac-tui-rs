use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEventKind, MouseEvent};
use ratatui::layout::Rect;
use crate::color::Palette;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FractalType {
    Mandelbrot,
    BurningShip,
}

/// App holds the state of our application.
pub struct App {
    pub center_x: f64,
    pub center_y: f64,
    /// Scale now represents the vertical height of the viewport
    pub scale: f64,
    pub should_quit: bool,
    pub palette: Palette,
    pub fractal_type: FractalType,
    pub max_iters: u32,
    pub canvas_area: Rect,
}

impl App {
    pub fn new() -> Self {
        Self {
            center_x: -0.75,
            center_y: 0.0,
            scale: 3.0,
            should_quit: false,
            palette: Palette::Classic,
            fractal_type: FractalType::Mandelbrot,
            max_iters: 112,
            canvas_area: Rect::default(),
        }
    }

    pub fn toggle_palette(&mut self) {
        self.palette = match self.palette {
            Palette::Classic => Palette::Rainbow,
            Palette::Rainbow => Palette::Magma,
            Palette::Magma => Palette::Classic,
        };
    }

    pub fn toggle_fractal_type(&mut self) {
        self.fractal_type = match self.fractal_type {
            FractalType::Mandelbrot => FractalType::BurningShip,
            FractalType::BurningShip => FractalType::Mandelbrot,
        };
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                self.on_key(key.code);
            }
            Event::Mouse(mouse) => {
                self.on_mouse(mouse);
            }
            _ => {}
        }
    }

    fn on_key(&mut self, key: KeyCode) {
        let move_amount = self.scale * 0.1;
        let zoom_factor = 0.9;

        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Left => self.center_x -= move_amount,
            KeyCode::Right => self.center_x += move_amount,
            KeyCode::Up => self.center_y += move_amount,
            KeyCode::Down => self.center_y -= move_amount,
            KeyCode::Char('+') | KeyCode::Char('=') => self.scale *= zoom_factor,
            KeyCode::Char('-') | KeyCode::Char('_') => self.scale /= zoom_factor,
            KeyCode::Char('r') => {
                self.center_x = -0.75;
                self.center_y = 0.0;
                self.scale = 3.0;
                self.max_iters = 112;
            }
            KeyCode::Char(' ') => self.toggle_palette(),
            KeyCode::Char('b') => self.toggle_fractal_type(),
            KeyCode::Char('d') => self.max_iters = self.max_iters.saturating_add(10),
            KeyCode::Char('s') => self.max_iters = self.max_iters.saturating_sub(10).max(10),
            _ => {}
        }
    }

    fn on_mouse(&mut self, mouse: MouseEvent) {
        let area = self.canvas_area;
        let x = mouse.column;
        let y = mouse.row;

        if x < area.left() || x >= area.right() || y < area.top() || y >= area.bottom() {
            return;
        }

        // We must calculate the aspect ratio here as well to map clicks correctly
        let aspect_ratio = area.width as f64 / (area.height as f64 * 2.0).max(1.0);
        let math_height = self.scale;
        let math_width = self.scale * aspect_ratio;

        let norm_x = (x as f64 - area.left() as f64) / area.width as f64;
        let norm_y = (y as f64 - area.top() as f64) / area.height as f64;

        // Use the aspect-corrected width for X coordinate calculation
        let mouse_world_x = (self.center_x - math_width / 2.0) + norm_x * math_width;
        let mouse_world_y = (self.center_y + math_height / 2.0) - norm_y * math_height;

        match mouse.kind {
            MouseEventKind::ScrollUp => {
                let new_scale = self.scale * 0.90;
                if new_scale < 1.0e-14 { return; }

                // Recalculate Width for the NEW scale
                let new_width = new_scale * aspect_ratio;

                // Adjust center based on new dimensions
                self.center_x = mouse_world_x - (norm_x - 0.5) * new_width;
                self.center_y = mouse_world_y - (0.5 - norm_y) * new_scale;
                self.scale = new_scale;
            }
            MouseEventKind::ScrollDown => {
                let new_scale = self.scale * 1.10;
                let new_width = new_scale * aspect_ratio;

                self.center_x = mouse_world_x - (norm_x - 0.5) * new_width;
                self.center_y = mouse_world_y - (0.5 - norm_y) * new_scale;
                self.scale = new_scale;
            }
            _ => {}
        }
    }
}