use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEvent, MouseEventKind};
use ratatui::layout::{Rect, Direction};
use crate::color::Palette;

const  MAX_ITER_DEFAULT: u32 = 1100;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FractalType {
    Mandelbrot,
    BurningShip,
}

#[derive(Clone)]
pub struct FractalPane {
    pub id: usize,
    pub center_x: f64,
    pub center_y: f64,
    pub scale: f64,
    pub palette: Palette,
    pub fractal_type: FractalType,
    pub max_iters: u32,
    pub area: Rect,
}

impl FractalPane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            center_x: -0.75,
            center_y: 0.0,
            scale: 3.0,
            palette: Palette::Classic,
            fractal_type: FractalType::Mandelbrot,
            max_iters: MAX_ITER_DEFAULT,
            area: Rect::default(),
        }
    }

    fn on_key(&mut self, key: KeyCode) {
        let move_amount = self.scale * 0.1;
        let zoom_factor = 0.9;

        match key {
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
                self.max_iters = MAX_ITER_DEFAULT;
            }
            KeyCode::Char(' ') => self.toggle_palette(),
            KeyCode::Char('b') => self.toggle_fractal_type(),
            KeyCode::Char('d') => self.max_iters = self.max_iters.saturating_add(10),
            KeyCode::Char('s') => self.max_iters = self.max_iters.saturating_sub(10).max(10),
            _ => {}
        }
    }

    fn on_mouse(&mut self, mouse: MouseEvent) {
        let area = self.area;
        let x = mouse.column;
        let y = mouse.row;

        if x < area.left() || x >= area.right() || y < area.top() || y >= area.bottom() {
            return;
        }

        let aspect_ratio = area.width as f64 / (area.height as f64 * 2.0).max(1.0);
        let math_width = self.scale * aspect_ratio;

        let norm_x = (x as f64 - area.left() as f64) / area.width as f64;
        let norm_y = (y as f64 - area.top() as f64) / area.height as f64;

        let mouse_world_x = (self.center_x - math_width / 2.0) + norm_x * math_width;
        let mouse_world_y = (self.center_y + self.scale / 2.0) - norm_y * self.scale;

        match mouse.kind {
            MouseEventKind::ScrollUp => {
                let new_scale = self.scale * 0.90;
                if new_scale < 1.0e-14 { return; }
                let new_width = new_scale * aspect_ratio;
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
}

pub enum PaneNode {
    Pane(FractalPane),
    Split {
        direction: Direction,
        children: Vec<PaneNode>,
    },
}

pub struct App {
    pub root: PaneNode,
    pub active_pane_id: usize,
    pub next_id: usize,
    pub should_quit: bool,
    pub show_quit_popup: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            root: PaneNode::Pane(FractalPane::new(0)),
            active_pane_id: 0,
            next_id: 1,
            should_quit: false,
            show_quit_popup: false,
        }
    }

    fn send_key_to_active(&mut self, key: KeyCode) {
        Self::traverse_and_handle_key(&mut self.root, self.active_pane_id, key);
    }

    fn traverse_and_handle_key(node: &mut PaneNode, target_id: usize, key: KeyCode) {
        match node {
            PaneNode::Pane(p) => {
                if p.id == target_id {
                    p.on_key(key);
                }
            }
            PaneNode::Split { children, .. } => {
                for child in children {
                    Self::traverse_and_handle_key(child, target_id, key);
                }
            }
        }
    }

    fn split_active(&mut self, direction: Direction, prepend: bool) {
        let new_id = self.next_id;
        self.next_id += 1;
        let new_pane = FractalPane::new(new_id);

        if Self::recursive_split(&mut self.root, self.active_pane_id, direction, prepend, new_pane) {
            self.active_pane_id = new_id;
        }
    }

    fn recursive_split(node: &mut PaneNode, target_id: usize, direction: Direction, prepend: bool, new_pane: FractalPane) -> bool {
        match node {
            PaneNode::Pane(p) => {
                if p.id == target_id {
                    let current_pane = p.clone();
                    let new_leaf = PaneNode::Pane(new_pane);
                    let current_leaf = PaneNode::Pane(current_pane);

                    let children = if prepend {
                        vec![new_leaf, current_leaf]
                    } else {
                        vec![current_leaf, new_leaf]
                    };

                    *node = PaneNode::Split { direction, children };
                    return true;
                }
                false
            }
            PaneNode::Split { children, .. } => {
                for child in children {
                    if Self::recursive_split(child, target_id, direction, prepend, new_pane.clone()) {
                        return true;
                    }
                }
                false
            }
        }
    }

    fn cycle_focus(&mut self) {
        let mut ids = Vec::new();
        Self::collect_ids(&self.root, &mut ids);

        if let Some(pos) = ids.iter().position(|&id| id == self.active_pane_id) {
            let next_pos = (pos + 1) % ids.len();
            self.active_pane_id = ids[next_pos];
        }
    }

    fn collect_ids(node: &PaneNode, ids: &mut Vec<usize>) {
        match node {
            PaneNode::Pane(p) => ids.push(p.id),
            PaneNode::Split { children, .. } => {
                for child in children {
                    Self::collect_ids(child, ids);
                }
            }
        }
    }

    fn pane_exists(&self, target_id: usize) -> bool {
        let mut ids = Vec::new();
        Self::collect_ids(&self.root, &mut ids);
        ids.contains(&target_id)
    }

    fn find_pane_at(&self, col: u16, row: u16) -> Option<usize> {
        Self::recursive_find_at(&self.root, col, row)
    }

    fn recursive_find_at(node: &PaneNode, col: u16, row: u16) -> Option<usize> {
        match node {
            PaneNode::Pane(p) => {
                let area = p.area;
                if col >= area.left() && col < area.right() && row >= area.top() && row < area.bottom() {
                    Some(p.id)
                } else {
                    None
                }
            }
            PaneNode::Split { children, .. } => {
                for child in children {
                    if let Some(id) = Self::recursive_find_at(child, col, row) {
                        return Some(id);
                    }
                }
                None
            }
        }
    }

    fn pass_mouse_to_active(&mut self, mouse: MouseEvent) {
        Self::recursive_mouse(&mut self.root, self.active_pane_id, mouse);
    }

    fn recursive_mouse(node: &mut PaneNode, target_id: usize, mouse: MouseEvent) {
        match node {
            PaneNode::Pane(p) => {
                if p.id == target_id {
                    p.on_mouse(mouse);
                }
            }
            PaneNode::Split { children, .. } => {
                for child in children {
                    Self::recursive_mouse(child, target_id, mouse);
                }
            }
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {


                if self.show_quit_popup {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            self.should_quit = true;
                        },
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            self.show_quit_popup = false;
                        }
                        _ => {}
                    }

                    return;
                }


                if let KeyCode::Char(c) = key.code {
                    if let Some(digit) = c.to_digit(10) {
                        if digit > 0 {
                            let target_id = (digit - 1) as usize;
                            if self.pane_exists(target_id) {
                                self.active_pane_id = target_id;
                            }
                            return;
                        }
                    }
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.show_quit_popup = true,
                    KeyCode::Char('D') => self.split_active(Direction::Vertical, false),
                    KeyCode::Char('U') => self.split_active(Direction::Vertical, true),
                    KeyCode::Char('R') => self.split_active(Direction::Horizontal, false),
                    KeyCode::Char('L') => self.split_active(Direction::Horizontal, true),
                    KeyCode::Tab => self.cycle_focus(),
                    _ => self.send_key_to_active(key.code),
                }
            }
            Event::Mouse(mouse) => {

                if self.show_quit_popup { return; }

                if let Some(id) = self.find_pane_at(mouse.column, mouse.row) {
                    match mouse.kind {
                        MouseEventKind::Down(_) | MouseEventKind::ScrollDown | MouseEventKind::ScrollUp => {
                            self.active_pane_id = id;
                        }
                        _ => {}
                    }
                    self.pass_mouse_to_active(mouse);
                }
            }
            _ => {}
        }
    }
}