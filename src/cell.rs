#[derive(Clone)]
pub struct Cell {
    state: CellState,
}

impl Cell {
    pub fn new() -> Self {
        Cell{state: CellState::EMPTY}
    }

    pub fn set_state(&mut self, state: &CellState) {
        self.state = state.clone();
    }

    pub fn is_empty(&self) -> bool {
        self.state == CellState::EMPTY
    }

    pub fn to_string(&self) -> String {
        self.state.to_string()
    }
}

#[derive(Clone, PartialEq)]
pub enum CellState {
    EMPTY,
    WHITE,
    BLACK,
}

impl CellState {
    pub fn to_string(&self) -> String {
        match self {
            CellState::EMPTY => " ".to_string(),
            CellState::WHITE => "o".to_string(),
            CellState::BLACK => "x".to_string(),
        }
    }

    pub fn have_same_state(&self, other: &Cell) -> bool {
        self == &other.state
    }

    pub fn have_another_color(&self, other: &Cell) -> bool {
        !&other.is_empty() && self != &other.state
    }
}
