use petgraph::algo::is_cyclic_directed;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::stable_graph::StableDiGraph;
use std::error::Error;
use std::fmt::{self};
use std::sync::Mutex;

#[derive(Debug)]
pub struct DeadlockError;

impl fmt::Display for DeadlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deadlock detected: The graph contains a cycle")
    }
}

impl Error for DeadlockError {}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Node {
    Lock(String),
    Bus(usize),
}

// Keep track of the 4 direction lock indexes
struct Locks {
    north: NodeIndex,
    south: NodeIndex,
    east: NodeIndex,
    west: NodeIndex,
}

pub struct Graph {
    graph: Mutex<StableDiGraph<Node, usize>>,
    locks: Mutex<Locks>,
}

impl Graph {
    pub fn new() -> Self {
        let mut graph = StableDiGraph::new();
        let north = graph.add_node(Node::Lock("North Lock".to_string()));
        let south = graph.add_node(Node::Lock("South Lock".to_string()));
        let east = graph.add_node(Node::Lock("East Lock".to_string()));
        let west = graph.add_node(Node::Lock("West Lock".to_string()));

        let graph_instance = Self {
            graph: Mutex::new(graph),
            locks: Mutex::new(Locks {
                north,
                south,
                east,
                west,
            }),
        };
        graph_instance
    }

    fn get_lock_from_direction(&self, direction: char) -> NodeIndex {
        let locks = self.locks.lock().unwrap();

        match direction.to_ascii_uppercase() {
            'N' => locks.north,
            'S' => locks.south,
            'E' => locks.east,
            'W' => locks.west,
            _ => unreachable!(),
        }
    }

    pub fn start_bus(&self, id: usize) -> NodeIndex {
        let node = Node::Bus(id);
        // Add it, not pointing anywhere
        self.add_node(node)
    }

    pub fn remove_bus(&self, node: NodeIndex) {
        let mut graph = self.graph.lock().unwrap();
        graph.remove_node(node);
    }

    pub fn request(&self, bus_node: NodeIndex, direction: char) -> EdgeIndex {
        // Create edge from bus to direction lock for request
        let direction_lock = self.get_lock_from_direction(direction);
        self.update_edge(&bus_node, &direction_lock, 1)
    }

    pub fn receive(
        &self,
        bus_node: NodeIndex,
        request_edge: EdgeIndex,
        direction: char,
    ) -> EdgeIndex {
        // Remove the request edge, create ownership edge from lock to bus
        let direction_lock = self.get_lock_from_direction(direction);
        self.remove_edge(request_edge);

        self.update_edge(&direction_lock, &bus_node, 2)
    }

    pub fn release(&self, edge: EdgeIndex) {
        // Release the node edge
        self.remove_edge(edge);
    }

    fn add_node(&self, node: Node) -> NodeIndex {
        // Add node to graph
        let mut graph = self.graph.lock().unwrap();
        let index = graph.add_node(node);

        index
    }

    fn update_edge(&self, from: &NodeIndex, to: &NodeIndex, weight: usize) -> EdgeIndex {
        // Adds edge or updates edge weight
        let mut graph = self.graph.lock().unwrap();

        graph.update_edge(*from, *to, weight)
    }

    fn remove_edge(&self, edge: EdgeIndex) {
        // Removes edge from graph
        let mut graph = self.graph.lock().unwrap();
        graph.remove_edge(edge);
    }

    pub fn deadlock(&self) -> Result<(), DeadlockError> {
        // Check if graph has cycle and therefore deadlock
        let graph = self.graph.lock().unwrap();
        if is_cyclic_directed(&*graph) {
            return Err(DeadlockError);
        } else {
            Ok(())
        }
    }

    pub fn display(&self) {
        let graph = self.graph.lock().unwrap();
        println!("{:?}", Dot::with_config(&*graph, &[Config::EdgeIndexLabel]));
    }
}
