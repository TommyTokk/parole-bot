use ratatui::{style::Color, widgets::TableState};

#[derive(Copy, Clone)]
pub enum CurrentScreen {
    Main,
    EditingTileChar,
    EditingTileColor,
    Exiting,
}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum CurrentlyEditing {
    TileChar,
    TileColor,
}

#[derive(PartialEq)]
pub enum TileColor {
    Selected,
    CorrectPlace,
    Absent,
    WrongPlace,
    Normal,
}

impl TileColor {
    pub fn to_color(&self) -> Color {
        match self {
            TileColor::Selected => Color::LightBlue,
            TileColor::CorrectPlace => Color::Green,
            TileColor::Absent => Color::Gray,
            TileColor::WrongPlace => Color::Yellow,
            TileColor::Normal => Color::White,
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
    pub tiles_grid: TilesGrid,
    pub selected_tile: (usize, usize),
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub table_state: TableState,
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
        table_state.select(Some(0)); // Seleziona la prima cella per impostazione predefinita

        App {
            tiles_grid,
            selected_tile: (0, 0),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            table_state,
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
    
    pub fn go_next_row(&mut self) {
        // Ottieni le coordinate della cella selezionata
        let (row, col) = self.selected_tile;

        // Passa alla riga successiva, se non è l'ultima riga e se la colonna è l'ultima
        if row < 5 && col == 4 {
            self.update_selected_tile(row + 1, 0);
            // Aggiorna lo stato della riga nel TableState
            self.table_state.select(Some((row + 1) * 5));
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
        todo!()
    }


    pub fn go_next_color(&mut self) {
        todo!()
    }

    pub fn go_prev_color(&mut self) {
        todo!()
    }
}
