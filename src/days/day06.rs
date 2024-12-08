use std::collections::HashMap;

use anyhow::{Ok, Result};

pub fn part1(input: String) -> Result<String> {
    let (mut grid, mut guard, (x_max, y_max)) = parse(input);
    loop {
        let pos_next @ (x_next, y_next) = guard.move_direction();
        if !(0..=x_max).contains(&x_next) || !(0..=y_max).contains(&y_next) {
            break;
        }
        if grid.get(&pos_next) == Some(&Block::Block) {
            guard.turn();
        } else {
            guard.update_pos(pos_next);
            grid.insert(pos_next, Block::Walked(guard.direction)); // Does not matter yet
        }
    }
    let result = grid.values().fold(0, |acc, val| match val {
        Block::Block => acc,
        Block::Walked(_) => acc + 1,
    });
    Ok(result.to_string())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}
fn turn_direction(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Guard {
    pub pos: (i32, i32),
    pub direction: Direction,
}

impl Guard {
    fn new(pos: (i32, i32)) -> Guard {
        Guard {
            pos,
            direction: Direction::North,
        }
    }
    fn update_pos(&mut self, pos: (i32, i32)) {
        self.pos = pos;
    }
    fn turn(&mut self) {
        self.direction = turn_direction(self.direction)
    }
    fn turnable(&self, direction: &Direction) -> bool {
        matches!(
            (self.direction, direction),
            (Direction::North, Direction::East)
                | (Direction::East, Direction::South)
                | (Direction::South, Direction::West)
                | (Direction::West, Direction::North)
        )
    }
    fn move_direction(&self) -> (i32, i32) {
        self.move_direction_point(self.pos)
    }
    fn move_direction_point(&self, pos: (i32, i32)) -> (i32, i32) {
        let (x, y) = pos;
        match self.direction {
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::North => (x, y - 1),
            Direction::West => (x - 1, y),
        }
    }
    fn try_move_lots(&self, grid: &Grid, x_max: i32, y_max: i32) -> bool {
        let direction = turn_direction(self.direction);
        let max_sim = 0;
        let mut sim = 0;
        let mut guard = Guard {
            pos: self.pos,
            direction,
        };
        loop {
            let pos_next @ (x_next, y_next) = guard.move_direction();
            if !(0..=x_max).contains(&x_next) || !(0..=y_max).contains(&y_next) {
                break false;
            }
            if let Some(block) = grid.get(&pos_next) {
                match block {
                    Block::Block => {
                        if sim >= max_sim {
                            break false;
                        } else {
                            sim += 1;
                            guard.turn();
                        }
                    }

                    Block::Walked(direction_test) => {
                        if &direction == direction_test {
                            println!("{:?}, {:?}", self.pos, pos_next);
                            print_grid_point(grid, x_max, y_max, pos_next);
                            println!();
                            break true;
                        }
                        guard.update_pos(pos_next);
                    }
                }
            } else {
                guard.update_pos(pos_next);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Block {
    Block,
    Walked(Direction),
}

type Grid = HashMap<(i32, i32), Block>;

fn print_grid(grid: &Grid, x_max: i32, y_max: i32) {
    for y in 0..=y_max {
        let mut str = y.to_string();
        for x in 0..=x_max {
            if let Some(block) = grid.get(&(x, y)) {
                match block {
                    Block::Block => str.push('#'),
                    Block::Walked(direction) => str.push(match direction {
                        Direction::North => '^',
                        Direction::East => '>',
                        Direction::South => 'v',
                        Direction::West => '<',
                    }),
                }
            } else {
                str.push('.')
            }
        }
        println!("{}", str);
    }
}

fn print_grid_point(grid: &Grid, x_max: i32, y_max: i32, pos: (i32, i32)) {
    for y in 0..=y_max {
        let mut str = y.to_string();
        for x in 0..=x_max {
            if pos == (x, y) {
                str.push('$')
            } else if let Some(block) = grid.get(&(x, y)) {
                match block {
                    Block::Block => str.push('#'),
                    Block::Walked(direction) => str.push(match direction {
                        Direction::North => '^',
                        Direction::East => '>',
                        Direction::South => 'v',
                        Direction::West => '<',
                    }),
                }
            } else {
                str.push('.')
            }
        }
        println!("{}", str);
    }
}

fn parse(input: String) -> (Grid, Guard, (i32, i32)) {
    let mut grid = HashMap::new();
    let mut guard_pos = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    grid.insert((x as i32, y as i32), Block::Block);
                }
                '^' => {
                    grid.insert((x as i32, y as i32), Block::Walked(Direction::North));
                    guard_pos = (x as i32, y as i32);
                }
                _ => (),
            };
        }
    }
    let y_max = input.lines().count() as i32;
    let x_max = input.lines().next().unwrap().chars().count() as i32;
    (grid, Guard::new(guard_pos), (x_max, y_max))
}

pub fn part2(input: String) -> Result<String> {
    let (mut grid, mut guard, (x_max, y_max)) = parse(input);
    let mut result = 0;
    loop {
        let pos_next @ (x_next, y_next) = guard.move_direction();
        if !(0..=x_max).contains(&x_next) || !(0..=y_max).contains(&y_next) {
            break;
        }
        // if let Some(block) = grid.get(&pos_next) {
        //     match block {
        //         Block::Block => guard.turn(),
        //         Block::Walked(direction) => {
        //             let direction = *direction; // immutable borrow from grid lost here
        //             guard.update_pos(pos_next);
        //             grid.insert(pos_next, Block::Walked(guard.direction)); // so we can borrow grid here mutably
        //             if !guard.turnable(&direction) {
        //                 continue;
        //             }
        //             let (x_next, y_next) = guard.move_direction();
        //             if !(0..=x_max).contains(&x_next) || !(0..=y_max).contains(&y_next) {
        //                 continue;
        //             } else {
        //                 println!("{:?}", (x_next, y_next));
        //                 result += 1;
        //             }
        //         }
        //     }
        // }
        if grid.get(&pos_next) == Some(&Block::Block) {
            guard.turn();
        } else {
            if guard.try_move_lots(&grid, x_max, y_max) {
                // println!("{:?}", guard.pos);
                result += 1;
            }
            guard.update_pos(pos_next);
            grid.insert(pos_next, Block::Walked(guard.direction));
        }
    }
    Ok(result.to_string())
}
