mod field;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write, Result};
use crate::field::{Cell, Dog, Entity, Field, Sheep};

fn to_key(key: (Sheep, Dog)) -> String{
    serde_json::to_string(&key).unwrap_or("".to_string())
}

fn from_serialized(json: String) -> (Sheep, Dog) {
    serde_json::from_str(&json).unwrap_or((Sheep::new(), Dog::new()))
}

// t-star is a hashmap that has states as keys and stores floats as values
// output should be a float

// make first value distance to the top + 2
// t_star(hashmap) -> hashmap
// calculate the expected moves for a given field
pub fn t_star(old_utility_map: HashMap<(Sheep, Dog), f32>) -> HashMap<(Sheep, Dog), f32> {
    let mut new_utility_map = HashMap::new();
    let beta = 0.99;

    for ((sheep, dog), _value) in &old_utility_map {
        if new_utility_map.contains_key(&(*sheep, *dog)) {
            continue;
        }
        let state = Field::with(*sheep, *dog);
        let possible_states = state.get_dog_states();

        let mut minimum = f32::MAX;
        let sheep_pos = (sheep.x, sheep.y);
        let dog_pos = (dog.x, dog.y);
        let goal_pos = (state.grid.len() as i32 / 2, state.grid.len() as i32 / 2);

        if sheep_pos == dog_pos {
            new_utility_map.insert((*sheep, *dog), f32::MAX);
            continue;
        } else if sheep_pos == goal_pos {
            new_utility_map.insert((*sheep, *dog), 0.0);
            continue;
        }

        for possible_state in possible_states {
            let sheep_states = possible_state.get_sheep_states();
            let number_states = sheep_states.len() as i32;
            let mut summation = 0.0;
            for sheep_state in sheep_states {

                let sheep_pos = (sheep_state.sheep.x, sheep_state.sheep.y);
                let dog_pos = (sheep_state.dog.x, sheep_state.dog.y);
                let goal_pos = (state.grid.len() as i32 / 2, state.grid.len() as i32 / 2);

                if sheep_pos == dog_pos {
                    new_utility_map.insert((sheep_state.sheep, sheep_state.dog), f32::MAX);
                    continue;
                } else if sheep_pos == goal_pos {
                    continue;
                }
                summation = summation + (1.0 as f32 / number_states as f32) * old_utility_map.get(&(sheep_state.sheep, sheep_state.dog)).unwrap();
            }
            if summation < minimum {
                minimum = summation;
            }
        }
        new_utility_map.insert((*sheep, *dog), 1.0 + beta * minimum);
    }

    new_utility_map
}

fn generate_optimal_utlility() -> HashMap<(Sheep, Dog), f32> {
    let mut utility_map = HashMap::new();

    let test_field = Field::new();

    for sheep_cell in test_field
        .grid
        .clone()
        .into_iter()
        .flatten()
        .collect::<Vec<Cell>>()
    {
        let mut new_sheep_cell = sheep_cell.clone();
        new_sheep_cell.entity = Entity::Sheep;
        let intermediate_state = test_field.move_sheep_to(new_sheep_cell);
        for dog_cell in intermediate_state
            .grid
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<Cell>>()
        {
            let mut new_dog_cell = dog_cell.clone();
            new_dog_cell.entity = Entity::Dog;
            let final_state = intermediate_state.move_dog_to(new_dog_cell);
            let sheep_pos = (final_state.sheep.x, final_state.sheep.y);
            let dog_pos = (final_state.dog.x, final_state.dog.y);
            let goal_pos = (
                final_state.grid.len() as i32 / 2,
                final_state.grid.len() as i32 / 2,
            );
            if goal_pos == sheep_pos {
                utility_map.insert((final_state.sheep, final_state.dog), 0.0);
            }

            if dog_pos == sheep_pos {
                utility_map.insert((final_state.sheep, final_state.dog), f32::MAX);
            }
            let sheep_goal_distance: f32 = (sheep_pos.0 - goal_pos.0).abs() as f32
                + (sheep_pos.1 - goal_pos.1).abs() as f32
                + 2.0;
            let initial_score =  sheep_goal_distance;
            utility_map.insert((final_state.sheep, final_state.dog), initial_score);
        }
    }

    println!("{:?}", utility_map.len());

    loop {
        let updated_map = t_star(utility_map.clone());

        // checking the expexted moves left from corner to corner
        let sheep = Sheep::at(0, 0);
        let dog = Dog::at(24, 24);
        let old_value = utility_map.get(&(sheep, dog)).unwrap();
        let new_value = updated_map.get(&(sheep, dog)).unwrap();
        println!("{} -> {}", old_value, new_value);
        if update_size(&utility_map, &updated_map) < 0.01 {
            return updated_map;
        }
        utility_map = updated_map;
    }
    
    utility_map
}

