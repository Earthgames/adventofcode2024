use std::collections::HashMap;

use anyhow::{Ok, Result};

pub fn part1(input: String) -> Result<String> {
    let (mut list_a, mut list_b) = parse(input);
    list_a.sort();
    list_b.sort();
    let result = list_a
        .iter()
        .zip(list_b)
        .fold(0, |i, (a, b)| i + (a - b).abs());
    Ok(result.to_string())
}

fn parse(input: String) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|s| {
            let mut a = s.split("   ");
            (a.next().unwrap().to_string(), a.next().unwrap().to_string())
        })
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip()
}

pub fn part2(input: String) -> Result<String> {
    let (list_a, list_b) = parse(input);
    let mut found: HashMap<i32, usize> = HashMap::new();
    let result = list_a.iter().fold(0, |o, i| {
        let add = if found.contains_key(i) {
            found[i]
        } else {
            let thing = *i as usize * find_amount(&list_b, i);
            found.insert(*i, thing);
            thing
        };
        add + o
    });
    Ok(result.to_string())
}

fn find_amount(search: &[i32], needle: &i32) -> usize {
    search.iter().filter(|x| *x == needle).count()
}
