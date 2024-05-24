use crate::junction::Junction;
use std::{sync::Arc, thread, time::Duration};

#[derive(Debug)]
pub struct Bus {
    pub id: usize,
    pub direction: char,
    pub neighbour: char,
}

impl Bus {
    pub fn new(id: usize, direction: char) -> Self {
        let neighbour = match direction {
            'N' => 'W',
            'S' => 'E',
            'E' => 'N',
            'W' => 'S',
            _ => unreachable!(),
        };

        Self {
            id,
            direction,
            neighbour,
        }
    }

    pub fn start(&self, junction: &Arc<Junction>) -> thread::JoinHandle<()> {
        let Bus {
            id,
            direction,
            neighbour,
        } = *self;

        let junction = Arc::clone(&junction);

        thread::spawn(move || {
            let bus_node = junction.access_graph().start_bus(id);
            println!("Bus {}: {} bus started", id, direction);

            // Lock the direction mutex
            let dir_req_edge = junction.access_graph().request(bus_node, direction);
            println!("Bus {}: Waiting for {} direction lock", id, direction);
            let _direction_lock = junction.access_direction(direction);
            println!("Bus {}: Acquired {} direction lock", id, direction);
            junction
                .access_graph()
                .receive(bus_node, dir_req_edge, direction);

            // Lock the neighbour mutex
            let neighbour_req_edge = junction.access_graph().request(bus_node, neighbour);
            println!("Bus {}: Waiting for {} direction lock", id, neighbour);
            let _neighbour_lock = junction.access_direction(neighbour);
            println!("Bus {}: Acquired {} direction lock", id, neighbour);
            junction
                .access_graph()
                .receive(bus_node, neighbour_req_edge, direction);

            // Lock the junction mutex
            println!("Bus {}: Waiting for junction lock", id);
            let _junction_lock = junction.access_junction();
            println!("Bus {}: Acquired junction lock", id);
            println!("Bus {}: Passing junction", id);
            thread::sleep(Duration::from_secs(2));

            // Remove the node
            junction.access_graph().remove_bus(bus_node);
        })
    }
}
