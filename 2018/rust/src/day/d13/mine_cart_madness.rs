use crate::day::{
    d13::{Cart, Cell, CellType, Direction},
    Input, InputError, InputResult, Output, Point, Wrapper,
};

fn solve(carts: &mut Vec<Cart>, grid: &mut Vec<Vec<Cell>>) -> (usize, usize, usize, usize) {
    let mut crashes: Vec<Point<usize>> = Vec::new();

    loop {
        carts.sort_by_key(|cart| cart.pos);

        for i in 0..carts.len() {
            if carts[i].crashed {
                continue;
            }

            grid[carts[i].pos.0][carts[i].pos.1].occupied = false;
            carts[i].pos = carts[i].dir.apply(carts[i].pos);

            if grid[carts[i].pos.0][carts[i].pos.1].occupied {
                if let Some(j) = (0..carts.len())
                    .find(|j| i != *j && carts[i].pos == carts[*j].pos && !carts[i].crashed)
                {
                    grid[carts[i].pos.0][carts[i].pos.1].occupied = false;
                    carts[i].crashed = true;
                    carts[j].crashed = true;
                    crashes.extend_from_slice(&[carts[i].pos, carts[j].pos]);
                } else {
                    panic!("whut");
                }

                continue;
            }
            grid[carts[i].pos.0][carts[i].pos.1].occupied = true;
            carts[i].dir = grid[carts[i].pos.0][carts[i].pos.1]
                .cell_type
                .apply(carts[i].dir, &mut carts[i].state);
        }

        if crashes.len() == carts.len() - 1 {
            let (py, px) = crashes[0];
            let (y, x) = carts
                .iter()
                .filter(|cart| !cart.crashed)
                .collect::<Vec<&Cart>>()[0]
                .pos;
            return (px, py, x, y);
        }
    }
}

pub fn run(input: Input) -> (Output, Output) {
    let (mut carts, mut grid): (Vec<Cart>, Vec<Vec<Cell>>) = input.unwrap();
    let (px, py, x, y) = solve(&mut carts, &mut grid);

    (
        Output::S(format!("{},{}", px, py)),
        Output::S(format!("{},{}", x, y)),
    )
}

pub fn parse() -> InputResult<Input> {
    let mut carts = Vec::<Cart>::new();
    let mut grid: Vec<Vec<Cell>> = Vec::new();

    for (y, line) in std::fs::read_to_string("../input/13")?.lines().enumerate() {
        grid.push(vec![]);
        for (x, ch) in line.chars().enumerate() {
            let (cell_type, opt_dir) = match ch {
                '-' => (CellType::Horizontal, None),
                '|' => (CellType::Vertical, None),
                '/' => (CellType::ForwardSlash, None),
                '\\' => (CellType::BackSlash, None),
                '+' => (CellType::Intersection, None),
                'v' => (CellType::Vertical, Some(Direction::South)),
                '^' => (CellType::Vertical, Some(Direction::North)),
                '>' => (CellType::Horizontal, Some(Direction::East)),
                '<' => (CellType::Horizontal, Some(Direction::West)),
                ' ' => (CellType::Empty, None),
                _ => return Err(InputError::InvalidInput),
            };

            grid[y].push(if let Some(dir) = opt_dir {
                carts.push(Cart {
                    pos: (y, x),
                    dir,
                    state: 0,
                    crashed: false,
                });
                Cell {
                    cell_type,
                    occupied: true,
                }
            } else {
                Cell {
                    cell_type,
                    occupied: false,
                }
            });
        }
    }

    Ok(Input::D13((carts, grid)))
}
