// Artificial Intelligence for Business
// Optimizing Warehouse Flows with Q-Learning

use rand::seq::SliceRandom;
use rand::Rng;

// Setting the parameters gamma and alpha for the Q-Learning
const GAMMA: f64 = 0.75;
const ALPHA: f64 = 0.9;

// Defining the states
const LOCATION_TO_STATE: [(&str, usize); 12] = [
    ("A", 0), ("B", 1), ("C", 2), ("D", 3), ("E", 4),
    ("F", 5), ("G", 6), ("H", 7), ("I", 8), ("J", 9),
    ("K", 10), ("L", 11),
];

// Defining the rewards
const R: [[f64; 12]; 12] = [
    [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1000.0, 1.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0],
];

// PART 2 - BUILDING THE AI SOLUTION WITH Q-LEARNING
// Initialize the Q-values
fn initialize_q_values() -> [[f64; 12]; 12] {
    [[0.0; 12]; 12]
}

// Implementing the Q-Learning process
fn q_learning(q: &mut [[f64; 12]; 12]) {
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let current_state = rng.gen_range(0..12);
        let playable_actions: Vec<usize> = (0..12)
            .filter(|&j| R[current_state][j] > 0.0)
            .collect();
        let next_state = *playable_actions.choose(&mut rng).unwrap();
        let td = R[current_state][next_state] + GAMMA * q[next_state]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max) - q[current_state][next_state];
        q[current_state][next_state] += ALPHA * td;
    }
}

// Making a mapping form the states to the locations
fn state_to_location() -> std::collections::HashMap<usize, &'static str> {
    LOCATION_TO_STATE.iter().map(|&(k, v)| (v, k)).collect()
}

// Making the final function that will return the optimal route
fn route(starting_location: &str, ending_location: &str) -> Vec<String> {
    let location_to_state: std::collections::HashMap<&str, usize> = LOCATION_TO_STATE.iter().copied().collect();
    let state_to_location = state_to_location();
    let mut r_new = R.clone();
    let ending_state = location_to_state[ending_location];
    r_new[ending_state][ending_state] = 1000.0;

    let mut q = initialize_q_values();
    q_learning(&mut q);

    let mut route = vec![starting_location.to_string()];
    let mut next_location = starting_location.to_string();

    while next_location != ending_location {
        let starting_state = location_to_state[next_location.as_str()];
        let next_state = q[starting_state]
            .iter()
            .copied()
            .enumerate()
            .fold((0, f64::NEG_INFINITY), |(idx_max, max_val), (idx, val)| {
                if val > max_val {
                    (idx, val)
                } else {
                    (idx_max, max_val)
                }
            })
            .0;
        next_location = state_to_location[&next_state].to_string();
        route.push(next_location.clone());
    }

    route
}

// PART 3 - GOING INTO PRODUCTION
// Making the final function that returns the optimal route
fn best_route(starting_location: &str, intermediary_location: &str, ending_location: &str) -> Vec<String> {
    let mut first_route = route(starting_location, intermediary_location);
    let second_route = route(intermediary_location, ending_location);
    first_route.extend_from_slice(&second_route[1..]);
    first_route
}

// Printing the final route
fn main() {
    let optimal_route = route("E", "G");
    println!("Route: {:?}", optimal_route);
}
