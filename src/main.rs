mod cell;
mod board;
mod game;
use game::Game;

use std::io;

fn main() -> io::Result<()>{
    let trace = search_draw().unwrap();
    println!("Ready!");
    let mut game = Game::new();
    for (r, c) in trace.into_iter() {
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        game.print_board_with_marks(vec![(r, c)]);
        game.put(r, c);
    }
    game.print_board();
    Ok(())
}

fn search_draw() -> Option<Vec<(usize, usize)>> {
    type State = (Game, Vec<(usize, usize)>, Vec<(usize, usize)>);
    let initial_game = Game::new();
    let mut state_queue: Vec<State> = vec![(initial_game.clone(), initial_game.availabe_cells(), vec![])];
    while ! state_queue.is_empty() {
        let (game, availabe_cells, trace) = state_queue.pop().unwrap();
        if game.ended() && game.is_draw() {
            return Some(trace);
        }
        for cell in availabe_cells.into_iter() {
            let mut next_game = game.clone();
            let mut next_trace = trace.clone();
            next_game.put(cell.0, cell.1);
            next_trace.push(cell);
            state_queue.push((next_game.clone(), next_game.availabe_cells(), next_trace));
        }
    }
    return None;
}
