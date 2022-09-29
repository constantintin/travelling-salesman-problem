#![allow(non_snake_case)]

use rand::Rng;

#[derive(Debug)]
struct Node {
    id: usize,
    x: f64,
    y: f64,
}

fn random_nodes(N: usize) -> Vec<Node> {
    let mut rng = rand::thread_rng();
    (0..N).map(|i| Node { id: i, x: rng.gen::<f64>(), y: rng.gen::<f64>()}).collect()
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

    // let hc_history = tsp_hill_climb(&nodes);
    // let sa_history = tsp_simulated_annealing(&nodes);
}
