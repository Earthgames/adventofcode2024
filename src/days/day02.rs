use std::fmt::format;

use anyhow::Result;

pub fn part1(input: String) -> Result<String> {
    let lists = parse(input);
    let result: u16 = lists
        .iter()
        .map(|x| match is_correct(x) {
            Some(_) => 0,
            None => 1,
        })
        .sum();
    Ok(result.to_string())
}

pub fn is_safe(a: &u16, b: &u16) -> (bool, bool) {
    if a == b {
        return (false, false);
    }
    if a > b {
        let diff = a - b;
        if (1..=3).contains(&diff) {
            (true, false)
        } else {
            (false, false)
        }
    } else {
        let diff = b - a;
        if (1..=3).contains(&diff) {
            (true, true)
        } else {
            (false, true)
        }
    }
}

fn parse(input: String) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|s| {
            s.split(" ")
                .map(|x| x.parse::<u16>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn is_correct(input: &[u16]) -> Option<usize> {
    let (safe, increase) = is_safe(&input[0], &input[1]);
    if !safe {
        return Some(0);
    }
    for i in 1..(input.len() - 1) {
        let (safe, new_increase) = is_safe(&input[i], &input[i + 1]);
        if !safe || increase != new_increase {
            return Some(i);
        }
    }
    None
}

fn try_make_correct(input: &[u16]) -> bool {
    match is_correct(input) {
        Some(i) => {
            let mut tries = vec![i];
            if i != 0 {
                tries.push(i - 1);
            }
            if i != input.len() - 1 {
                tries.push(i + 1);
            }
            tries.iter().any(|i| {
                let mut check = input.to_vec();
                check.remove(*i);
                is_correct(&check).is_none()
            })
        }
        None => true,
    }
}

pub fn part2(input: String) -> Result<String> {
    let lists = parse(input);
    let result: Vec<bool> = lists.iter().map(|x| try_make_correct(x)).collect();
    Ok(result.iter().filter(|x| **x).count().to_string())
}
