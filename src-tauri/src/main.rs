// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod cell;

use board::Board;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

fn main() {
    let board = Arc::new(Board::new());

    tauri::Builder::default()
        .manage(board)
        .invoke_handler(tauri::generate_handler![
            create_board,
            kill_board,
            compute_next_gen,
            update_cell_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_board(
    state: State<Arc<Board>>,
    width: usize,
    height: usize,
) -> Vec<(Uuid, usize, usize)> {
    let board = state.inner();

    let time = std::time::Instant::now();

    board.create_board(width, height);

    println!(
        "Time to create board: {:?} with a size of {}x{}",
        time.elapsed(),
        width,
        height
    );

    board
        .get_cells()
        .par_iter()
        .map(|cell| (cell.id, cell.x, cell.y))
        .collect()
}

#[tauri::command]
fn kill_board(state: State<Arc<Board>>) {
    let board = state.inner();

    let time = std::time::Instant::now();
    board.kill_board();
    println!("Time to kill board: {:?}", time.elapsed());
}

#[tauri::command]
fn update_cell_state(state: State<Arc<Board>>, id: Uuid, new_state: bool) -> (Uuid, bool) {
    let board = state.inner();

    let cell = board.get_cell(id);

    let result = match cell {
        Some(cell) => {
            cell.set_alive(new_state);
            (cell.id, true)
        }
        None => (id, false),
    };

    result
}

#[tauri::command]
fn compute_next_gen(state: State<Arc<Board>>) -> Vec<(Uuid, bool)> {
    let board = state.inner();

    let time = std::time::Instant::now();

    let next_gen = board.compute_next_generation();

    board.update_next_generation(&next_gen);

    println!(
        "Time to compute next generation: {:?} for {} cells",
        time.elapsed(),
        next_gen.len()
    );

    next_gen
}
