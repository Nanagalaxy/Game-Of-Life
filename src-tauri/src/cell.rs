use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use uuid::Uuid;

#[derive(Debug)]
pub struct Cell {
    pub id: Uuid,
    alive: Mutex<bool>,
    x: usize,
    y: usize,
    neighbors: Mutex<HashMap<(usize, usize), Weak<Cell>>>,
}

impl Cell {
    pub fn new(alive: bool, x: usize, y: usize) -> Arc<Self> {
        Arc::new(Self {
            id: Uuid::new_v4(),
            alive: Mutex::new(alive),
            x,
            y,
            neighbors: Mutex::new(HashMap::new()),
        })
    }

    pub fn set_alive(&self, alive: bool) {
        let mut alive_ref = self.alive.lock().unwrap();

        *alive_ref = alive;
    }

    pub fn add_neighbor(&self, neighbor_x: usize, neighbor_y: usize, neighbor: Arc<Cell>) {
        let mut neighbors = self.neighbors.lock().unwrap();

        neighbors.insert((neighbor_x, neighbor_y), Arc::downgrade(&neighbor));
    }

    pub fn get_neighbors(&self) -> Vec<Arc<Cell>> {
        let neighbors = self.neighbors.lock().unwrap();

        neighbors
            .iter()
            .filter_map(|(_, weak_neighbor)| weak_neighbor.upgrade()) // Upgrade the weak reference to a strong reference
            .collect()
    }

    pub fn count_alive_neighbors(&self) -> usize {
        let neighbors = self.neighbors.lock().unwrap();

        neighbors
            .iter()
            .filter_map(|(_, weak_neighbor)| weak_neighbor.upgrade()) // Upgrade the weak reference to a strong reference
            .filter(|neighbor| *neighbor.alive.lock().unwrap()) // Filter out the neighbors that are not alive
            .count()
    }

    pub fn offset_position(position: usize, offset: isize) -> Option<usize> {
        if offset < 0 {
            position.checked_sub(offset.abs() as usize)
        } else {
            position.checked_add(offset as usize)
        }
    }
}
