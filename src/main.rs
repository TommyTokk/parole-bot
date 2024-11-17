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
pub mod solver;
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
                        app.current_screen = CurrentScreen::EditingTileChar;
                        app.currently_editing = Some(CurrentlyEditing::TileChar);
                    }
                    _ => {}
                },
                CurrentScreen::EditingTileChar => match key.code {
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
                    KeyCode::Left =>{
                        app.go_prev_col();
                    }
                    KeyCode::Right =>{
                        app.go_next_col();
                    }
                    KeyCode::Char(c) => {
                        for upper_c in c.to_uppercase() {
                            app.insert_char(upper_c);
                        }
                    }
                    KeyCode::Tab => {
                        app.current_screen = CurrentScreen::EditingTileColor;
                        app.currently_editing = Some(CurrentlyEditing::TileColor);
                    }
                    _ => {}
                },
                CurrentScreen::EditingTileColor => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Left =>{
                        app.go_prev_col();
                    }
                    KeyCode::Right =>{
                        app.go_next_col();
                    }
                    KeyCode::Up =>{
                        app.go_prev_row();
                    }
                    KeyCode::Down =>{
                        app.go_next_row();
                    }
                    KeyCode::Char('n') => {
                        app.go_next_color();
                    }
                    KeyCode::Tab => {
                        app.current_screen = CurrentScreen::EditingTileChar;
                        app.currently_editing = Some(CurrentlyEditing::TileChar);
                    }
                    KeyCode::Enter =>{
                        app.calculate_next_word();
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