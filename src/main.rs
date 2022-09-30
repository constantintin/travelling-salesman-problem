#![allow(non_snake_case)]

use std::hash::{Hash, Hasher};

use rand::Rng;
use itertools::Itertools;

#[derive(Debug)]
struct Node {
    id: usize,
    x: f64,
    y: f64,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Node {}

fn random_nodes(N: usize) -> Vec<Node> {
    let mut rng = rand::thread_rng();
    (0..N).map(|i| Node { id: i, x: rng.gen::<f64>(), y: rng.gen::<f64>()}).collect()
}

/// euclidian distance between 2 nodes
fn node_distance(node1: &Node, node2: &Node) -> f64 {
    ((node2.x - node1.x).powi(2) + (node2.y - node1.y).powi(2)).sqrt()
}

/// traverses pairs of nodes in order and sums the distances
fn get_tour_length(nodes: &[&Node]) -> f64 {
    let mut length: f64 = 0.0;
    for window_slice in nodes.windows(2) {
        match window_slice {
            [n1, n2] => length += node_distance(n1, n2),
            _ => unreachable!(".windows should guarantee slices of 2 always")
        }
    }
    length
}

// fn tsp_hill_climb(nodes: &Vec<Node>) -> Vec<f64> {
//     todo!();
// }

// fn tsp_simulated_annealing(nodes: &Vec<Node>) -> Vec<f64> {
//     todo!();
// }

fn main() {
    let N = 10;
    let nodes = random_nodes(N);

    println!("{:?}", nodes)
    println!("tour length: {:?}", get_tour_length(&nodes.iter().collect::<Vec<_>>()));

    // let hc_history = tsp_hill_climb(&nodes);
    // let sa_history = tsp_simulated_annealing(&nodes);
}
