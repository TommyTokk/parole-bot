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
            TileColor::CorrectPlace => Color::Rgb((108), (169), (101)),
            TileColor::Absent => Color::Red,
            TileColor::WrongPlace => Color::Rgb((200), (182), (83)),
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
    pub listState: ListState,
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

        let mut listState = ListState::default();
        listState.select(Some(0));

        App {
            calculating_receiver: None,
            is_solving: false,
            tiles_grid,
            selected_tile: (0, 0),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            table_state,
            next_possible_words: Vec::new(),
            listState,
            solver: Solver::new(),
        }
    }

    pub fn insert_char(&mut self, c: char) {
        // Ottieni le coordinate della cella selezionata
        let (row, col) = self.selected_tile;

        // Inserisci il carattere nella cella selezionata
        if let Some(tile) = self.tiles_grid.tiles.get_mut(row).and_then(|r| r.get_mut(col)) {
            tile.character = c;
        } else {
            // Ritorna immediatamente se la posizione è invalida
            return;
        }

        // Passa alla cella successiva a destra, se non è l'ultima colonna
        if col < 4 {
            self.update_selected_tile(row, col + 1);
            // Aggiorna lo stato della colonna nel TableState
            self.table_state.select(Some(row * 5 + col + 1));
        } 
    }
    
    
    pub fn update_selected_tile(&mut self, nrow: usize, ncol: usize) {
        // Controlla che la posizione sia valida prima di aggiornare la selezione
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
        // Ottieni le coordinate della cella selezionata
        let (row, col) = self.selected_tile;
        
        // Rimuovi il carattere dalla cella selezionata
        if let Some(tile) = self.tiles_grid.tiles.get_mut(row).and_then(|r| r.get_mut(col)) {
            tile.character = ' ';
        }
        
        // Passa alla cella precedente a sinistra, se non è la prima colonna
        //if col > 0 {
        //    self.update_selected_tile(row, col - 1);
        //    // Aggiorna lo stato della colonna nel TableState
        //    self.table_state.select(Some(row * 5 + col - 1));
        //}
        
        //passa alla riga precedente se la colonna è la prima
        //else if row > 0{
        //    self.update_selected_tile(row - 1, 4);
        //    // Aggiorna lo stato della riga nel TableState
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
        // Ottieni le coordinate della cella selezionata
        let (row, col) = self.selected_tile;
        
        if self.current_screen == CurrentScreen::EditingTileChar{
            // Passa alla riga precedente, se non è la prima riga e se la colonna è la prima
            if row > 0 || col == 0 {
                self.update_selected_tile(row - 1, 4);
                // Aggiorna lo stato della riga nel TableState
                self.table_state.select(Some((row - 1) * 5 + 4));
            }
        }else{
            // Passa alla riga precedente, se non è la prima riga
            if row > 0 {
                self.update_selected_tile(row - 1, col);
                // Aggiorna lo stato della riga nel TableState
                self.table_state.select(Some((row - 1) * 5 + col));
            }
        }
    }

    pub fn go_prev_col(&mut self) {
        // Ottieni le coordinate della cella selezionata
        let (row, col) = self.selected_tile;

        // Passa alla colonna precedente, se non è la prima colonna
        if col > 0 {
            self.update_selected_tile(row, col - 1);
            // Aggiorna lo stato della colonna nel TableState
            self.table_state.select(Some(row * 5 + col - 1));
        }
    }

    pub fn go_next_col(&mut self) {
        // Ottieni le coordinate della cella selezionata
        let (row, col) = self.selected_tile;

        // Passa alla colonna successiva, se non è l'ultima colonna
        if col < 4 {
            self.update_selected_tile(row, col + 1);
            // Aggiorna lo stato della colonna nel TableState
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

    pub fn calculate_next_word(&mut self) {
        let current_row_tile: &Vec<Tile> = &self.tiles_grid.tiles[self.selected_tile.0];

        let word: String = current_row_tile.iter().map(|tile| tile.character).collect();
        let color_state = self.get_color_state(current_row_tile);

        self.next_possible_words = self.solver.get_next_possible_words(&word.to_lowercase(), &color_state);
    }

    pub fn calculate_next_possible_word(&mut self){
        let current_row_tile: &Vec<Tile> = &self.tiles_grid.tiles[self.selected_tile.0];

        let word: String = current_row_tile.iter().map(|tile| tile.character).collect();
        let color_state = self.get_color_state(current_row_tile);

        let solver = self.solver.clone();
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

    pub fn update(&mut self){
        if let Some(receiver) = &self.calculating_receiver{
            match receiver.try_recv(){
                Ok(words) => {
                    self.next_possible_words = words;
                    self.calculating_receiver = None;
                    self.is_solving = false;
                },
                Err(mpsc::TryRecvError::Empty) => {},
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.calculating_receiver = None;
                    self.is_solving = false;
                }
            }
        }
    }
    
}
