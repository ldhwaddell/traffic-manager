use crate::arguments::Arguments;
use crate::bus::Bus;
use crate::junction::Junction;
use rand::Rng;
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Manager {
    pub probability: f32,
    pub sequence: String,
    pub junction: Arc<Junction>,
}

impl Manager {
    pub fn new(args: &Arguments) -> Self {
        // Build graph based on args
        Self {
            probability: args.probability,
            sequence: args.sequence.to_owned(),
            junction: Arc::new(Junction::new()),
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

        // Keep track of thread handles
        let mut thread_handles = Vec::new();

        'outer: loop {
            // Probability p of checking for deadlock
            if rng.gen_range(0.0..1.0) < self.probability {
                println!("Checking for deadlock");
                self.junction.access_graph().deadlock()?;
            } else if let Some(bus) = buses.next() {
                // Clone mutex for graph
                let handle = bus.start(&self.junction);
                thread_handles.push(handle);
            } else {
                // Enter main loop to check for deadlock once a second
                loop {
                    println!("Periodically checking for deadlock...");
                    self.junction.access_graph().deadlock()?;

                    // Check if all threads have finished
                    if thread_handles.iter().all(|handle| handle.is_finished()) {
                        break 'outer;
                    }

                    thread::sleep(Duration::from_secs(1));
                }
            }
            // Sleep a second to limit thread races
            thread::sleep(Duration::from_secs(1));
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }

        println!("Buses finished with no deadlock detected.");

        Ok(())
    }
}
