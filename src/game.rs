use crate::board::{Board};
use crate::cell::{CellState};

#[derive(Clone)]
pub struct Game {
    board: Board,
    next_player: CellState
}

impl Game {
    pub fn new() -> Game {
        let mut g = Game {
            board: Board::new(8),
            next_player: CellState::WHITE,
        };
        g.board.init();
        return g;
    }

    pub fn can_put(&self, r: usize, c: usize) -> bool {
        self.board.can_put(r-1, c-1, &self.next_player)
    }

    pub fn put(&mut self, r: usize, c: usize) {
        if !self.board.can_put(r-1, c-1, &self.next_player) {
            panic!("you can not put at [{}, {}]", r, c);
        }
        self.board.put(r-1, c-1, &self.next_player);

        self.next_turn();
        if self.availabe_cells().is_empty() {
            self.next_turn();
        }
    }

    pub fn ended(&self) -> bool {
        self.board.ended()
    }

    pub fn is_draw(&self) -> bool {
        self.ended() && self.board.count_stones(&CellState::WHITE) == self.board.count_stones(&CellState::BLACK)
    }

    pub fn print_board(&self) {
        println!("{}", self.board.to_string());
        println!("next player {}", self.next_player.to_string());
        println!("#o: {} #x: {}", self.board.count_stones(&CellState::WHITE), self.board.count_stones(&CellState::BLACK));
    }

    pub fn print_board_with_marks(&self, mark_positions: Vec<(usize, usize)>) {
        println!("{}", self.board.to_string_with_marks(mark_positions));
        println!("next player {}", self.next_player.to_string());
        println!("#o: {} #x: {}", self.board.count_stones(&CellState::WHITE), self.board.count_stones(&CellState::BLACK));
    }

    pub fn availabe_cells(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for r in 1..(self.board.get_size()+1) {
            for c in 1..(self.board.get_size()+1) {
                if self.can_put(r, c) { result.push((r, c)); }
            }
        }
        result
    }

    fn next_turn(&mut self) {
        self.next_player = match self.next_player {
            CellState::EMPTY => CellState::EMPTY,
            CellState::WHITE => CellState::BLACK,
            CellState::BLACK => CellState::WHITE,
        }
    }

}
