pub mod cell {
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct Cell {
        pub id: Uuid,

        pub alive: bool,

        pub x: u32,

        pub y: u32,
    }

    impl Cell {
        pub fn new(alive: bool, x: u32, y: u32) -> Self {
            Self {
                id: Uuid::new_v4(),
                alive,
                x,
                y,
            }
        }

        /// Toggle the alive state of the cell
        ///
        /// # Examples
        /// ```rust
        /// use cell_manager::cell::Cell;
        ///
        /// let mut cell = Cell::new(true, 0, 0);
        ///
        /// cell.toggle_alive();
        ///
        /// assert_eq!(cell.alive, false);
        ///
        /// cell.toggle_alive();
        ///
        /// assert_eq!(cell.alive, true);
        /// ```
        pub fn toggle_alive(&mut self) {
            self.alive = !self.alive;
        }

        /// Set the alive state of the cell
        ///
        /// # Examples
        /// ```rust
        /// use cell_manager::cell::Cell;
        ///
        /// let mut cell = Cell::new(true, 0, 0);
        ///
        /// cell.set_alive(false);
        ///
        /// assert_eq!(cell.alive, false);
        /// ```
        pub fn set_alive(&mut self, alive: bool) {
            self.alive = alive;
        }

        /// Find a cell by its ID
        pub fn find_by_id<'a>(cells_map: &'a HashMap<Uuid, &Cell>, id: Uuid) -> Option<&'a Cell> {
            cells_map.get(&id).copied()
        }

        /// Find a cell by its position
        pub fn find_by_position<'a>(
            cells_map: &'a HashMap<(u32, u32), &Cell>,
            x: u32,
            y: u32,
        ) -> Option<&'a Cell> {
            cells_map.get(&(x, y)).copied()
        }

        /// Offset the position by the given offset
        pub fn offset_position(position: u32, offset: i8) -> Option<u32> {
            if offset < 0 {
                position.checked_sub(offset.abs() as u32)
            } else {
                position.checked_add(offset as u32)
            }
        }

        /// Get the neighbors of the cell
        pub fn neighbors<'a>(&'a self, cells_map: &'a HashMap<(u32, u32), &Cell>) -> Vec<&Cell> {
            let mut neighbors = Vec::new();

            for (dx, dy) in NeighborsPosition::get_all() {
                let offset_x = if let Some(offset_x) = Self::offset_position(self.x, dx) {
                    offset_x
                } else {
                    continue;
                };
                let offset_y = if let Some(offset_y) = Self::offset_position(self.y, dy) {
                    offset_y
                } else {
                    continue;
                };

                if let Some(neighbor) = Self::find_by_position(cells_map, offset_x, offset_y) {
                    neighbors.push(neighbor);
                }
            }

            neighbors
        }

        /// Get the alive neighbors of the cell
        pub fn alive_neighbors<'a>(&'a self, neighbors: &'a Vec<&Cell>) -> Vec<&Cell> {
            neighbors
                .par_iter()
                .cloned()
                .filter(|cell| cell.alive)
                .collect()
        }

        /// Get the dead neighbors of the cell
        pub fn dead_neighbors<'a>(&'a self, neighbors: &'a Vec<&Cell>) -> Vec<&Cell> {
            neighbors
                .par_iter()
                .cloned()
                .filter(|cell| !cell.alive)
                .collect()
        }
    }

    struct NeighborsPosition;

    impl NeighborsPosition {
        const NORTH_WEST: (i8, i8) = (-1, -1);
        const NORTH: (i8, i8) = (0, -1);
        const NORTH_EAST: (i8, i8) = (1, -1);
        const WEST: (i8, i8) = (-1, 0);
        const EAST: (i8, i8) = (1, 0);
        const SOUTH_WEST: (i8, i8) = (-1, 1);
        const SOUTH: (i8, i8) = (0, 1);
        const SOUTH_EAST: (i8, i8) = (1, 1);

        fn get_all() -> Vec<(i8, i8)> {
            vec![
                Self::NORTH_WEST,
                Self::NORTH,
                Self::NORTH_EAST,
                Self::WEST,
                Self::EAST,
                Self::SOUTH_WEST,
                Self::SOUTH,
                Self::SOUTH_EAST,
            ]
        }
    }
}

use self::cell::Cell;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

/// Get the relevant cells for the next generation, i.e. the cells that are alive and their neighbors
fn get_relevant_cells(cells: Vec<Cell>) -> Vec<Cell> {
    // Create a HashMap for fast lookup
    let cells_map: HashMap<(u32, u32), &Cell> = cells
        .par_iter()
        .map(|cell| ((cell.x, cell.y), cell))
        .collect();

    let mut relevant_cells: Vec<Cell> = Vec::new();

    for cell in cells_map.values() {
        if cell.alive {
            relevant_cells.push(**cell);

            for neighbor in cell.neighbors(&cells_map) {
                if !relevant_cells.contains(neighbor) {
                    relevant_cells.push(*neighbor);
                }
            }
        }
    }

    relevant_cells
}

fn cell_future_state(cell: &Cell, neighbors: &Vec<&Cell>) -> bool {
    let alive_neighbors = cell.alive_neighbors(neighbors);

    if cell.alive {
        match alive_neighbors.len() {
            2 | 3 => true,
            _ => false,
        }
    } else {
        match alive_neighbors.len() {
            3 => true,
            _ => false,
        }
    }
}

pub fn compute_next_generation(all_cells: Vec<Cell>) -> Vec<Cell> {
    // Get the relevant cells for the next generation
    let relevant_cells = get_relevant_cells(all_cells);

    let relevant_cells_map: HashMap<(u32, u32), &Cell> = relevant_cells
        .par_iter()
        .map(|cell| ((cell.x, cell.y), cell))
        .collect();

    let next_generation: Vec<Cell> = relevant_cells
        .par_iter()
        .map(|cell| {
            let neighbors = cell.neighbors(&relevant_cells_map);
            let alive = cell_future_state(&cell, &neighbors);

            if alive != cell.alive {
                Cell {
                    alive,
                    ..cell.clone()
                }
            } else {
                cell.clone()
            }
        })
        .collect();

    next_generation
}
