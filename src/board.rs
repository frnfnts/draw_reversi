use crate::cell::{Cell, CellState};
use std::collections::HashSet;


#[derive(Clone)]
pub struct Board {
    size: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            size: size,
            cells: vec![vec![Cell::new(); size]; size],
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn init(&mut self) {
        let middle = (self.size - 1) / 2;
        self.cells[middle][middle].set_state(&CellState::WHITE);
        self.cells[middle+1][middle].set_state(&CellState::BLACK);
        self.cells[middle][middle+1].set_state(&CellState::BLACK);
        self.cells[middle+1][middle+1].set_state(&CellState::WHITE);
    }

    pub fn can_put(&self, r: usize, c: usize, new_state: &CellState) -> bool {
         self.cells[r][c].is_empty() && self.have_opposite_side(r, c, new_state)
    }

    pub fn put(&mut self, r: usize, c: usize, color: &CellState) {
        if ! self.cells[r][c].is_empty() {
            panic!("stone already exists. [{}, {}]", r, c);
        }
        self.cells[r][c].set_state(color);
        self.flip(r, c, color);
    }

    pub fn all_cells(&self) -> Cells {
        Cells {
            count: 0,
            size: self.get_size(),
            cells: &self.cells,
        }
    }

    pub fn to_string(&self) -> String {
        let mut result: Vec<String> = vec![];
        result.push(" |".to_string() + &(0..self.size).map(|col_num| (1+col_num).to_string()).collect::<Vec<String>>().join("-"));
        for r in 0..self.size {
            let row_string: Vec<String> = self.cells[r].iter().map(|cell| cell.to_string()).collect();
            result.push((r+1).to_string() + "|" + &row_string.join("|"));
            result.push("-".repeat(self.size * 2 + 1));
        }
        result.join("|\n") + "|"
    }

    pub fn to_string_with_marks<T: IntoIterator<Item=(usize, usize)>>(&self, mark_positions: T) -> String {
        let mut result: Vec<String> = vec![];
        let mark_pos_set: HashSet<(usize, usize)> = mark_positions.into_iter().map(|(r, c)| (r-1, c-1)).collect();
        result.push(" |".to_string() + &(0..self.size).map(|col_num| (1+col_num).to_string()).collect::<Vec<String>>().join("-"));
        for r in 0..self.size {
            let mut row_string: Vec<String> = vec![];
            for c in 0..self.size {
                row_string.push(
                    if mark_pos_set.contains(&(r, c)) {
                        "@".to_string()
                    } else {
                        self.cells[r][c].to_string()
                    }
                )
            }
            result.push((r+1).to_string() + "|" + &row_string.join("|"));
            result.push("-".repeat(self.size * 2 + 1));
        }
        result.join("|\n") + "|"
    }

    pub fn ended(&self) -> bool {
        ! self.all_cells().any(|cell| cell.is_empty())
    }

    pub fn count_stones(&self, color: &CellState) -> usize {
        self.all_cells().filter(|&cell| color.have_same_state(cell)).fold(0, |sum, _| sum + 1)
    }

    fn have_opposite_side(&self, r: usize, c: usize, color: &CellState) -> bool {
        for diff_pair in EIGHT_WAYS.iter() {
            let mut next_r = r as i32 + diff_pair.0;
            let mut next_c = c as i32 + diff_pair.1;
            let mut have_another_color_inside = false;
            while self.is_inside(next_r, next_c) && color.have_another_color(&self.cells[next_r as usize][next_c as usize]) {
                next_r += diff_pair.0;
                next_c += diff_pair.1;
                have_another_color_inside = true;
            }
            if self.is_inside(next_r, next_c) && color.have_same_state(&self.cells[next_r as usize][next_c as usize]) && have_another_color_inside {
                return true;
            }
        }
        return false;
    }

    fn flip(&mut self, r: usize, c: usize, color: &CellState) {
        for direction in EIGHT_WAYS.iter() {
            if let Some((r_end, c_end)) = self.find_opposite(r, c, direction, color) {
                let mut ri = r as i32;
                let mut ci = c as i32;
                while ri as usize != r_end || ci as usize != c_end {
                    ri += direction.0;
                    ci += direction.1;
                    self.cells[ri as usize][ci as usize].set_state(color);
                }
            }
        }
    }

    fn find_opposite(&self, r: usize, c: usize, direction: &(i32, i32), color: &CellState) -> Option<(usize, usize)> {
        let mut next_r = r as i32 + direction.0;
        let mut next_c = c as i32 + direction.1;
        let mut have_another_color_inside = false;
        while self.is_inside(next_r, next_c) && color.have_another_color(&self.cells[next_r as usize][next_c as usize]) {
            next_r += direction.0;
            next_c += direction.1;
            have_another_color_inside = true;
        }
        if self.is_inside(next_r, next_c) && color.have_same_state(&self.cells[next_r as usize][next_c as usize]) && have_another_color_inside {
            return Some((next_r as usize, next_c as usize));
        }
        return None;
    }

    fn is_inside(&self, r: i32, c:i32) -> bool {
        0 <= r && r < self.size as i32 &&
        0 <= c && c < self.size as i32
    }
}

pub struct Cells<'a> {
    count: usize,
    size: usize,
    cells: &'a Vec<Vec<Cell>>,
}

impl<'a> Iterator for Cells<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.size * self.size { return None; }
        let (r, c) = (self.count / self.size, self.count % self.size);
        self.count += 1;
        return Some(&self.cells[r][c]);
    }
}

const EIGHT_WAYS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
