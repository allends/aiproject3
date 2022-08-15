mod data;
mod field;
mod math;
mod simulations;
mod solve_markov;

#[macro_use]
extern crate queues;
use std::collections::HashMap;
use std::hash::Hash;

use crate::data::{load_partitioned_data, load_distance_data, save_distance_data};
use crate::field::{Dog, Field, Sheep};
use crate::math::bfs_sheep;
use crate::simulations::run_simulation_with_model;
use data::load_utility_map;
use math::{
    dot_product, loss, model_1, model_2, scalar_multiple, vector_subtraction, weighted_loss, bfs_dog,
};
use rand::seq::SliceRandom;
use rand::Rng;

fn stochastic_gradient_descent(
    data: (
        Vec<((Sheep, Dog), f32)>,
        Vec<((Sheep, Dog), f32)>,
        Vec<((Sheep, Dog), f32)>,
    ),
) -> (f32, f32, f32, f32, f32) {
    let distance_map_sheep = make_distance_map_sheep();
    println!("loaded the distance maps");
    let training_data = data.0.clone();
    let learning_rate = 0.0000003;
    let file_path = "stochastic_gradient_descent_loss2.csv";
    let mut wtr = match csv::Writer::from_path(file_path) {
        Ok(writer) => writer,
        Err(_) => return (0.0, 0.0, 0.0, 0.0, 0.0),
    };

    let mut rng = rand::thread_rng();
    let w0 = (
        rng.gen::<f32>() / 10.0,
        rng.gen::<f32>() / 10.0,
        rng.gen::<f32>() / 10.0,
        rng.gen::<f32>() / 10.0,
        rng.gen::<f32>() / 10.0,
    );

    let mut w_k = w0.clone();
    let unit_vector = (1.0, 1.0, 1.0, 1.0, 1.0);
    let mut loss_value = weighted_loss(&data.0, w_k);
    let mut best_vector = unit_vector;
    for iteration in 0..10000 {
        let data_point = training_data.choose(&mut rng).unwrap();
        let data_vector = model_2(data_point.0, &distance_map_sheep);
        let test_value = dot_product(data_vector, w_k);
        let difference = test_value - data_point.1;
        let difference_vector =
            scalar_multiple(scalar_multiple(data_vector, difference), learning_rate);
        let new_vector = vector_subtraction(difference_vector, w_k);

        let new_vector_loss_testing = weighted_loss(&data.1, new_vector);

        let _result = wtr.write_record(&[
            format!("{}", iteration),
            format!("{}", loss_value),
            format!("{}", new_vector_loss_testing),
        ]);

        if new_vector_loss_testing < loss_value {
            loss_value = new_vector_loss_testing;
            best_vector = new_vector;
        }
        if dot_product(unit_vector, new_vector) > 1000.0 {
            w_k = best_vector;
        } else if new_vector_loss_testing > 10.0 + loss_value {
            w_k = best_vector;
        } else {
            w_k = new_vector;
        }
    }
    let _result = wtr.write_record(&[format!("{:?}", best_vector), format!("{}", loss_value)]);
    let _result = wtr.flush();
    println!("{}", loss_value);
    best_vector
}

fn make_distance_map_sheep() -> HashMap<Sheep, f32> {
    let mut result = HashMap::new();
    for row in 0..31 {
        for column in 0..31 {
            let sheep = Sheep::at(column, row);
            let position = if row == 0 && column == 0 { 1 } else { 0 };
            let dog = Dog::at(position, position);
            result.insert(sheep, bfs_sheep((sheep, dog)));
        }
    }
    result
}

fn make_distance_map_dog() -> HashMap<(Sheep, Dog), f32> {
    let mut result = HashMap::new();
    for row in 0..31 {
        for column in 0..31 {
            let dog = Dog::at(column, row);
            for row in 0..31 {
                for column in 0..31 {
                    let sheep = Sheep::at(row, column);
                    result.insert((sheep, dog), bfs_dog((sheep, dog)));
                }
            }
        }
    }
    result
}

fn run_simulations(model: (f32, f32, f32, f32, f32), map: HashMap<(Sheep, Dog), f32>) {
    let mut average = 0.0;
    let mut games_won = 0.0;
    let mut games_expired = 0.0;
    let distance_map_sheep = make_distance_map_sheep();

    for _ in 0..1000 {
        let (difference, game_won) = run_simulation_with_model(model, &map, &distance_map_sheep);
        if game_won {
            average = average + difference;
            games_won = games_won + 1.0;
        }
        if !game_won && difference == 251.0 {
            games_expired = games_expired + 1.0;
        }
    }
    average = average / games_won;
    println!(
        "{} across {} games. {} games expired. {} lost",
        average,
        games_won,
        games_expired,
        10.0 - games_won - games_expired
    );
}

fn main() {
    let name = "cached_utlity_map_31x31";
    let data_file = "partitioned_data";
    let map = load_utility_map(name);
    // let partitioned_data = load_partitioned_data(data_file, &map);
    println!("loaded the map");

    // let best_model = stochastic_gradient_descent(partitioned_data);
    // println!("{:?}", best_model);


    // let model = stochastic_gradient_descent(partitioned_data);
    let model_latest_try = (0.5948616, 0.62768173, 0.07846236, 0.4726258, 0.0911483);
    // let best_model2 = (0.33271807, 0.8405044, 0.022575932, 0.52544063, 0.09900899);
    // let best_model = (0.66582894, 0.6932436, 0.056437638, 0.3580382, 0.04303138); // avg error 17.512537
    // let model_2_trial2 = (0.5116242, 0.68731534, 0.09846029, 0.44221365, 0.0347358);
    run_simulations(model_latest_try, map);
    // let _result = save_partitioned_data(data_file, partitioned_data);
    // let _result = save_utility_map(name, map);
}
