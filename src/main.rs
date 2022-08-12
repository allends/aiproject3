mod field;
use std::collections::HashMap;

use crate::field::{Cell, Dog, Entity, Field, Sheep};

// t-star is a hashmap that has states as keys and stores floats as values
// output should be a float

// make first value distance to the top + 2
// t_star(hashmap) -> hashmap
// calculate the expected moves for a given field
pub fn t_star(old_utility_map: HashMap<(Sheep, Dog), f32>) -> HashMap<(Sheep, Dog), f32> {
    let mut new_utility_map = old_utility_map.clone();
    let beta = 0.99;

    for ((sheep, dog), _value) in &old_utility_map {
        let state = Field::with(*sheep, *dog);
        let possible_states = state.get_dog_states();

        let mut minimum = f32::MAX;
        for possible_state in possible_states {
            let sheep_states = possible_state.get_sheep_states();
            let number_states = sheep_states.len() as i32;
            let mut summation = 0.0;
            for sheep_state in sheep_states {
                if sheep_state.sheep.x == sheep_state.grid.len() as i32 / 2
                    && sheep_state.sheep.y == sheep_state.grid.len() as i32 / 2
                {
                    summation = summation + 0.0;
                } else if sheep_state.sheep.x == sheep_state.dog.x
                    && sheep_state.sheep.y == sheep_state.dog.y
                {
                    summation = summation + (1.0 as f32 / number_states as f32) * f32::MAX;
                } else {
                    summation = summation
                        + (1.0 as f32 / number_states as f32)
                            * old_utility_map
                                .get(&(sheep_state.sheep, sheep_state.dog))
                                .unwrap();
                }
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
            let sheep_dog_distance =
                (sheep_pos.0 - dog_pos.0).abs() as f32 + (sheep_pos.1 - dog_pos.1).abs() as f32;
            let sheep_goal_distance: f32 = (sheep_pos.0 - goal_pos.0).abs() as f32
                + (sheep_pos.1 - goal_pos.1).abs() as f32
                + 2.0;
            let initial_score = sheep_dog_distance + sheep_goal_distance;
            utility_map.insert((final_state.sheep, final_state.dog), initial_score);
        }
    }

    println!("{:?}", utility_map.len());

    for _ in 0..3 {
        let updated_map = t_star(utility_map.clone());

        // checking the expexted moves left from corner to corner
        let sheep = Sheep::at(0, 0);
        let dog = Dog::at(20, 20);
        let old_value = utility_map.get(&(sheep, dog)).unwrap();
        let new_value = updated_map.get(&(sheep, dog)).unwrap();
        println!("{} -> {}", old_value, new_value);
        if old_value == new_value {
            return updated_map;
        }
        utility_map = updated_map;
    }

    utility_map
}

fn main() {
    let result = generate_optimal_utlility();
    // let states = field.get_dog_states();
    // for state in states {
    //     state.print();
    //     println!();
    // }
}
