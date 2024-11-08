use std::{error::Error, io};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout(); // This is a special case. Normally using stdout is fine
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Tab => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::TileColor);
                    }
                    _ => {}
                },
                CurrentScreen::Editing => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Enter => {
                        app.go_next_row();
                    }
                    KeyCode::Backspace => {
                        app.remove_char();
                    }
                    KeyCode::Char(c) => {
                        for upper_c in c.to_uppercase() {
                            app.insert_char(upper_c);
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => {
                    return Ok(true);
                },
            }
        }
    }
}