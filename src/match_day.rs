use adventofcode2024::days::{day01, day02};
use anyhow::anyhow;
use anyhow::Result;
use std::fs;
use std::time::{Duration, Instant};

macro_rules! run_day {
    ($days:ident, $part:ident, $input:expr) => {{
        let start = Instant::now();
        let result = $days::$part($input());
        let end = Instant::now();
        (result, end - start)
    }};
}

pub fn match_day(day: u8, part2: bool, test: bool) -> (Result<String>, Duration) {
    let input = || get_input(day, test);
    match (day, part2) {
        (1, false) => run_day!(day01, part1, input),
        (1, true) => run_day!(day01, part2, input),
        (2, false) => run_day!(day02, part1, input),
        (2, true) => run_day!(day02, part2, input),
        _ => (Err(anyhow!("could not find day")), Duration::from_millis(0)),
    }
}

fn get_input(day: u8, test: bool) -> String {
    let mut path = format!("input/input{}", day);
    if test {
        path = format!("{}test", path);
    }
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("could not read input file \"{path}\""))
}
