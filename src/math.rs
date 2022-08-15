use std::collections::HashMap;
use std::ops::RangeInclusive;

use crate::field::Dog;
use queues::*;

use crate::field::Field;
use crate::field::Sheep;

pub(crate) fn loss(data: &Vec<((Sheep, Dog), f32)>, model: (f32, f32, f32, f32, f32)) -> f32 {
    let mut total_loss = 0.0;
    for ((sheep, dog), data_point_output) in data {
        total_loss =
            total_loss + (dot_product(model, model_1((*sheep, *dog))) - data_point_output).abs();
    }
    total_loss
}

pub(crate) fn weighted_loss(
    data: &Vec<((Sheep, Dog), f32)>,
    model: (f32, f32, f32, f32, f32),
) -> f32 {
    let mut total_loss = 0.0;
    let weight = 1.0 / data.len() as f32;
    for ((sheep, dog), data_point_output) in data {
        total_loss = total_loss
            + weight * (dot_product(model, model_1((*sheep, *dog))) - data_point_output).abs();
    }
    total_loss
}

pub(crate) fn model_1(state: (Sheep, Dog)) -> (f32, f32, f32, f32, f32) {
    let sheep = state.0;
    let dog = state.1;
    let center = (15, 15);
    let sheep_to_center = ((center.0 - sheep.x).abs() + (center.1 - sheep.y).abs()) as f32;
    let sheep_to_dog = ((dog.x - sheep.x).abs() + (dog.y - sheep.y).abs()) as f32;
    let dog_to_center = ((dog.x - center.0).abs() + (dog.y - center.1).abs()) as f32;
    let dog_next_to_sheep = match ((dog.x - sheep.x).abs() + (dog.y - sheep.y).abs()) as f32 {
        1.0 => 1.0,
        _ => 0.0,
    };
    let dog_won = match ((center.0 - sheep.x).abs() + (center.1 - sheep.y).abs()) as f32 {
        0.0 => 1.0,
        _ => 0.0,
    };
    let sheep_square = sheep_to_center * sheep_to_center;
    (
        dog_to_center,
        sheep_to_center,
        dog_next_to_sheep,
        sheep_to_dog,
        dog_won,
    )
}

pub fn bfs_sheep(state: (Sheep, Dog)) -> f32 {
    let field = Field::with(state.0, state.1);
    let center = (15, 15);
    let mut queue = Queue::new();
    let mut scores = HashMap::new();
    queue.add(field.clone());
    scores.insert(field, 0.0);
    while queue.size() > 0 {
        let current = queue.remove().unwrap();
        for child in current.get_sheep_moves_bfs() {
            if child.dog_won() {
                return *scores.get(&current).unwrap() + 1.0;
            }
            let sheep_to_center_child =
                ((center.0 - child.sheep.x).abs() + (center.1 - child.sheep.y).abs()) as f32;
            let sheep_to_center =
                ((center.0 - current.sheep.x).abs() + (center.1 - current.sheep.y).abs()) as f32;
            if (sheep_to_center_child < sheep_to_center || sheep_to_center < 5.0)
                && !scores.contains_key(&child)
            {
                queue.add(child.clone());
                scores.insert(child, scores.get(&current).unwrap() + 1.0 as f32);
            }
        }
    }
    0.0
}

pub fn bfs_dog(state: (Sheep, Dog)) -> f32 {
    let field = Field::with(state.0, state.1);
    let center = (15, 15);
    let dog_to_center = ((center.0 - state.1.x).abs() + (center.1 - state.1.y).abs()) as f32;
    let sheep_to_center = ((center.0 - state.0.x).abs() + (center.1 - state.0.y).abs()) as f32;
    if sheep_to_center > 3.0 && dog_to_center > 3.0 {
        return (((state.1.x - state.0.x) * (state.1.x - state.0.x)
            + (state.1.y - state.0.y) * (state.1.y - state.0.y)) as f32)
            .sqrt();
    }
    let mut queue = Queue::new();
    let mut scores = HashMap::new();
    queue.add(field.clone());
    scores.insert(field, 0.0);
    while queue.size() > 0 {
        let current = queue.remove().unwrap();
        for child in current.get_dog_moves_bfs() {
            if child.sheep_won() {
                return *scores.get(&current).unwrap() + 1.0;
            }
            let sheep_to_dog_child = (((child.dog.x - child.sheep.x)
                * (child.dog.x - child.sheep.x)
                + (child.dog.y - child.sheep.y) * (child.dog.y - child.sheep.y))
                as f32)
                .sqrt();
            let sheep_to_dog = (((current.dog.x - current.sheep.x)
                * (current.dog.x - current.sheep.x)
                + (current.dog.y - current.sheep.y) * (current.dog.y - current.sheep.y))
                as f32)
                .sqrt();
            let dog_to_center =
                ((center.0 - child.dog.x).abs() + (center.1 - child.dog.y).abs()) as f32;

            if (sheep_to_dog_child < sheep_to_dog || dog_to_center < 5.0)
                && !scores.contains_key(&child)
            {
                queue.add(child.clone());
                scores.insert(child, scores.get(&current).unwrap() + 1.0 as f32);
            }
        }
    }
    0.0
}

pub(crate) fn model_2(
    state: (Sheep, Dog),
    sheep_distances: &HashMap<Sheep, f32>,
) -> (f32, f32, f32, f32, f32) {
    let sheep = state.0;
    let dog = state.1;
    let center = (15, 15);
    let sheep_to_center = sheep_distances.get(&sheep).unwrap();
    let sheep_to_dog = bfs_dog((sheep, dog));
    let dog_to_center = sheep_distances.get(&(Sheep::at(dog.x, dog.y))).unwrap();
    let dog_next_to_sheep = match ((dog.x - sheep.x).abs() + (dog.y - sheep.y).abs()) as f32 {
        1.0 => 1.0,
        2.0 => 0.5,
        _ => 0.0,
    };
    let dog_won = match ((center.0 - sheep.x).abs() + (center.1 - sheep.y).abs()) as f32 {
        0.0 => 1.0,
        1.0 => 0.5,
        _ => 0.0,
    };
    (
        *dog_to_center,
        *sheep_to_center,
        dog_next_to_sheep,
        sheep_to_dog,
        dog_won,
    )
}

pub(crate) fn dot_product(one: (f32, f32, f32, f32, f32), two: (f32, f32, f32, f32, f32)) -> f32 {
    one.0 * two.0 + one.1 * two.0 + one.2 * two.2 + one.3 * two.3 + one.4 * two.4
}

pub(crate) fn scalar_multiple(
    vector: (f32, f32, f32, f32, f32),
    scalar: f32,
) -> (f32, f32, f32, f32, f32) {
    (
        vector.0 * scalar,
        vector.1 * scalar,
        vector.2 * scalar,
        vector.3 * scalar,
        vector.4 * scalar,
    )
}

pub(crate) fn vector_subtraction(
    one: (f32, f32, f32, f32, f32),
    two: (f32, f32, f32, f32, f32),
) -> (f32, f32, f32, f32, f32) {
    (
        two.0 - one.0,
        two.1 - one.1,
        two.2 - one.2,
        two.3 - one.3,
        two.4 - one.4,
    )
}
