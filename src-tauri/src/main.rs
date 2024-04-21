// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod cell_manager;

use cell_manager::{cell::Cell, compute_next_generation};
use uuid::Uuid;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            display_cells,
            display_cell,
            compute_next_gen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn display_cells(cells: Vec<Cell>) {
    println!("Cells: {:#?}", cells);

    let cell = Cell::find_by_position(cells.clone(), 0, 0);

    if let Some(cell) = cell {
        println!("Cell: {:#?}", cell);

        let neighbors = cell.neighbors(cells.clone());

        println!("Neighbors: {:#?}", neighbors);
    } else {
        println!("Cell not found!");
    }
}

#[tauri::command]
fn display_cell(cell: Cell) {
    println!("Cell: {:#?}", cell);
    println!("Cell ID: {:#?}", cell.id);
    println!("Cell Alive: {:#?}", cell.alive);
    println!("Cell X: {:#?}", cell.x);
    println!("Cell Y: {:#?}", cell.y);

    println!(
        "Parsed Cell Id: {:#?}",
        Uuid::parse_str(&cell.id.to_string()).unwrap()
    );
}

#[tauri::command]
fn compute_next_gen(cells: Vec<Cell>) -> Vec<Cell> {
    return compute_next_generation(cells);
}
