mod field;
use crate::field::Field;

fn main() {
    let mut field = Field::new();
    field.print();
    println!();
    let best_moves = Field::t_star(field);
    println!("the optimal path is {} moves", best_moves);
}
