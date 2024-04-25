use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use uuid::Uuid;

/// A struct representing a cell
#[derive(Debug)]
pub struct Cell {
    /// The id of the cell
    pub id: Uuid,

    /// Whether the cell is alive or not
    pub alive: Mutex<bool>,

    /// The x position of the cell
    pub x: usize,

    /// The y position of the cell
    pub y: usize,

    /// The list of neighbors of the cell
    neighbors: Mutex<HashMap<(usize, usize), Weak<Cell>>>,
}

impl Cell {
    /// Create a new cell
    pub fn new(alive: bool, x: usize, y: usize) -> Arc<Self> {
        Arc::new(Self {
            id: Uuid::new_v4(),
            alive: Mutex::new(alive),
            x,
            y,
            neighbors: Mutex::new(HashMap::new()),
        })
    }

    /// Set the alive state of the cell
    /// `true` if the cell is alive, `false` otherwise
    pub fn set_alive(&self, alive: bool) {
        let mut alive_ref = self.alive.lock().unwrap();

        *alive_ref = alive;
    }

    /// Add a neighbor to the cell
    pub fn add_neighbor(&self, neighbor_x: usize, neighbor_y: usize, neighbor: Arc<Cell>) {
        let mut neighbors = self.neighbors.lock().unwrap();

        neighbors.insert((neighbor_x, neighbor_y), Arc::downgrade(&neighbor));
    }

    /// Get the list of neighbors of the cell
    pub fn get_neighbors(&self) -> Vec<Arc<Cell>> {
        let neighbors = self.neighbors.lock().unwrap();

        neighbors
            .par_iter()
            .filter_map(|(_, weak_neighbor)| weak_neighbor.upgrade()) // Upgrade the weak reference to a strong reference
            .collect()
    }

    /// Get the number of alive neighbors of the cell
    pub fn count_alive_neighbors(&self) -> usize {
        let neighbors = self.neighbors.lock().unwrap();

        neighbors
            .par_iter()
            .filter_map(|(_, weak_neighbor)| weak_neighbor.upgrade()) // Upgrade the weak reference to a strong reference
            .filter(|neighbor| *neighbor.alive.lock().unwrap()) // Filter out the neighbors that are not alive
            .count()
    }

    /// Calculates an offset position relative to the cell position and an offset.
    /// Returns `Some(usize)` if the offset position is valid, `None` otherwise.
    /// A position is valid if it does not overflow the `usize` type.
    pub fn offset_position(position: usize, offset: isize) -> Option<usize> {
        if offset < 0 {
            position.checked_sub(offset.abs() as usize)
        } else {
            position.checked_add(offset as usize)
        }
    }

    /// Compute the future state of the cell
    pub fn compute_future_state(&self) -> bool {
        let alive = *self.alive.lock().unwrap();
        let alive_neighbors = self.count_alive_neighbors();

        match (alive, alive_neighbors) {
            (true, 2) | (true, 3) => true,
            (false, 3) => true,
            _ => false,
        }
    }
}
