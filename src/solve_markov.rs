
use crate::field::Entity;
use crate::field::Cell;
use crate::field::Field;
use crate::field::Dog;
use crate::field::Sheep;

use std::collections::HashMap;

// make first value distance to the top + 2
// t_star(hashmap) -> hashmap
// calculate the expected moves for a given field
pub fn t_star(old_utility_map: HashMap<(Sheep, Dog), f32>) -> HashMap<(Sheep, Dog), f32> {
    let mut new_utility_map = old_utility_map.clone();
    let beta = 0.99;

    for ((sheep, dog), _value) in &old_utility_map {
        let test_sheep = Sheep::at(0, 0);
        let test_dog = Dog::at(1, 0);
        let state = Field::with(*sheep, *dog);
        let mut flag = false;
        if !state.is_valid() || state.sheep_won() || state.dog_won() {
            continue;
        }
        if *dog == test_dog && *sheep == test_sheep {
            flag = true;
        }
        let dog_actions = state.get_dog_states();
        let mut minimum = f32::MAX;
        for dog_action in dog_actions {
            let sheep_actions = dog_action.get_sheep_states();
            let movement_probability = 1.0 / sheep_actions.len() as f32;
            let mut summation = 0.0;
            for sheep_action in sheep_actions {
                match old_utility_map.get(&(sheep_action.sheep, sheep_action.dog)) {
                    Some(e) => (),
                    None => sheep_action.print(),
                };
                let action_reward = movement_probability
                    * old_utility_map
                        .get(&(sheep_action.sheep, sheep_action.dog))
                        .unwrap();

                summation = summation + action_reward;
            }
            if summation < minimum {
                minimum = summation;
            }
        }
        if flag {
            println!("minimum for them next to each other: {}", minimum);
        }
        new_utility_map.insert((*sheep, *dog), 1.0 + beta * minimum);
    }

    new_utility_map
}

pub(crate) fn generate_optimal_utlility() -> HashMap<(Sheep, Dog), f32> {
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
            if !final_state.is_valid() {
                continue;
            }
            let sheep_pos = (final_state.sheep.x, final_state.sheep.y);
            let goal_pos = (
                final_state.grid.len() as i32 / 2,
                final_state.grid.len() as i32 / 2,
            );
            if final_state.dog_won() {
                utility_map.insert((final_state.sheep, final_state.dog), 0.0);
                continue;
            }

            if final_state.sheep_won() {
                utility_map.insert((final_state.sheep, final_state.dog), 10000.0);
                continue;
            }
            let sheep_goal_distance: f32 = (sheep_pos.0 - goal_pos.0).abs() as f32
                + (sheep_pos.1 - goal_pos.1).abs() as f32
                + 2.0;
            let initial_score = sheep_goal_distance;
            utility_map.insert((final_state.sheep, final_state.dog), initial_score);
        }
    }

    println!("{:?}", utility_map.len());

    loop {
        let updated_map = t_star(utility_map.clone());
        let update_size = update_size(&utility_map, &updated_map);
        println!("update size: {}", update_size);
        if update_size < 0.01 {
            return updated_map;
        }
        utility_map = updated_map;
    }
}

fn update_size(old_map: &HashMap<(Sheep, Dog), f32>, new_map: &HashMap<(Sheep, Dog), f32>) -> f32 {
  let mut summation: f32 = 0.0;
  for (key, value) in old_map {
      let new_val = new_map.get(&key).unwrap();
      let difference = value - new_val;
      summation = summation + difference.abs();
  }
  2.0 * summation / (1.0 - 0.99)
}
