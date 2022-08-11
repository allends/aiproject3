use rand::{self, seq::SliceRandom};

#[derive(Clone, Copy)]
struct Sheep {
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

#[derive(Clone, Copy)]
struct Dog {
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
    grid: Vec<Vec<Cell>>,
    sheep: Sheep,
    dog: Dog,
}

impl Field {
    pub fn new() -> Self {
        // initialize an empty field, then make the pen
        let mut empty_grid = vec![];
        let mut sheep = Sheep::new();
        let mut dog = Dog::new();
        sheep.x = 0;
        sheep.y = 0;
        let size: usize = 4;

        dog.y = size as i32;
        dog.x = size as i32;

        let middle = size / 2;

        for i in 0..size+1 {
            let mut row = Vec::new();
            for j in 0..size+1 {
                row.push(Cell::new(i as i32, j as i32, Entity::Empty));
            }
            empty_grid.push(row);
        }

        // left wall
        empty_grid[middle - 1][middle - 1] = Cell::new(24, 24, Entity::Fence);
        empty_grid[middle][middle - 1] = Cell::new(25, 24, Entity::Fence);
        empty_grid[middle + 1][middle - 1] = Cell::new(26, 24, Entity::Fence);

        // right wall
        empty_grid[middle - 1][middle + 1] = Cell::new(24, 26, Entity::Fence);
        empty_grid[middle][middle + 1] = Cell::new(25, 26, Entity::Fence);
        empty_grid[middle + 1][middle + 1] = Cell::new(26, 26, Entity::Fence);

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
        if x - 1 >= 0 {
            moves.push(Cell::new(x - 1, y, Entity::Dog));
            if y - 1 >= 0 {
                moves.push(Cell::new(x - 1, y - 1, Entity::Dog))
            };
            if y + 1 < self.grid.len() as i32 {
                moves.push(Cell::new(x - 1, y + 1, Entity::Dog))
            };
        };
        if x + 1 < self.grid.len() as i32 {
            moves.push(Cell::new(x + 1, y, Entity::Dog));
            if y - 1 >= 0 {
                moves.push(Cell::new(x + 1, y - 1, Entity::Dog))
            };
            if y + 1 < self.grid.len() as i32 {
                moves.push(Cell::new(x + 1, y + 1, Entity::Dog))
            };
        };
        if y - 1 >= 0 {
            moves.push(Cell::new(x, y - 1, Entity::Dog))
        };
        if y + 1 < self.grid.len() as i32 {
            moves.push(Cell::new(x, y + 1, Entity::Dog))
        };
        moves
    }

    fn move_dog_to(&self, cell: Cell) -> Field {
        let mut new_field = self.clone();
        let old_dog = self.dog.as_cell();
        new_field.grid[cell.y as usize][cell.x as usize] = cell;
        new_field.grid[old_dog.y as usize][old_dog.x as usize] =
            Cell::new(old_dog.x, old_dog.y, Entity::Empty);
        new_field
    }

    fn get_dog_states(&self) -> Vec<Field> {
        let possible_moves = self.get_dog_moves();
        let mut states = Vec::<Field>::new();
        for possible_move in possible_moves {
            states.push(self.move_dog_to(possible_move));
        }
        states
    }

    // sheep related functions
    pub fn get_sheep_states(&self) -> Vec<Field> {
        let mut states = Vec::<Field>::new();
        let mut moves = Vec::<Cell>::new();

        if self.dog_in_view().is_some() {
            let position = (self.dog.x, self.dog.y);
            let x = self.sheep.x;
            let y = self.sheep.y;
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
        } else {
            let x = self.sheep.x;
            let y = self.sheep.y;
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
        }
        for possible_move in moves {
            states.push(self.move_sheep_to(possible_move));
        }
        states
    }

    pub fn move_sheep_to(&self, cell: Cell) -> Field {
        let mut new_field = self.clone();
        let old_sheep = self.sheep.as_cell();
        new_field.grid[cell.y as usize][cell.x as usize] = cell;
        new_field.grid[old_sheep.y as usize][old_sheep.x as usize] =
            Cell::new(old_sheep.x, old_sheep.y, Entity::Empty);
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
    pub fn t_star(state: Field) -> i32 {
        let sheep_cell = state.sheep.as_cell();
        let dog_cell = state.dog.as_cell();
        let sheep_pos = (sheep_cell.x, sheep_cell.y);
        let dog_pos = (dog_cell.x, dog_cell.y);

        // if the sheep is caught, return 0 since the task is over
        if sheep_pos == (state.grid.len() as i32, state.grid.len() as i32) {
            return 0;
        }

        // if they have the same position then the dog is dead
        if sheep_pos == dog_pos {
            return i32::MAX;
        }

        // here we use the recursive definition in order to get the cost
        // find all of the moves that the dog can make and the states that result
        let possible_states = state.get_dog_states();
        let mut minimum = i32::MAX;
        for possible_state in possible_states {
            let sheep_states = possible_state.get_sheep_states();
            let number_states = sheep_states.len() as i32;
            let mut summation = 0;
            for sheep_state in sheep_states {
                summation = summation + 1 / number_states * Field::t_star(sheep_state);
            }
            if summation < minimum {
              minimum = summation;
            }
        }

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
struct Cell {
    x: i32,
    y: i32,
    entity: Entity,
}

impl Cell {
    fn new(x: i32, y: i32, entity: Entity) -> Self {
        Self { x, y, entity }
    }
}
