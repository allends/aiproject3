mod field;
use std::collections::HashMap;

use crate::field::Field;

fn main() {
    let mut field = Field::new();
    field.print();
    println!();
    // let states = field.get_dog_states();
    // for state in states {
    //     state.print();
    //     println!();
    // }
    let mut state_set = HashMap::new();
    let best_moves = Field::t_star(field, &mut state_set);
    println!("the optimal path is {} moves", best_moves);
}
