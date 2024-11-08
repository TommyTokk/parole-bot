use ratatui::{style::Color, widgets::{ListState, TableState}};

#[derive(Copy, Clone)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

#[derive(Copy, Clone)]
pub enum CurrentlyEditing {
    TileColor,
}

pub struct TilesGrid {
    pub tiles: Vec<Vec<String>>,
    pub selected_tile: Option<(usize, usize)>,
}

pub struct App {
    pub words: Vec<String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub current_char: Option<char>,
    pub current_row: usize,
    pub current_col: usize,
    pub selected_tile: Option<(usize, usize)>,
    pub tiles_grid: TilesGrid,
    pub tile_colors: Vec<Color>,
    pub tile_list_state: ListState,
    pub tile_table_state: TableState,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            current_char: None,
            current_row: 0,
            current_col: 0,
            selected_tile: Some((0, 0)),
            tiles_grid: TilesGrid {
                tiles: vec![vec![" ".to_string(); 5]; 6],
                selected_tile: Some((0, 0)),
            },
            tile_colors: vec![
                Color::Green,
                Color::Yellow,
                Color::Gray,
                Color::White,
            ],
            tile_list_state: ListState::default(),
            tile_table_state: TableState::default(),
            words: vec![
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if let Some((row, col)) = self.selected_tile {
            self.tiles_grid.tiles[row][col] = c.to_string();
            if col < 4 {
                self.selected_tile = Some((row, col + 1));
            }
        }
    }

    pub fn remove_char(&mut self) {
        if let Some((row, col)) = self.selected_tile {
            self.tiles_grid.tiles[row][col] = " ".to_string();
            if col > 0 {
                self.selected_tile = Some((row, col - 1));
            } else if row > 0 {
                self.selected_tile = Some((row - 1, 4));
            }
        }
    }

    pub fn go_next_row(&mut self) {
        if let Some((row, col)) = self.selected_tile {
            if col == 4 && row < 5 {
                self.selected_tile = Some((row + 1, 0));
            }
        }
    }
}