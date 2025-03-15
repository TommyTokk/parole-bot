use ratatui::{style::Color, widgets::{ListState, TableState}};
use std::sync::mpsc::{self, Receiver};


use crate::solver::Solver;

#[derive(Copy, Clone, PartialEq)]
pub enum CurrentScreen {
    Main,
    EditingTileChar,
    EditingTileColor,
    Exiting,
}



#[derive(PartialEq, Clone, Copy)]
pub enum CurrentlyEditing {
    TileChar,
    TileColor,
}

#[derive(PartialEq, Clone)]
pub enum TileColor {
    CorrectPlace,
    Absent,
    WrongPlace,
    Normal,
}

impl TileColor {
    pub fn to_color(&self) -> Color {
        match self {
            TileColor::CorrectPlace => Color::Rgb(108, 169, 101),
            TileColor::Absent => Color::Red,
            TileColor::WrongPlace => Color::Rgb(200, 182, 83),
            TileColor::Normal => Color::Gray,
        }
    }
}

#[derive(PartialEq)]
pub struct Tile {
    pub character: char,
    pub color: TileColor,
    pub selected: bool,
    pub position: (usize, usize),
}

pub struct TilesGrid {
    pub tiles: Vec<Vec<Tile>>,
}

pub struct App {
    pub calculating_receiver: Option<Receiver<Vec<String>>>,
    pub is_solving:bool,
    pub tiles_grid: TilesGrid,
    pub selected_tile: (usize, usize),
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub table_state: TableState,
    pub next_possible_words: Vec<String>,
    pub list_state: ListState,
    pub solver: Solver,
}

impl App {
    pub fn new() -> App {
        let tiles_grid = TilesGrid {
            tiles: (0..6).map(|row| {
                (0..5).map(|col| {
                    Tile {
                        character: ' ',
                        color: TileColor::Normal,
                        selected: false,
                        position: (row, col),
                    }
                }).collect()
            }).collect(),
        };
        
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        App {
            calculating_receiver: None,
            is_solving: false,
            tiles_grid,
            selected_tile: (0, 0),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            table_state,
            next_possible_words: Vec::new(),
            list_state,
            solver: Solver::new(),
        }
    }

    pub fn insert_char(&mut self, c: char) {
        // Get the coordinates of the selected cell
        let (row, col) = self.selected_tile;

        // Insert the character in the selected cell
        if let Some(tile) = self.tiles_grid.tiles.get_mut(row).and_then(|r| r.get_mut(col)) {
            tile.character = c;
        } else {
            // Return immediately if the position is invalid
            return;
        }

        // Move to the next cell to the right, if not the last column
        if col < 4 {
            self.update_selected_tile(row, col + 1);
            // Update column state in TableState
            self.table_state.select(Some(row * 5 + col + 1));
        } 
    }
    
    
    pub fn update_selected_tile(&mut self, nrow: usize, ncol: usize) {
        // Check that the position is valid before updating the selection
        if nrow < self.tiles_grid.tiles.len() && ncol < self.tiles_grid.tiles[nrow].len() {
            // Deselect the current tile
            let (row, col) = self.selected_tile;
            if let Some(tile) = self.tiles_grid.tiles.get_mut(row).and_then(|r| r.get_mut(col)) {
                tile.selected = false;
            }
            
            // Set the new selected position
            self.selected_tile = (nrow, ncol);
            if let Some(new_tile) = self.tiles_grid.tiles.get_mut(nrow).and_then(|r| r.get_mut(ncol)) {
                new_tile.selected = true;
            }
        }
    }
    
    pub fn remove_char(&mut self) {
        // Get the coordinates of the selected cell
        let (row, col) = self.selected_tile;
        
        // Remove the character from the selected cell
        if let Some(tile) = self.tiles_grid.tiles.get_mut(row).and_then(|r| r.get_mut(col)) {
            tile.character = ' ';
        }
        
        // Go to the previous cell if not in the first row
        if col > 0 {
            self.update_selected_tile(row, col - 1);
           // Update the tablestate
           self.table_state.select(Some(row * 5 + col - 1));
        }
        
        // Move to the previous row if this is the first column
        //else if row > 0{
        //    self.update_selected_tile(row - 1, 4);
        //    // Update row state in TableState
        //    self.table_state.select(Some((row - 1) * 5 + 4));
        //}
        
    }
    
    pub fn go_next_row(&mut self) {
        let (row, col) = self.selected_tile;
        // Check if we're at the last column
        if col == 4 {
            // Move to the first tile of the next row if not at the last row
            if row < self.tiles_grid.tiles.len() - 1 {
                self.update_selected_tile(row + 1, 0);
                // Update table state
                self.table_state.select(Some((row + 1) * 5));
            }
        }
        // Do nothing if not at the last column
    }

