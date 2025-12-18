use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEvent, MouseEventKind};
use ratatui::layout::{Rect, Direction};
use crate::color::Palette;

const MAX_ITER_DEFAULT: u32 = 1100;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FractalType {
    Mandelbrot,
    BurningShip,
    Julia,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputField {
    Cx,
    Cy,
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
    pub julia_cx: f64,
    pub julia_cy: f64,
    pub active_input: Option<InputField>,
    pub input_buffer: String,
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
            julia_cx: -0.5125,
            julia_cy: 0.5213,
            active_input: None,
            input_buffer: String::new(),
        }
    }

    fn on_key(&mut self, key: KeyCode) {
        if let Some(field) = self.active_input {
            match key {
                KeyCode::Char(c) if c.is_ascii_digit() || c == '.' || c == '-' => {
                    self.input_buffer.push(c);
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Enter => {
                    if let Ok(val) = self.input_buffer.parse::<f64>() {
                        match field {
                            InputField::Cx => self.julia_cx = val,
                            InputField::Cy => self.julia_cy = val,
                        }
                    }
                    self.active_input = None;
                    self.input_buffer.clear();
                }
                KeyCode::Esc => {
                    self.active_input = None;
                    self.input_buffer.clear();
                }
                _ => {}
            }
            return;
        }

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
                self.julia_cx = -0.5125;
                self.julia_cy = 0.5213;
                self.active_input = None;
                self.input_buffer.clear();
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

        if self.fractal_type == FractalType::Julia {
            // Inputs at bottom-3 (Cx) and bottom-2 (Cy)
            let bottom = area.bottom();
            let cx_y = bottom.saturating_sub(3);
            let cy_y = bottom.saturating_sub(2);

            if y == cx_y || y == cy_y {
                let field = if y == cx_y { InputField::Cx } else { InputField::Cy };

                if let MouseEventKind::Down(_) = mouse.kind {
                    self.active_input = Some(field);
                    self.input_buffer.clear();
                }
                return;
            }
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
            FractalType::BurningShip => FractalType::Julia,
            FractalType::Julia => FractalType::Mandelbrot,
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
    pub show_help_popup: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            root: PaneNode::Pane(FractalPane::new(0)),
            active_pane_id: 0,
            next_id: 1,
            should_quit: false,
            show_quit_popup: false,
            show_help_popup: false,
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

    fn recursive_delete(node: &mut PaneNode, target_id: usize) -> bool {
        match node {
            PaneNode::Pane(_) => false,
            PaneNode::Split { children, .. } => {
                if let Some(pos) = children.iter().position(|child| match child {
                    PaneNode::Pane(p) => p.id == target_id,
                    _ => false
                }) {
                    children.remove(pos);
                    true
                } else {
                    let mut deleted = false;
                    for child in children.iter_mut() {
                        if Self::recursive_delete(child, target_id) {
                            deleted = true;
                            if let PaneNode::Split { children: sub_children, .. } = child {
                                if sub_children.len() == 1 {
                                    *child = sub_children.pop().unwrap();
                                }
                            }
                            break;
                        }
                    }
                    deleted
                }
            }
        }
    }

    fn close_active(&mut self) {
        let mut ids = Vec::new();
        Self::collect_ids(&self.root, &mut ids);
        if ids.len() <= 1 {
            return;
        }

        if Self::recursive_delete(&mut self.root, self.active_pane_id) {
            if let PaneNode::Split { children, .. } = &mut self.root {
                if children.len() == 1 {
                    self.root = children.pop().unwrap();
                }
            }
        }

        if !self.pane_exists(self.active_pane_id) {
            let mut new_ids = Vec::new();
            Self::collect_ids(&self.root, &mut new_ids);
            if let Some(&last_id) = new_ids.last() {
                self.active_pane_id = last_id;
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

    fn is_active_pane_capturing_input(&self) -> bool {
        Self::recursive_is_capturing(&self.root, self.active_pane_id)
    }

    fn recursive_is_capturing(node: &PaneNode, target_id: usize) -> bool {
        match node {
            PaneNode::Pane(p) => p.id == target_id && p.active_input.is_some(),
            PaneNode::Split { children, .. } => {
                children.iter().any(|child| Self::recursive_is_capturing(child, target_id))
            }
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                // INTERCEPT INPUT FOR POPUP
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

                if self.show_help_popup {
                    if let KeyCode::Esc | KeyCode::Char('h') | KeyCode::Char('q') = key.code {
                        self.show_help_popup = false;
                    }
                    return;
                }

                // If capturing input, pass everything to active pane except maybe global quit?
                // Actually, let's just prioritize the active pane for everything if it's capturing input.
                if self.is_active_pane_capturing_input() {
                    self.send_key_to_active(key.code);
                    return;
                }

                if let KeyCode::Char(c) = key.code {
                    if let Some(digit) = c.to_digit(10) {
                        if digit > 0 {
                            let index = (digit - 1) as usize;
                            // Collect IDs in visual order
                            let mut ids = Vec::new();
                            Self::collect_ids(&self.root, &mut ids);

                            // Map index (1st, 2nd...) to actual ID
                            if index < ids.len() {
                                self.active_pane_id = ids[index];
                            }
                            return;
                        }
                    }
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.show_quit_popup = true,
                    KeyCode::Char('h') => self.show_help_popup = true,
                    KeyCode::Char('D') => self.split_active(Direction::Vertical, false),
                    KeyCode::Char('U') => self.split_active(Direction::Vertical, true),
                    KeyCode::Char('R') => self.split_active(Direction::Horizontal, false),
                    KeyCode::Char('L') => self.split_active(Direction::Horizontal, true),
                    KeyCode::Char('X') => self.close_active(),
                    KeyCode::Tab => self.cycle_focus(),
                    _ => self.send_key_to_active(key.code),
                }
            }
            Event::Mouse(mouse) => {
                if self.show_quit_popup || self.show_help_popup { return; }

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