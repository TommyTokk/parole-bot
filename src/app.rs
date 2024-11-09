
use ratatui::{style::Color, widgets::{ListState, TableState}};

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
    pub selected_tile: Tile,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub list_state: ListState,
    pub table_state: TableState,

}

impl App {
    pub fn new() -> App {
        let tiles_grid = TilesGrid {
            tiles: vec![vec![
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (0, 0),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (0, 1),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (0, 2),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (0, 3),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (0, 4),
                },
            ],
            vec![
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (1, 0),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (1, 1),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (1, 2),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (1, 3),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (1, 4),
                },
            ],
            vec![
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (2, 0),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (2, 1),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (2, 2),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (2, 3),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (2, 4),
                },
            ],
            vec![
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (3, 0),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (3, 1),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (3, 2),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (3, 3),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (3, 4),
                },
            ],
            vec![
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (4, 0),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (4, 1),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (4, 2),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (4, 3),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (4, 4),
                },
            ],
            vec![
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (5, 0),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (5, 1),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (5, 2),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (5, 3),
                },
                Tile {
                    character: ' ',
                    color: TileColor::Normal,
                    selected: false,
                    position: (5, 4),
                },
            ]],
        };
        App {
            tiles_grid,
            selected_tile: Tile {
                character: ' ',
                color: TileColor::Normal,
                selected: false,
                position: (0, 0),
            },
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            list_state: ListState::default(),
            table_state: TableState::default(),
        }
    }

    pub fn insert_char(&mut self, c: char) {
        let (row, col) = self.selected_tile.position;
        self.tiles_grid.tiles[row][col].character = c;
        
        if col < 4 {
            self.selected_tile.position = (row, col + 1);
        }

    }

    pub fn remove_char(&mut self) {
        let (row, col) = self.selected_tile.position;
        self.tiles_grid.tiles[row][col].character = ' ';
        if col > 0 {
            self.selected_tile.position = (row, col - 1);
        } else if row > 0 {
            self.selected_tile.position = (row - 1, 4);
        }
    }

    pub fn go_next_row(&mut self) {
        let (row, col) = self.selected_tile.position;
        if col == 4 && row < 5 {
            self.selected_tile.position = (row + 1, 0);
        }
    }

    pub fn go_next_color(&mut self) {
        todo!()
    }

    pub fn go_prev_color(&mut self) {
        todo!()
    }

}