    pub fn go_prev_row(&mut self) {
        // Get the coordinates of the selected cell
        let (row, col) = self.selected_tile;
        
        if self.current_screen == CurrentScreen::EditingTileChar{
            // Move to the previous row if not the first row and if column is the first
            if row > 0 || col == 0 {
                self.update_selected_tile(row - 1, 4);
                // Update row state in TableState
                self.table_state.select(Some((row - 1) * 5 + 4));
            }
        }else{
            // Move to the previous row if not the first row
            if row > 0 {
                self.update_selected_tile(row - 1, col);
                // Update row state in TableState
                self.table_state.select(Some((row - 1) * 5 + col));
            }
        }
    }

    pub fn go_prev_col(&mut self) {
        // Get the coordinates of the selected cell
        let (row, col) = self.selected_tile;

        // Move to the previous column if not the first column
        if col > 0 {
            self.update_selected_tile(row, col - 1);
            // Update column state in TableState
            self.table_state.select(Some(row * 5 + col - 1));
        }
    }

    pub fn go_next_col(&mut self) {
        // Get the coordinates of the selected cell
        let (row, col) = self.selected_tile;

        // Move to the next column if not the last column
        if col < 4 {
            self.update_selected_tile(row, col + 1);
            // Update column state in TableState
            self.table_state.select(Some(row * 5 + col + 1));
        }
    }

    pub fn go_next_color(&mut self){
        let (row, col) = self.selected_tile;
        if let Some(tile) = self.tiles_grid.tiles.get_mut(row).and_then(|r| r.get_mut(col)) {
            match tile.color {
                TileColor::Normal => tile.color = TileColor::CorrectPlace,
                TileColor::CorrectPlace => tile.color = TileColor::Absent,
                TileColor::Absent => tile.color = TileColor::WrongPlace,
                TileColor::WrongPlace => tile.color = TileColor::Normal,
            }
        }
    }

    pub fn go_prev_color(&mut self){
        let (row, col) = self.selected_tile;
        if let Some(tile) = self.tiles_grid.tiles.get_mut(row).and_then(|r| r.get_mut(col)) {
            match tile.color {
                TileColor::Normal => tile.color = TileColor::WrongPlace,
                TileColor::CorrectPlace => tile.color = TileColor::Normal,
                TileColor::Absent => tile.color = TileColor::CorrectPlace,
                TileColor::WrongPlace => tile.color = TileColor::Absent,
            }
        }
    }

    
    // pub fn calculate_next_word(&mut self) {
    //     let current_row_tile = &self.tiles_grid.tiles[self.selected_tile.0];
    //     let word: String = current_row_tile.iter().map(|tile| tile.character).collect();
    //     let color_state = self.get_color_state(current_row_tile);
        
    //     // First, add this word to used words
    //     self.solver.add_used_word(&word.to_lowercase(), &color_state);
        
    //     // Then get the next possible words
    //     self.next_possible_words = self.solver.get_next_possible_words(&word.to_lowercase(), &color_state);
    // }

    pub fn calculate_next_possible_word(&mut self){
        let current_row_tile: &Vec<Tile> = &self.tiles_grid.tiles[self.selected_tile.0];

        let word: String = current_row_tile.iter().map(|tile| tile.character).collect();
        let color_state = self.get_color_state(current_row_tile);
        
        self.solver.add_used_word(&word.to_lowercase(), &color_state);

        let mut solver = self.solver.clone();
        let word_clone = word.to_lowercase();
        let color_state_clone = color_state.clone();

        let (tx, rx) = mpsc::channel();
        self.calculating_receiver = Some(rx);
        self.is_solving = true;

        std::thread::spawn(move || {
            let next_possible_words = solver.get_next_possible_words(&word_clone, &color_state_clone);
            tx.send(next_possible_words).unwrap();
        });
    }

    pub fn get_color_state(&self, row: &Vec<Tile>) -> String{
        let mut color_state = String::new();
        for tile in row{
            match tile.color {
                TileColor::Normal => color_state.push('W'),
                TileColor::CorrectPlace => color_state.push('G'),
                TileColor::Absent => color_state.push('R'),
                TileColor::WrongPlace => color_state.push('Y'),
            }
        }
        color_state
    }

    pub fn update(&mut self) {
        // Check for completed calculations
        if let Some(receiver) = &self.calculating_receiver {
            match receiver.try_recv() {
                Ok(words) => {
                    // Calculation completed successfully
                    self.next_possible_words = words;
                    self.calculating_receiver = None;
                    self.is_solving = false;
                
                    
                    // Force redraw by updating a UI-related field
                    if let Some(selected) = self.list_state.selected() {
                        self.list_state.select(Some(std::cmp::min(selected, self.next_possible_words.len().saturating_sub(1))));
                    } else if !self.next_possible_words.is_empty() {
                        self.list_state.select(Some(0));
                    }
                },
                Err(mpsc::TryRecvError::Empty) => {
                    // Still calculating, do nothing
                },
                Err(mpsc::TryRecvError::Disconnected) => {
                    // Thread ended unexpectedly
                    self.calculating_receiver = None;
                    self.is_solving = false;
                }
            }
        }
    }
}
