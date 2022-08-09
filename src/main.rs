use rand::{self, seq::SliceRandom};
struct Sheep {
    x: i32,
    y: i32,
}

impl Sheep {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

struct Field {
    grid: Vec<Vec<Cell>>,
    sheep: Sheep,
}

impl Field {
    fn new() -> Self {
        // initialize an empty field, then make the pen
        let mut empty_grid = vec![];
        let sheep = Sheep::new();

        for i in 0..51 {
            let mut row = Vec::new();
            for j in 0..51 {
                row.push(Cell::new(i as i32, j as i32, Entity::Empty));
            }
            empty_grid.push(row);
        }

        // left wall
        empty_grid[24][24] = Cell::new(24, 24, Entity::Fence);
        empty_grid[25][24] = Cell::new(25, 24, Entity::Fence);
        empty_grid[26][24] = Cell::new(26, 24, Entity::Fence);

        // right wall
        empty_grid[24][26] = Cell::new(24, 26, Entity::Fence);
        empty_grid[25][26] = Cell::new(25, 26, Entity::Fence);
        empty_grid[26][26] = Cell::new(26, 26, Entity::Fence);

        // bottom of pen
        empty_grid[26][25] = Cell::new(26, 25, Entity::Fence);

        // put the sheep on the grid
        empty_grid[sheep.y as usize][sheep.x as usize] = Cell::new(sheep.x, sheep.y, Entity::Sheep);

        return Self {
            grid: empty_grid,
            sheep,
        };
    }

    fn move_sheep(&mut self) {
        // get the squares nearby and see if the dog is in one of them
        match self.dog_in_view() {
            Some(cell) => (),
            None => self.move_sheep_random(),
        }
    }

    fn move_sheep_towards(&mut self, position: (i32, i32)) {}

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
    }

    fn get_sheep_view(&self) -> Vec<Cell> {
        let sheep_postion = (self.sheep.x, self.sheep.y);
        let lower_bound_x = sheep_postion.0 - 2;
        let upper_bound_x = sheep_postion.0 + 2;
        let lower_bound_y = sheep_postion.1 - 2;
        let upper_bound_y = sheep_postion.1 + 2;
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

    fn print(&self) {
        for row in self.grid.iter() {
            for cell in row {
                match cell.entity {
                    Entity::Empty => print!("_ "),
                    Entity::Fence => print!("☐ "),
                    Entity::Sheep => print!("S "),
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

fn main() {
    let mut field = Field::new();
    field.print();
    field.move_sheep();
    println!();
    field.print();
}
