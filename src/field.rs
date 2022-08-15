use rand::{self, seq::SliceRandom};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Sheep {
    pub x: i32,
    pub y: i32,
}

impl Sheep {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn at(x: i32, y: i32) -> Self {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Dog {
    pub x: i32,
    pub y: i32,
}

impl Dog {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn at(x: i32, y: i32) -> Self {
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Field {
    pub grid: Vec<Vec<Cell>>,
    pub sheep: Sheep,
    pub dog: Dog,
}

impl Field {
    pub fn new() -> Self {
        // initialize an empty field, then make the pen
        let mut empty_grid = vec![];
        let size: usize = 31;
        let middle = (size - 1) / 2;
        let mut sheep = Sheep::new();
        let mut dog = Dog::new();

        //sheep starting position
        sheep.x = 0;
        sheep.y = 0;

        // dog starting position
        dog.y = size as i32 - 1;
        dog.x = size as i32 - 1;

        for i in 0..size {
            let mut row = Vec::new();
            for j in 0..size {
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
        empty_grid[middle + 1][middle] = Cell::new(middle as i32 + 1, middle as i32, Entity::Fence);

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

    pub fn is_valid(&self) -> bool {
        let size: usize = self.grid.len();
        let middle = (size - 1) / 2;
        let grid = self.grid.clone();

        // left wall
        let left_wall = grid[middle - 1][middle - 1].entity == Entity::Fence
            && grid[middle][middle - 1].entity == Entity::Fence
            && grid[middle + 1][middle - 1].entity == Entity::Fence;

        // right wall
        let right_wall = grid[middle - 1][middle + 1].entity == Entity::Fence
            && grid[middle][middle + 1].entity == Entity::Fence
            && grid[middle + 1][middle + 1].entity == Entity::Fence;

        // bottom of pen
        let bottom_wall = grid[middle + 1][middle].entity == Entity::Fence;

        return left_wall && right_wall && bottom_wall;
    }

    pub fn sheep_won(&self) -> bool {
        self.dog.x == self.sheep.x && self.dog.y == self.sheep.y
    }

    pub fn with(sheep: Sheep, dog: Dog) -> Self {
        let mut field = Field::new();
        let old_dog = field.dog;
        let old_sheep = field.sheep;
        field.grid[old_sheep.y as usize][old_sheep.x as usize].entity = Entity::Empty;
        field.grid[old_dog.y as usize][old_dog.x as usize].entity = Entity::Empty;
        field.sheep = sheep;
        field.dog = dog;
        field.grid[sheep.y as usize][sheep.x as usize].entity = Entity::Sheep;
        field.grid[dog.y as usize][dog.x as usize].entity = Entity::Dog;
        field
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

    fn get_dog_moves_ignore_sheep(&self) -> Vec<Cell> {
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
                            Entity::Empty | Entity::Sheep => {
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

    pub fn move_dog_to(&self, cell: Cell) -> Field {
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

    pub fn get_sheep_moves_bfs(&self) -> Vec<Field> {
        let mut states = Vec::<Field>::new();
        let possible_moves = self.get_sheep_moves();
        for possible_move in possible_moves {
            states.push(self.move_sheep_to(possible_move));
        }
        states
    }

    pub fn get_dog_moves_bfs(&self) -> Vec<Field> {
        let mut states = Vec::<Field>::new();
        let possible_moves = self.get_dog_moves_ignore_sheep();
        for possible_move in possible_moves {
            states.push(self.move_dog_to(possible_move));
        }
        states
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

            if possible_moves.len() == 0 {
                possible_moves = self.get_sheep_moves();
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
            Cell::new(old_sheep.x, old_sheep.y, Entity::Empty);
        new_field.sheep.x = cell.x;
        new_field.sheep.y = cell.y;
        new_field
    }

    pub fn move_sheep(&mut self) {
        // get the squares nearby and see if the dog is in one of them
        let states = self.get_sheep_states();
        let chosen_move = states.choose(&mut rand::thread_rng()).unwrap();
        self.grid = chosen_move.grid.clone();
        self.sheep = chosen_move.sheep;
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

    pub fn dog_won(&self) -> bool {
        (self.sheep.x == self.grid.len() as i32 / 2 && self.sheep.y == self.grid.len() as i32 / 2)
            || ((self.dog.x == self.grid.len() as i32 / 2
                && self.dog.y == self.grid.len() as i32 / 2)
                && (self.sheep.x == self.grid.len() as i32 / 2
                    && self.sheep.y == self.grid.len() as i32 / 2 - 1))
    }

    pub fn print(&self) {
        for row in self.grid.iter() {
            for cell in row {
                match cell.entity {
                    Entity::Empty => print!("_ "),
                    Entity::Fence => print!("☐ "),
                    Entity::Sheep => print!("S "),
                    Entity::Dog => print!("D "),
                    // _ => print!("  "),
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum Entity {
    Empty,
    Fence,
    Sheep,
    Dog,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Cell {
    x: i32,
    y: i32,
    pub entity: Entity,
}

impl Cell {
    pub fn new(x: i32, y: i32, entity: Entity) -> Self {
        Self { x, y, entity }
    }
}
