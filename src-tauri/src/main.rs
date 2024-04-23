// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod cell;
mod cell_manager;

use board::Board;
use cell_manager::{cell::Cell, compute_next_generation};
use std::sync::Arc;
use tauri::State;

fn main() {
    let board = Arc::new(Board::new());

    let time = std::time::Instant::now();
    board.create_board(1000, 1000);
    println!("Time to create board: {:?}", time.elapsed());

    tauri::Builder::default()
        .manage(board)
        .invoke_handler(tauri::generate_handler![compute_next_gen, test_new_board])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn compute_next_gen(cells: Vec<Cell>) -> Vec<Cell> {
    return compute_next_generation(cells);
}

#[tauri::command]
fn test_new_board(state: State<Arc<Board>>) {
    let board = state.inner();

    println!("{:#?}", board);
}
