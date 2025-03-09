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
    // Update app state first
    app.update();
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
        .title("Top 3 Suggested Words");

    let right_bottom = Block::default()
        .borders(Borders::ALL)
        .title("Right Bottom");

    // Create text for top 3 words
    let words_text = if app.is_solving {
        vec![Line::from(Span::styled(
            "Calculating next words...",
            Style::default().fg(Color::Yellow),
        ))]
    } else if app.next_possible_words.is_empty() {
        vec![Line::from(Span::styled(
            "No suggestions available",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        app.next_possible_words
            .iter()
            .take(3)
            .enumerate()
            .map(|(i, word)| {
                Line::from(Span::styled(
                    format!("{}. {}", i + 1, word),
                    Style::default().fg(Color::Green),
                ))
            })
            .collect::<Vec<Line>>()
    };

    let right_upper_paragraph = Paragraph::new(words_text)
        .block(right_upper)
        .alignment(Alignment::Left);

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
                "(q) to quit | (L/R) change column | (U) change color | (Enter) confirm colors",
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
    // Create inner area with top padding
    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),  // Reduced top padding from 8 to 2
            Constraint::Percentage(100),  // Use remaining height for content area
        ])
        .split(main_chunks[0])[1];  // Get the content area after top padding

    let inner_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(3),  // Left padding
            Constraint::Min(1),     // Content area
            Constraint::Length(3),  // Right padding
        ])
        .split(inner_area)[1];  // Get the middle section

    // Create rows as before
    let rows = app.tiles_grid.tiles.iter().enumerate().map(|(row_idx, row)| {
        let cells = row.iter().enumerate().map(|(col_idx, tile)| {
            let is_selected = app.selected_tile == (row_idx, col_idx);
            let mut style = Style::default().fg(tile.color.to_color());
            if is_selected {
                style = style.bg(Color::White);
            }
            let cell_content = format!("{:^5}", tile.character.to_string());
            Cell::from(Span::styled(cell_content, style))
        })
        .collect::<Vec<_>>();

        Row::new(cells).height(4)
    }).collect::<Vec<_>>();

    // Make columns fill the available space evenly
    let widths = vec![Constraint::Percentage(20); app.tiles_grid.tiles[0].len()];

    let table = Table::new(rows, widths)
        .block(Block::default())
        .style(Style::default());

    // Render the table without using stateful widget
    frame.render_widget(table, inner_area);
}

pub fn plot_graph(){
    
}
