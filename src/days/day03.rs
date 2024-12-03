use anyhow::Result;
use regex::{Regex, RegexSet};

pub fn part1(input: String) -> Result<String> {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut found = vec![];
    for (_, [a, b]) in regex.captures_iter(&input).map(|c| c.extract()) {
        found.push((a.parse::<u32>()?, b.parse::<u32>()?));
    }
    let result = found.iter().fold(0, |acc, (a, b)| a * b + acc);
    Ok(result.to_string())
}

pub fn part2(input: String) -> Result<String> {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|don't\(\)|do\(\)").unwrap();
    let mut found = vec![];
    let mut enabled = true;
    for cap in regex.captures_iter(&input) {
        match &cap[0] {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if !enabled {
                    continue;
                }
                found.push((cap[1].parse::<u32>()?, cap[2].parse::<u32>()?));
            }
        }
    }
    let result = found.iter().fold(0, |acc, (a, b)| a * b + acc);
    Ok(result.to_string())
}
