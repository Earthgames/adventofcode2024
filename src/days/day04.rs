use anyhow::{Ok, Result};

pub fn part1(input: String) -> Result<String> {
    let grid = parse(input);
    let mut result = 0;
    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[0].len() {
            if grid.point((x, y)) == Some(Xmas::X) {
                result += DIRECTIONS
                    .iter()
                    .filter(|d| grid.check_direction(**d, (x, y)))
                    .count()
            }
        }
    }
    Ok(result.to_string())
}

const DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];
const DIAGONALS: [Direction; 4] = [
    Direction::NorthEast,
    Direction::SouthEast,
    Direction::SouthWest,
    Direction::NorthWest,
];
#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn move_direction(direction: Direction, point: (usize, usize)) -> Option<(usize, usize)> {
    let (x, y) = point;
    let (x_null, y_null) = (x == 0, y == 0);
    match (direction, x_null, y_null) {
        (Direction::South, _, _) => Some((x, y + 1)),
        (Direction::SouthEast, _, _) => Some((x + 1, y + 1)),
        (Direction::East, _, _) => Some((x + 1, y)),
        (Direction::NorthEast, _, false) => Some((x + 1, y - 1)),
        (Direction::North, _, false) => Some((x, y - 1)),
        (Direction::NorthWest, false, false) => Some((x - 1, y - 1)),
        (Direction::West, false, _) => Some((x - 1, y)),
        (Direction::SouthWest, false, _) => Some((x - 1, y + 1)),
        _ => None,
    }
}

struct Grid {
    pub grid: Vec<Vec<Xmas>>,
}

impl Grid {
    fn new(grid: Vec<Vec<Xmas>>) -> Grid {
        Grid { grid }
    }
    fn check_direction(&self, direction: Direction, point: (usize, usize)) -> bool {
        let mut mut_point = Some(point);
        for xma in XMAS {
            let point = match mut_point {
                None => return false,
                Some(p) => p,
            };
            if self.point(point) != Some(xma) {
                return false;
            }
            mut_point = move_direction(direction, point);
        }
        true
    }

    fn check_x_max(&self, point: (usize, usize)) -> bool {
        if self.point(point) != Some(Xmas::A) {
            return false;
        }
        let mut xmas = X_MAS.to_vec();
        'outer: for _ in 0..4 {
            xmas.rotate_right(1);
            for (i, dir) in DIAGONALS.iter().enumerate() {
                match move_direction(*dir, point) {
                    Some(p) => {
                        if self.point(p) != Some(xmas[i]) {
                            continue 'outer;
                        }
                    }
                    None => return false,
                }
            }
            return true;
        }
        false
    }

    fn point(&self, point: (usize, usize)) -> Option<Xmas> {
        self.grid.get(point.1)?.get(point.0).copied()
    }
}
const XMAS: [Xmas; 4] = [Xmas::X, Xmas::M, Xmas::A, Xmas::S];
const X_MAS: [Xmas; 4] = [Xmas::M, Xmas::M, Xmas::S, Xmas::S];
#[derive(Clone, Copy, PartialEq, Eq)]
enum Xmas {
    X,
    M,
    A,
    S,
}

fn parse(input: String) -> Grid {
    Grid::new(
        input
            .lines()
            .map(|c| {
                c.chars()
                    .map(|c| match c {
                        'X' => Xmas::X,
                        'M' => Xmas::M,
                        'A' => Xmas::A,
                        'S' => Xmas::S,
                        _ => panic!("Not xmas"),
                    })
                    .collect()
            })
            .collect(),
    )
}

pub fn part2(input: String) -> Result<String> {
    let grid = parse(input);
    let mut result = 0;
    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[0].len() {
            if grid.point((x, y)) == Some(Xmas::A) && grid.check_x_max((x, y)) {
                result += 1;
                println!("{:?}", (x, y));
            }
        }
    }
    Ok(result.to_string())
}
