#![allow(non_snake_case)]

use std::hash::{Hash, Hasher};

use itertools::Itertools;
use rand::Rng;

use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;

#[derive(Debug, Clone)]
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
    (0..N)
        .map(|i| Node {
            id: i,
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
        })
        .collect()
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
            _ => unreachable!(".windows should guarantee slices of 2 always"),
        }
    }
    // if vector has at least 1 element, add distance from last to first node
    if let Some(first) = nodes.iter().nth(0) {
        if let Some(last) = nodes.iter().last() {
            length += node_distance(last, first);
        }
    }
    length
}

/// considers every possible unique permutation (n-1)!
///
/// (some permutations are the same, e.g. [0, 1, 2] = [1, 2, 0])
/// can be optimized by keeping the first node the same
/// and probably checking uniqueness upfront somehow
/// not the point tho, just getting my feet wet here
fn tsp_brute_force(nodes: &Vec<Node>) -> Vec<f64> {
    let mut optimization_hc: Vec<f64> = Vec::new();
    let mut optimal_length = f64::INFINITY;
    // loop over all possible unique tours
    for tour in nodes.iter().permutations(nodes.len()).unique() {
        let new_length = get_tour_length(&tour);
        if new_length < optimal_length {
            optimal_length = new_length;
            optimization_hc.push(optimal_length);
        }
    }
    optimization_hc
}

/// start at first node and always choose closest next node
fn tsp_nearest_neighbor(nodes: &Vec<Node>) -> Vec<Node> {
    let mut nearest_neighbor: Vec<Node> = Vec::new();
    let mut leftovers: Vec<Node> = nodes.clone();

    while !leftovers.is_empty() {
        if nearest_neighbor.is_empty() {
            // leftovers isn't empty per loop cond
            if let Some(first) = leftovers.pop() {
                nearest_neighbor.push(first);
            }
        } else {
            // nearest_neighbor isn't empty per if cond above
            if let Some(last_neighbor) = nearest_neighbor.last() {
                let mut smallest_distance: f64 = f64::INFINITY;
                let mut nn_position: usize = 0;
                for (i, node) in leftovers.iter().enumerate() {
                    let new_distance = node_distance(node, last_neighbor);
                    if new_distance < smallest_distance {
                        smallest_distance = new_distance;
                        nn_position = i;
                    }
                }

                nearest_neighbor.push(leftovers.swap_remove(nn_position));
            }
        }
    }

    nearest_neighbor
}

/// swap two random nodes, returning the swapped indices
/// indices are never equal
fn random_swap(nodes: &mut Vec<Node>) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0..nodes.len());
    let b = loop {
        let random = rng.gen_range(0..nodes.len());
        if random != a {
            break random;
        }
    };

    nodes.swap(a, b);
    (a, b)
}

/// searches for best tour by randomly swapping Nodes,
/// accepting swaps with shorter tours.
/// swaps that beget longer tours are accepted based on a
/// probability function that decreases over time
fn tsp_simulated_annealing(nodes: &Vec<Node>) -> Vec<Node> {
    const ITERATIONS: u32 = 10000;
    const START_TEMP: f64 = 3.0;
    const COOLING_FACTOR: f64 = 0.88;

    let mut history: Vec<f64> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut annealed = nodes.clone();
    let mut temp = START_TEMP;
    let mut current_length = get_tour_length(&annealed.iter().collect::<Vec<_>>());

    for iteration in 0..ITERATIONS {
        history.push(current_length);
        let (a, b) = random_swap(&mut annealed);
        let new_length = get_tour_length(&annealed.iter().collect::<Vec<_>>());
        let delta = new_length - current_length;

        // probability to accept swap
        let probability = if delta > 0.0 {
            f64::exp(-(delta / temp))
        } else {
            1.0
        };

        // debugging
        // println!("length: {:.7}, temp: {:.7}, delta: {:.7} prob: {:.7}", current_length, temp, delta, probability);

        // swap back if longer + failed probability test
        if rng.gen::<f64>() > probability {
            annealed.swap(a, b);
        } else {
            current_length = new_length;
        }

        // cooling
        temp = COOLING_FACTOR * temp;

        // add to history
    }

    history.push(current_length);

    annealed
}

/// draw tour with plotters to filename
fn draw_tour(filename: &str, nodes: &Vec<Node>) -> Result<(), Box<dyn std::error::Error>> {
    if nodes.is_empty() {
        return Err("can't draw empty tour".into());
    }
    let root = BitMapBackend::new(filename, (1111, 1111)).into_drawing_area();
    let root = root.titled(
        &format!(
            "'{}', tour length: {}",
            filename,
            get_tour_length(&nodes.iter().collect::<Vec<_>>())
        ),
        TextStyle::from(("sans-serif", 24).into_font()).color(&WHITE),
    )?;

    root.fill(&RGBColor(245, 245, 245))?;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(
        0f64..1f64,
        0f64..1f64,
        (0..1000, 0..1000),
    ));

    let dot_and_id = |node: &Node| {
        return EmptyElement::at((node.x, node.y))
            + Circle::new((0, 0), 7, ShapeStyle::from(&BLACK).filled())
            + Text::new(
                format!("{}", node.id),
                (13, 0),
                ("sans-serif", 23.0).into_font(),
            );
    };

    //
    // draw nodes
    //
    for node in nodes {
        root.draw(&dot_and_id(node))?;
    }

    //
    // draw edges
    //
    let mut edge_points = nodes
        .iter()
        .map(|n| (n.x, n.y))
        .collect::<Vec<(f64, f64)>>();
    // edge_points is just transformed nodes, which can't be empty
    edge_points.insert(0, *edge_points.last().unwrap());
    root.draw(&PathElement::new(
        edge_points,
        ShapeStyle::from(&BLACK).filled(),
    ))?;

    root.present()?;
    Ok(())
}

fn main() {
    let N = 13;
    let nodes = random_nodes(N);
    println!(
        "random tour length: {:?}",
        get_tour_length(&nodes.iter().collect::<Vec<_>>())
    );

    let nn_tour = tsp_nearest_neighbor(&nodes);
    println!(
        "nearest neighbor length: {:?}",
        get_tour_length(&nn_tour.iter().collect::<Vec<_>>())
    );

    let sa_tour = tsp_simulated_annealing(&nodes);
    println!(
        "sa length: {:?}",
        get_tour_length(&sa_tour.iter().collect::<Vec<_>>())
    );


    if let Err(err) = draw_tour("random.png", &nodes) {
        println!("Error drawing:\n{}", err);
    }
    if let Err(err) = draw_tour("nn.png", &nn_tour) {
        println!("Error drawing:\n{}", err);
    }
    if let Err(err) = draw_tour("sa.png", &sa_tour) {
        println!("Error drawing:\n{}", err);
    }
}
