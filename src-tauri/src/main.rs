// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod cell_manager;

use cell_manager::{cell::Cell, compute_next_generation};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compute_next_gen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn compute_next_gen(cells: Vec<Cell>) -> Vec<Cell> {
    return compute_next_generation(cells);
}
