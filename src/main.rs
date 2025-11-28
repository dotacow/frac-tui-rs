mod hooks;
mod color;
mod render;
mod ui;
mod utils;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::{self, stdout};

use crate::hooks::App;
use crate::ui::ui;

fn main() -> io::Result<()> {
    // 1. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Initialize App State
    let mut app = App::new();

    // 3. Main Loop
    loop {
        // Pass &mut app so we can update the canvas_area during render
        terminal.draw(|f| ui(f, &mut app))?;

        let event = event::read()?;
        app.handle_event(event);

        if app.should_quit {
            break;
        }
    }

    // 4. Cleanup Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}