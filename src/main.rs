use std::{error::Error, io, panic, sync::{mpsc, Arc, atomic::{AtomicBool, Ordering}}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
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
    // Setup panic hook for proper cleanup
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // First cleanup terminal
        disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
        // Then call the original panic handler
        original_hook(panic_info);
    }));

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create shared shutdown signal
    let running = Arc::new(AtomicBool::new(true));

    // Create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app, running.clone());

    // Signal threads to stop
    running.store(false, Ordering::SeqCst);

    // Cleanup terminal
    cleanup_terminal()?;
    
    if let Err(err) = res {
        println!("Error: {}", err);
    }

    Ok(())
}

fn cleanup_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>, 
    app: &mut App, 
    running: Arc<AtomicBool>
) -> io::Result<bool> {
    while running.load(Ordering::SeqCst) {
        // Draw UI - calls app.update() internally to check for completed calculations
        terminal.draw(|f| ui(f, app))?;

        // Non-blocking event check
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
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
                        KeyCode::Up => {
                            app.go_prev_row();
                        }
                        KeyCode::Down => {
                            app.go_next_row();
                        }
                        KeyCode::Left => {
                            app.go_prev_col();
                        }
                        KeyCode::Right => {
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
                        KeyCode::Left => {
                            app.go_prev_col();
                        }
                        KeyCode::Right => {
                            app.go_next_col();
                        }
                        KeyCode::Up => {
                            app.go_prev_row();
                        }
                        KeyCode::Down => {
                            app.go_next_row();
                        }
                        KeyCode::Char('n') => {
                            app.go_next_color();
                        }
                        KeyCode::Tab => {
                            app.current_screen = CurrentScreen::EditingTileChar;
                            app.currently_editing = Some(CurrentlyEditing::TileChar);
                        }
                        KeyCode::Enter => {
                            app.calculate_next_possible_word();
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None;
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => {
                        running.store(false, Ordering::SeqCst);
                        return Ok(true);
                    },
                }
            }
        }
    }

    Ok(true)
}