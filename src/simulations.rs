

use rand::Rng;
use crate::field::Entity;
use crate::field::Cell;
use crate::field::Field;
use crate::field::Dog;
use crate::field::Sheep;
use crate::make_distance_map_sheep;
use crate::math::dot_product;
use crate::math::model_1;
use crate::math::model_2;
use std::collections::HashMap;

pub(crate) fn find_best_starting_location(map: &HashMap<(Sheep, Dog), f32>) -> Field {
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
        let mut score = 0.0;
        for sheep_cell in intermediate_state
            .grid
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<Cell>>()
        {
            let mut new_sheep_cell = sheep_cell.clone();
            new_sheep_cell.entity = Entity::Sheep;
            let final_state = intermediate_state.move_sheep_to(new_sheep_cell);
            if !final_state.is_valid() {
                continue;
            }
            let subscore = map.get(&(final_state.sheep, final_state.dog)).unwrap();
            score = score + subscore;
        }
        if score < lowest_score && intermediate_state.is_valid() {
            lowest_score = score;
            best_field = intermediate_state;
        }
    }
    best_field
}

pub(crate) fn run_simulation(map: &HashMap<(Sheep, Dog), f32>) -> f32 {
    let mut rng = rand::thread_rng();
    let dog = Dog::at(15, 12);
    let sheep = Sheep::at(rng.gen_range(0..31), rng.gen_range(0..31));
    let mut game = Field::with(sheep, dog);
    while !game.is_valid() {
        let sheep = Sheep::at(rng.gen_range(0..31), rng.gen_range(0..31));
        game = Field::with(sheep, dog);
    }
    let expexted_moves = map.get(&(game.sheep, game.dog)).unwrap();
    let mut actual_moves = 0.0;
    game.print();
    while !game.dog_won() && game.is_valid() && !game.sheep_won() {
        // for _ in 0..5 {
        let possible_states = game.get_dog_states();
        let mut best_state = possible_states[0].clone();
        let mut best_value: &f32 = &10000.0;
        for possible_state in possible_states {
            let mut reaction_state = possible_state.clone();
            reaction_state.move_sheep();
            let test_value = map
                .get(&(reaction_state.sheep, reaction_state.dog))
                .unwrap();
            if test_value < best_value {
                best_state = reaction_state;
                best_value = test_value;
            }
        }
        game = best_state;
        actual_moves = actual_moves + 1.0;
    }
    actual_moves - expexted_moves
}

pub(crate) fn run_simulation_with_model(
  model: (f32, f32, f32, f32, f32),
  map: &HashMap<(Sheep, Dog), f32>,
  distance_map_sheep: &HashMap<Sheep, f32>,
) -> (f32, bool) {
  let mut rng = rand::thread_rng();
  let dog = Dog::at(15, 12);
  let sheep = Sheep::at(rng.gen_range(0..31), rng.gen_range(0..31));
  let mut game = Field::with(sheep, dog);
  while !game.is_valid() {
      let sheep = Sheep::at(rng.gen_range(0..31), rng.gen_range(0..31));
      game = Field::with(sheep, dog);
  }
  let expexted_moves = map.get(&(game.sheep, game.dog)).unwrap();
  let mut actual_moves = 0.0;
  while !game.dog_won() && game.is_valid() && !game.sheep_won() {
      // for _ in 0..5 {
      let possible_states = game.get_dog_states();
      let mut best_state = possible_states[0].clone();
      let mut best_value: f32 = 10000.0;
      for possible_state in possible_states {
          let mut reaction_state = possible_state.clone();
          reaction_state.move_sheep();
          let data_point = (reaction_state.sheep, reaction_state.dog);
          let data_vector = model_2(data_point, &distance_map_sheep);
          let test_value = dot_product(data_vector, model);
          if test_value < best_value {
              best_state = reaction_state;
              best_value = test_value;
          }
      }
      game = best_state;
      actual_moves = actual_moves + 1.0;
      if actual_moves > 250.0 {
        return (251.0, false);
      }
  }
  game.print();
  (actual_moves - expexted_moves, game.dog_won())
}