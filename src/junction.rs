use std::sync::{Mutex, MutexGuard};

pub struct Junction {
    north: Mutex<()>,
    south: Mutex<()>,
    east: Mutex<()>,
    west: Mutex<()>,
    cross: Mutex<()>,
}

impl Junction {
    pub fn new() -> Self {
        Self {
            north: Mutex::new(()),
            south: Mutex::new(()),
            east: Mutex::new(()),
            west: Mutex::new(()),
            cross: Mutex::new(()),
        }
    }
    pub fn access_direction(&self, direction: char) -> MutexGuard<()> {
        match direction {
            'N' => self.north.lock().unwrap(),
            'S' => self.south.lock().unwrap(),
            'E' => self.east.lock().unwrap(),
            'W' => self.west.lock().unwrap(),
            _ => unreachable!(),
        }
    }

    pub fn access_junction(&self) -> MutexGuard<()> {
        self.cross.lock().unwrap()
    }

    pub fn access_neighbour(&self, direction: char) -> MutexGuard<()> {
        let neighbour = match direction {
            'N' => 'W',
            'S' => 'E',
            'E' => 'N',
            'W' => 'S',
            _ => unreachable!(),
        };
        self.access_direction(neighbour)
    }
}
