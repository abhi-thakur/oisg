mod components;
mod app;
mod styles;
mod common;

use std::error::Error;
use std::io;
use std::time::{ Duration, Instant };

use crossterm::{
    ExecutableCommand,
    terminal::{
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    },
    event
};

use tui::{
    backend::CrosstermBackend,
    terminal::Terminal
};
use crate::components::{BaseComponent, DrawableComponent};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal for drawing
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;

    // init terminal with backend
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    // create application
    let mut application = app::application::Application::new();

    let tick_rate = Duration::from_millis(200);
    let last_tick = Instant::now();

    terminal.draw(|f| application.draw(f, f.size()))?;

    // wait for use to press any key
    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_millis(0));

        if event::poll(timeout)? {
            // just let application consume the event
            match application.event(event::read()?) {
                Ok(consumed) if consumed => {
                    terminal.draw(|f| application.draw(f, f.size()))?;
                }
                _ => {}
            }
        }

        if application.is_quit() {
            break;
        }
    }

    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}
