use crossterm::queue;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Layout generale: titolo, corpo principale, e footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Titolo
            Constraint::Min(1),     // Corpo principale
            Constraint::Length(3),  // Footer
        ])
        .split(frame.area());

    // Titolo
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Parole bot",
        Style::default().fg(Color::Yellow),
    ))
    .block(title_block)
    .alignment(Alignment::Center);

    frame.render_widget(title, chunks[0]);

    // Layout principale: sinistra (griglia) e destra (altre informazioni)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let right_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    // Blocchi sinistri e destri
    let left_block = Block::default()
        .borders(Borders::ALL)
        .title("Inserimento Parole");

    frame.render_widget(left_block, main_chunks[0]);

    // Definiamo le dimensioni della griglia e i margini per centrarla
    let grid_width = 5 * 7;  // 5 colonne con 7 unità di larghezza ciascuna
    let grid_height = 6 * 3; // 6 righe con 3 unità di altezza ciascuna

    let left_block_area = main_chunks[0];
    let horizontal_margin = (left_block_area.width.saturating_sub(grid_width)) / 2;
    let vertical_margin = (left_block_area.height.saturating_sub(grid_height)) / 2;

    let grid_area = Rect {
        x: left_block_area.x + horizontal_margin,
        y: left_block_area.y + vertical_margin,
        width: grid_width as u16,
        height: grid_height as u16,
    };

    // Layout della griglia 5x6 all'interno del blocco sinistro centrato
    let grid_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints((0..6).map(|_| Constraint::Length(3)).collect::<Vec<_>>())
        .split(grid_area);

    for (row_idx, row_chunk) in grid_chunks.iter().enumerate() {
        let row_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints((0..5).map(|_| Constraint::Length(7)).collect::<Vec<_>>())  // 7 è la larghezza di ogni cella
            .split(*row_chunk);

        for (col_idx, cell_chunk) in row_layout.iter().enumerate() {
            let cell_content = &app.tiles_grid.tiles[row_idx][col_idx];
            let cell_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White));

            let cell_paragraph = Paragraph::new(Span::styled(
                cell_content.clone(),
                Style::default().fg(Color::LightBlue),
            ))
            .block(cell_block)
            .alignment(Alignment::Center);  // Centra il testo all'interno del quadrato

            frame.render_widget(cell_paragraph, *cell_chunk);
        }
    }

    // Blocchi destri
    let right_upper = Block::default()
        .borders(Borders::ALL)
        .title("Right Upper");

    let right_bottom = Block::default()
        .borders(Borders::ALL)
        .title("Right Bottom");

    let right_upper_paragraph = Paragraph::new(Text::styled(
        "Right Upper",
        Style::default().fg(Color::White),
    ))
    .block(right_upper)
    .alignment(Alignment::Center);

    let right_bottom_paragraph = Paragraph::new(Text::styled(
        "Right Bottom",
        Style::default().fg(Color::White),
    ))
    .block(right_bottom)
    .alignment(Alignment::Center);

    frame.render_widget(right_upper_paragraph, right_chunk[0]);
    frame.render_widget(right_bottom_paragraph, right_chunk[1]);

    // Footer con modalità e hint
    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::TileColor => {
                        Span::styled("Editing Tile Color", Style::default().fg(Color::Yellow))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(Space) to toggle color | (q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}
