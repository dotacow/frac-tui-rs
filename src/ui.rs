use ratatui::{
    prelude::*,
    symbols::Marker,
    widgets::canvas::Canvas,
    widgets::{Block, Borders, Paragraph, Clear, Table, Row},
};
use crate::hooks::{App, PaneNode, FractalType, InputField};
use crate::render::draw_fractal;

pub fn ui(f: &mut Frame, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    let header_text = "Shift+{u,d,l,r}: Split Pane | Shift+x: Close Pane | Tab: Cycle | {1..9}: Switch | r: Reset | Click: Focus | h: Help | Q: Quit";

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Frac-tui"))
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(header, main_layout[0]);

    let mut pane_counter = 1;
    draw_tree(f, &mut app.root, main_layout[1], app.active_pane_id, &mut pane_counter);

    if app.show_help_popup {
        let popup_area = centered_rect(60, 60, f.area());

        let rows = vec![
            Row::new(vec!["Key", "Action"]),
            Row::new(vec!["Arrow Keys", "Pan view"]),
            Row::new(vec!["Mouse Wheel", "Zoom in/out (cursor)"]),
            Row::new(vec!["+/-", "Zoom in/out (center)"]),
            Row::new(vec!["Space", "Cycle Palette"]),
            Row::new(vec!["b", "Cycle Fractal Type"]),
            Row::new(vec!["d/s", "Increase/Decrease Iterations"]),
            Row::new(vec!["r", "Reset View"]),
            Row::new(vec!["Shift + Arrow", "Split Pane (Direction)"]),
            Row::new(vec!["Shift + x", "Close Active Pane"]),
            Row::new(vec!["Tab", "Cycle Focus"]),
            Row::new(vec!["1-9", "Switch Focus to Pane #"]),
            Row::new(vec!["h", "Toggle Help"]),
            Row::new(vec!["q / Esc", "Quit"]),
        ];

        let table = Table::new(rows, [Constraint::Percentage(30), Constraint::Percentage(70)])
            .block(Block::default().title(" Help ").borders(Borders::ALL))
            .header(Row::new(vec!["Key", "Action"]).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
            .column_spacing(1);

        f.render_widget(Clear, popup_area);
        f.render_widget(table, popup_area);
    }

    if app.show_quit_popup {
        let popup_area = centered_rect(60, 20, f.area());

        let popup_block = Paragraph::new("Are you sure you want to quit?\n\n(y) Yes / (n) No")
            .block(
                Block::default()
                    .title(" Warning ")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black).fg(Color::Red))
            )
            .alignment(Alignment::Center);


        f.render_widget(Clear, popup_area);
        f.render_widget(popup_block, popup_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn draw_tree(f: &mut Frame, node: &mut PaneNode, area: Rect, active_id: usize, pane_counter: &mut usize) {
    match node {
        PaneNode::Pane(pane) => {
            pane.area = area;

            let is_active = pane.id == active_id;
            let border_style = if is_active {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let aspect_ratio = area.width as f64 / (area.height as f64 * 2.0).max(1.0);
            let height = pane.scale;
            let width = pane.scale * aspect_ratio;

            let mut x_left = pane.center_x - width / 2.0;
            let mut x_right = pane.center_x + width / 2.0;
            let mut y_bottom = pane.center_y - height / 2.0;
            let mut y_top = pane.center_y + height / 2.0;

            if !x_left.is_finite() || !x_right.is_finite() || !y_bottom.is_finite() || !y_top.is_finite() {
                pane.center_x = -0.75;
                pane.center_y = 0.0;
                pane.scale = 3.0;
                let aspect = area.width as f64 / (area.height as f64 * 2.0).max(1.0);
                let w = 3.0 * aspect;
                x_left = -0.75 - w/2.0;
                x_right = -0.75 + w/2.0;
                y_bottom = -1.5;
                y_top = 1.5;
            }

            let p_palette = pane.palette;
            let p_type = pane.fractal_type;
            let p_iters = pane.max_iters;
            let p_julia_cx = pane.julia_cx;
            let p_julia_cy = pane.julia_cy;

            let display_id = *pane_counter;
            *pane_counter += 1;

            let title = format!("{}: [{:?}, {}, {:?}]", display_id, p_type, p_iters, p_palette);

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(title)
                        .border_style(border_style)
                )
                .marker(Marker::Braille)
                .x_bounds([x_left, x_right])
                .y_bounds([y_bottom, y_top])
                .paint(move |ctx| {
                    draw_fractal(ctx, x_left, x_right, y_bottom, y_top, p_palette, p_type, p_iters, area, p_julia_cx, p_julia_cy);
                });

            f.render_widget(canvas, area);

            if p_type == FractalType::Julia {
                let cx_area = Rect {
                    x: area.x + 1,
                    y: area.bottom().saturating_sub(3),
                    width: area.width.saturating_sub(2),
                    height: 1,
                };
                let cy_area = Rect {
                    x: area.x + 1,
                    y: area.bottom().saturating_sub(2),
                    width: area.width.saturating_sub(2),
                    height: 1,
                };

                let cx_text = if pane.active_input == Some(InputField::Cx) {
                    format!("Cx: {}_", pane.input_buffer)
                } else {
                    format!("Cx: {:.4}", pane.julia_cx)
                };

                let cy_text = if pane.active_input == Some(InputField::Cy) {
                    format!("Cy: {}_i", pane.input_buffer)
                } else {
                    format!("Cy: {:.4}", pane.julia_cy)
                };

                let cx_style = if pane.active_input == Some(InputField::Cx) {
                    Style::default().fg(Color::Yellow).bg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::Yellow)
                };

                let cy_style = if pane.active_input == Some(InputField::Cy) {
                    Style::default().fg(Color::Yellow).bg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::Yellow)
                };

                let cx_widget = Paragraph::new(cx_text).style(cx_style);
                let cy_widget = Paragraph::new(cy_text).style(cy_style);

                f.render_widget(Clear, cx_area);
                f.render_widget(cx_widget, cx_area);

                f.render_widget(Clear, cy_area);
                f.render_widget(cy_widget, cy_area);
            }
        }
        PaneNode::Split { direction, children } => {
            if children.is_empty() { return; }

            let ratio = 100 / children.len() as u16;
            let constraints: Vec<Constraint> = children.iter()
                .map(|_| Constraint::Percentage(ratio))
                .collect();

            let chunks = Layout::default()
                .direction(*direction)
                .constraints(constraints)
                .split(area);

            for (i, child) in children.iter_mut().enumerate() {
                if i < chunks.len() {
                    draw_tree(f, child, chunks[i], active_id, pane_counter);
                }
            }
        }
    }
}