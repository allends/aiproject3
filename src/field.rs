use std::collections::HashMap;

use rand::{self, seq::SliceRandom};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sheep {
    x: i32,
    y: i32,
}

impl Sheep {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn at(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn as_cell(&self) -> Cell {
        Cell {
            x: self.x,
            y: self.y,
            entity: Entity::Sheep,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dog {
    x: i32,
    y: i32,
}

impl Dog {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn at(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn as_cell(&self) -> Cell {
        Cell {
            x: self.x,
            y: self.y,
            entity: Entity::Dog,
        }
    }
}

#[derive(Clone)]
pub struct Field {
    pub grid: Vec<Vec<Cell>>,
    sheep: Sheep,
    dog: Dog,
}

impl Field {
    pub fn new() -> Self {
        // initialize an empty field, then make the pen
        let mut empty_grid = vec![];
        let size: usize = 6;
        let middle = size / 2;
        let mut sheep = Sheep::new();
        let mut dog = Dog::new();

        //sheep starting position
        // sheep.x = middle as i32 - 2;
        // sheep.y = middle as i32 - 1;
        sheep.x = 2;
        sheep.y = 2;

        // dog starting position
        // dog.y = size as i32;
        // dog.x = size as i32;
        dog.y = 1;
        dog.x = 1;

        for i in 0..size + 1 {
            let mut row = Vec::new();
            for j in 0..size + 1 {
                row.push(Cell::new(j as i32, i as i32, Entity::Empty));
            }
            empty_grid.push(row);
        }

        // left wall
        empty_grid[middle - 1][middle - 1] =
            Cell::new(middle as i32 - 1, middle as i32 - 1, Entity::Fence);
        empty_grid[middle][middle - 1] = Cell::new(middle as i32 - 1, middle as i32, Entity::Fence);
        empty_grid[middle + 1][middle - 1] =
            Cell::new(middle as i32 - 1, middle as i32 + 1, Entity::Fence);

        // right wall
        empty_grid[middle - 1][middle + 1] =
            Cell::new(middle as i32 + 1, middle as i32 - 1, Entity::Fence);
        empty_grid[middle][middle + 1] = Cell::new(middle as i32 + 1, middle as i32, Entity::Fence);
        empty_grid[middle + 1][middle + 1] =
            Cell::new(middle as i32 + 1, middle as i32 + 1, Entity::Fence);

        // bottom of pen
        empty_grid[middle + 1][middle] = Cell::new(26, 25, Entity::Fence);

        // put the sheep on the grid
        empty_grid[sheep.y as usize][sheep.x as usize] = sheep.as_cell();

        // test if the sheeps movement actually works
        empty_grid[dog.y as usize][dog.x as usize] = dog.as_cell();

        return Self {
            grid: empty_grid,
            sheep,
            dog,
        };
    }

    // dog related functions

    fn get_dog_moves(&self) -> Vec<Cell> {
        let x = self.dog.x;
        let y = self.dog.y;
        let mut moves = Vec::<Cell>::new();
        for row in y - 1..y + 2 {
            for column in x - 1..x + 2 {
                if column == x && row == y {
                    continue;
                }
                match self.grid.get(row as usize) {
                    Some(row) => match row.get(column as usize) {
                        Some(cell) => match cell.entity {
                            Entity::Empty => {
                                let mut dog_cell = cell.clone();
                                dog_cell.entity = Entity::Dog;
                                moves.push(dog_cell);
                            }
                            _ => continue,
                        },
                        None => continue,
                    },
                    None => continue,
                };
            }
        }
        moves
    }

    fn move_dog_to(&self, cell: Cell) -> Field {
        let mut new_field = self.clone();
        let old_dog = self.dog.as_cell();
        new_field.grid[cell.y as usize][cell.x as usize] = cell;
        new_field.grid[old_dog.y as usize][old_dog.x as usize] =
            Cell::new(old_dog.x, old_dog.y, Entity::Empty);
        new_field.dog.x = cell.x;
        new_field.dog.y = cell.y;
        new_field
    }

    pub fn get_dog_states(&self) -> Vec<Field> {
        let possible_moves = self.get_dog_moves();
        let mut states = Vec::<Field>::new();
        for possible_move in possible_moves {
            states.push(self.move_dog_to(possible_move));
        }
        states
    }

    // sheep related functions

    pub fn get_sheep_moves(&self) -> Vec<Cell> {
        let x = self.sheep.x;
        let y = self.sheep.y;
        let mut moves = Vec::<Cell>::new();
        for row in y - 1..y + 2 {
            for column in x - 1..x + 2 {
                if (column != x && row != y) || (column == x && row == y) {
                    continue;
                }
                match self.grid.get(row as usize) {
                    Some(grid_row) => match grid_row.get(column as usize) {
                        Some(cell) => match cell.entity {
                            Entity::Empty | Entity::Dog => {
                                let mut sheep_cell = cell.clone();
                                sheep_cell.entity = Entity::Sheep;
                                moves.push(sheep_cell);
                            }
                            _ => continue,
                        },
                        None => continue,
                    },
                    None => continue,
                };
            }
        }
        moves
    }

    pub fn get_sheep_states(&self) -> Vec<Field> {
        let mut states = Vec::<Field>::new();
        let mut possible_moves = Vec::<Cell>::new();

        if self.dog_in_view().is_some() {
            let position = (self.dog.x, self.dog.y);
            let x = self.sheep.x;
            let y = self.sheep.y;
            for sheep_move in self.get_sheep_moves() {
                if (sheep_move.x - position.0).abs() < (x - position.0).abs() {
                    possible_moves.push(sheep_move);
                }
                if (sheep_move.y - position.1).abs() < (y - position.1).abs() {
                    possible_moves.push(sheep_move);
                }
            }
        } else {
            possible_moves = self.get_sheep_moves();
        }
        for possible_move in possible_moves {
            states.push(self.move_sheep_to(possible_move));
        }
        states
    }

    pub fn move_sheep_to(&self, cell: Cell) -> Field {
        let mut new_field = self.clone();
        let old_sheep = self.sheep.as_cell();
        new_field.grid[cell.y as usize][cell.x as usize] = cell;
        new_field.grid[old_sheep.y as usize][old_sheep.x as usize] =
            Cell::new(old_sheep.y, old_sheep.x, Entity::Empty);
        new_field.sheep.x = cell.x;
        new_field.sheep.y = cell.y;
        new_field
    }

    pub fn move_sheep(&mut self) {
        // get the squares nearby and see if the dog is in one of them
        match self.dog_in_view() {
            Some(cell) => self.move_sheep_towards((cell.x, cell.y)),
            None => self.move_sheep_random(),
        }
    }

    fn move_sheep_towards(&mut self, position: (i32, i32)) {
        let x = self.sheep.x;
        let y = self.sheep.y;
        let mut moves = Vec::<Cell>::new();
        if position.0 < x {
            moves.push(Cell::new(x - 1, y, Entity::Sheep))
        };
        if position.0 > x {
            moves.push(Cell::new(x + 1, y, Entity::Sheep))
        };
        if position.1 < y {
            moves.push(Cell::new(x, y - 1, Entity::Sheep))
        };
        if position.1 > y {
            moves.push(Cell::new(x, y + 1, Entity::Sheep))
        };
        let chosen_move = moves.choose(&mut rand::thread_rng()).unwrap();
        self.grid[self.sheep.y as usize][self.sheep.x as usize].entity = Entity::Empty;
        self.grid[chosen_move.y as usize][chosen_move.x as usize] = *chosen_move;
        self.sheep = Sheep::at(chosen_move.x, chosen_move.y);
    }

    fn move_sheep_random(&mut self) {
        let x = self.sheep.x;
        let y = self.sheep.y;
        let mut moves = Vec::<Cell>::new();
        if x - 1 >= 0 {
            moves.push(Cell::new(x - 1, y, Entity::Sheep))
        };
        if x + 1 < 51 {
            moves.push(Cell::new(x + 1, y, Entity::Sheep))
        };
        if y - 1 >= 0 {
            moves.push(Cell::new(x, y - 1, Entity::Sheep))
        };
        if y + 1 < 51 {
            moves.push(Cell::new(x, y + 1, Entity::Sheep))
        };
        let chosen_move = moves.choose(&mut rand::thread_rng()).unwrap();
        self.grid[self.sheep.y as usize][self.sheep.x as usize].entity = Entity::Empty;
        self.grid[chosen_move.y as usize][chosen_move.x as usize] = *chosen_move;
        self.sheep = Sheep::at(chosen_move.x, chosen_move.y);
    }

    fn get_sheep_view(&self) -> Vec<Cell> {
        let sheep_postion = (self.sheep.x, self.sheep.y);
        let lower_bound_x = sheep_postion.0 - 2;
        let upper_bound_x = sheep_postion.0 + 3;
        let lower_bound_y = sheep_postion.1 - 2;
        let upper_bound_y = sheep_postion.1 + 3;
        let mut result = Vec::new();
        for i in lower_bound_y..upper_bound_y {
            for j in lower_bound_x..upper_bound_x {
                let cell_view = self.grid.get(i as usize);
                match cell_view {
                    Some(row) => match row.get(j as usize) {
                        Some(cell) => result.push(cell.clone()),
                        None => continue,
                    },
                    None => continue,
                }
            }
        }
        result
    }

    fn dog_in_view(&self) -> Option<Cell> {
        let view = self.get_sheep_view();
        for cell in view.iter() {
            if cell.entity == Entity::Dog {
                return Some(cell.clone());
            }
        }
        None
    }

    // calculate the expected moves for a given field
    pub fn t_star(state: Field, state_set: &mut HashMap<(Sheep, Dog), i32>) -> i32 {
        let sheep_cell = state.sheep.as_cell();
        let dog_cell = state.dog.as_cell();
        let sheep_pos = (sheep_cell.x, sheep_cell.y);
        let dog_pos = (dog_cell.x, dog_cell.y);

        // if the sheep is caught, return 0 since the task is over
        if sheep_pos == (state.grid.len() as i32 / 2, state.grid.len() as i32 / 2) {
            println!("returning a base case");
            state_set.insert((state.sheep, state.dog), i32::MAX);
            return 0;
        }

        // if they have the same position then the dog is dead
        if sheep_pos == dog_pos {
            println!("returning a base case max");
            state_set.insert((state.sheep, state.dog), i32::MAX);
            return i32::MAX;
        }

        // here we use the recursive definition in order to get the cost
        // find all of the moves that the dog can make and the states that result
        let possible_states = state.get_dog_states();
        let mut minimum = i32::MAX;
        for possible_state in possible_states {
            possible_state.print();
            println!();
            let sheep_states = possible_state.get_sheep_states();
            let number_states = sheep_states.len() as i32;
            let mut summation = 0;
            for sheep_state in sheep_states {
                state_set.insert((sheep_state.sheep, sheep_state.dog), i32::MAX);
                sheep_state.print();
                println!();
                summation = summation + 1 / number_states * Field::t_star(sheep_state, state_set);
            }
            if summation < minimum {
                minimum = summation;
            }
        }
        println!("returning {}", minimum);
        minimum
    }

    pub fn print(&self) {
        for row in self.grid.iter() {
            for cell in row {
                match cell.entity {
                    Entity::Empty => print!("_ "),
                    Entity::Fence => print!("☐ "),
                    Entity::Sheep => print!("S "),
                    Entity::Dog => print!("D "),
                    _ => print!("  "),
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Entity {
    Empty,
    Fence,
    Sheep,
    Dog,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    x: i32,
    y: i32,
    entity: Entity,
}

impl Cell {
    fn new(x: i32, y: i32, entity: Entity) -> Self {
        Self { x, y, entity }
    }
}
