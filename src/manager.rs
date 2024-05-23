use rand::Rng;
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
// use petgraph::Graph;
use crate::bus::Bus;
use crate::graph::Graph;
use crate::junction::Junction;

use crate::arguments::Arguments;

pub struct Manager {
    pub probability: f32,
    pub sequence: String,
    pub junction: Arc<Junction>,
    pub graph: Arc<Graph>,
}

impl Manager {
    pub fn new(args: &Arguments) -> Self {
        // Build graph based on args
        Self {
            probability: args.probability,
            sequence: args.sequence.to_owned(),
            junction: Arc::new(Junction::new()),
            graph: Arc::new(Graph::new()),
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        // Create rng
        let mut rng = rand::thread_rng();

        // Create buses
        let mut buses = self
            .sequence
            .chars()
            .enumerate()
            .map(|(i, c)| Bus::new(i, c));

        'outer: loop {
            // Probability p of checking for deadlock
            if rng.gen_range(0.0..1.0) < self.probability {
                println!("Checking for deadlock");
                // if deadlock return error
                // self.graph.check_deadlock()?;


            } else if let Some(bus) = buses.next() {
                // Clone mutex for graph
                bus.start(&self.junction, &self.graph);
            } else {
                // Enter main loop to check for deadlock once a second
                let mut i = 0;

                loop {
                    println!("Periodically checking for deadlock...");
                    thread::sleep(Duration::from_secs(1));
                    i = i + 1;
                    // println!("i: {}", i);


                    if i == 5 {
                        // if deadlock return error
                        println!("Breaking outer");
                        break 'outer;
                    }
                }
            }

            thread::sleep(Duration::from_secs(1));
        }

        Ok(())
    }
}
