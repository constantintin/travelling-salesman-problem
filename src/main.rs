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

// fn tsp_hill_climb(nodes: &Vec<Node>) -> Vec<f64> {
//     todo!();
// }

// fn tsp_simulated_annealing(nodes: &Vec<Node>) -> Vec<f64> {
//     todo!();
// }

/// draw tour with plotters to filename
fn draw_tour(filename: &str, nodes: &Vec<Node>) -> Result<(), Box<dyn std::error::Error>> {
    if nodes.is_empty() {
        return Err("can't to draw empty tour".into());
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
    let N = 10;
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

    if let Err(err) = draw_tour("random.png", &nodes) {
        println!("Error drawing:\n{}", err);
    }
    if let Err(err) = draw_tour("nn.png", &nn_tour) {
        println!("Error drawing:\n{}", err);
    }
}