fn save_utility_map<P: AsRef<Path>>(path: P, map: HashMap<(Sheep, Dog), f32>) -> Result<()> {
    let mut new_map = HashMap::new();
    for (key, value) in &map {
        new_map.insert(to_key(*key), value);
    }
    let mut f = File::create(path)?;
    println!("created the file");
    match serde_json::to_vec(&map) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    let buf = serde_json::to_vec(&new_map)?;
    println!("serialized the data structure");
    f.write_all(&buf[..])?;
    println!("wrote to the file");
    Ok(())
}

fn update_size(old_map: & HashMap<(Sheep, Dog), f32>, new_map: & HashMap<(Sheep, Dog), f32>) -> f32 {
    let mut summation: f32 = 0.0;
    for (key, value) in old_map {
        let new_val = new_map.get(&key).unwrap();
        let difference = value - new_val;
        summation = summation + difference.abs();
    }
     2.0 * summation / (1.0 - 0.99)
}

fn inner_load_utility_map<P: AsRef<Path>>(path: P) -> Option<HashMap<String, f32>> {
    if let Ok(mut file) = File::open(path) {
        let mut buf = vec![];
        if file.read_to_end(&mut buf).is_ok() {
            if let Ok(map ) = serde_json::from_slice(&buf[..]) {
                return Some(map);
            }
        }
    }
    None
}

fn load_utility_map<P: AsRef<Path>>(path: P) -> HashMap<(Sheep, Dog), f32> {
    match inner_load_utility_map(path) {
        Some(map) => {
            let mut new_map = HashMap::new();
            for (key, value) in &map {
                new_map.insert(from_serialized(key.clone()), *value);
            }
            return new_map;
        },
        None => return generate_optimal_utlility(),
    }
}

fn find_best_starting_location(map: & HashMap<(Sheep, Dog), f32>) -> Field {
    let test_field = Field::new();
    let mut lowest_score = f32::MAX;
    let mut best_field = test_field.clone();
    for dog_cell in test_field
        .grid
        .clone()
        .into_iter()
        .flatten()
        .collect::<Vec<Cell>>()
    {
        let mut new_dog_cell = dog_cell.clone();
        new_dog_cell.entity = Entity::Dog;
        let intermediate_state = test_field.move_dog_to(new_dog_cell);
        let new_score = map.get(&(intermediate_state.sheep, intermediate_state.dog)).unwrap();
        if *new_score < lowest_score {
            lowest_score = *new_score;
            best_field = intermediate_state;
        }
    }
    best_field
}

fn run_simulation(map: & HashMap<(Sheep, Dog), f32>) -> f32 {
    let mut game = find_best_starting_location(&map);
    let expexted_moves = map.get(&(game.sheep, game.dog)).unwrap();
    let mut actual_moves = 0.0;
    while !game.won() {
        let possible_states = game.get_dog_states();
        let mut best_state = possible_states[0].clone();
        let mut best_value = map.get(&(best_state.sheep, best_state.dog)).unwrap();
        for possible_state in possible_states {
            let test_value =  map.get(&(possible_state.sheep, possible_state.dog)).unwrap();
            if test_value < best_value {
                best_state = possible_state;
                best_value = test_value;
            }
        }
        game = best_state;
        actual_moves = actual_moves + 1.0;
        game.move_sheep();
        game.print();
        println!();
    }

    expexted_moves - actual_moves
} 

fn main() {
    let map = load_utility_map("cached_utility_map");
    
    let best_start = find_best_starting_location(&map);
    best_start.print();

    let difference = run_simulation(&map);
    println!("{}", difference);

    // let result = save_utility_map("cached_utility_map", map);
    // match result {
    //     Err(_) => println!("couldn't save the file"),
    //     Ok(_) => println!("saved successfully"),
    // }
}
