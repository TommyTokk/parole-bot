use std::cell;

use crossterm::queue;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
use crate::app::{self, App, CurrentScreen, CurrentlyEditing, TileColor};

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

    render_grid(&main_chunks, app, frame);

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
            CurrentScreen::EditingTileChar => {
                Span::styled("Editing tile char Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::EditingTileColor => {
                Span::styled("Editing tile color Mode", Style::default().fg(Color::Yellow))
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
                    CurrentlyEditing::TileChar => {
                        Span::styled("Editing Tile Char", Style::default().fg(Color::Yellow))
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
            CurrentScreen::EditingTileChar => Span::styled(
                "(q) quit | (U/D) change row | (L/R) change column | (Enter) next row | (Esc) exit editing",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::EditingTileColor => Span::styled(
                "(q) to quit | (L/R) change column | (U) change color",
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

pub fn render_grid(main_chunks: &[Rect], app: &mut App, frame: &mut Frame) {
    // Creazione delle righe della tabella
    let rows = app.tiles_grid.tiles.iter().enumerate().map(|(row_idx, row)| {
        let cells = row.iter().enumerate().map(|(col_idx, tile)| {
            // Verifica se questa è la cella selezionata
            let is_selected = app.selected_tile == (row_idx, col_idx);
            
            // Imposta lo stile della cella in base al colore e alla selezione
            let mut style = Style::default().fg(tile.color.to_color());
            if is_selected {
                style = style.bg(Color::White);  // Colore speciale per la cella selezionata
            }

            let cell_content = format!("{:^3}", tile.character.to_string());

            // Crea la cella con il carattere e lo stile
            Cell::from(Span::styled(cell_content, style))})
            .collect::<Vec<_>>();  // Assicurati di avere un `Vec<Cell>`


        Row::new(cells).height(3)
    }).collect::<Vec<_>>();  // Assicurati di avere un `Vec<Row>`

    // Define the widths for the table columns
    let widths = vec![Constraint::Length(7); app.tiles_grid.tiles[0].len()];

    // Calculate the available space for the table within the left chunk
    let left_chunk = main_chunks[0];
    

    //divide left chunk in 3 parts with 30%, 40% and 30% width
    let table_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(left_chunk);

    //divide the table layout vertically in 3 parts with 10%, 80% and 10% height
    let table_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(table_layout[1]);

    // Configure the table with rows and other properties
    let table = Table::new(rows, widths)
        .block(Block::default().borders(Borders::ALL))  // Every column is wide 7 units
        .highlight_style(Style::default());  // Highlighting style

    // Render the table within the centered layout
    frame.render_stateful_widget(table, table_layout[1], &mut app.table_state);
}
