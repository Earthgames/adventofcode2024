use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use anyhow::Result;

pub fn part1(input: String) -> Result<String> {
    let (rules, updates) = parse(input);
    let result: u16 = updates
        .iter()
        .filter(|update| {
            let mut forbidden = HashSet::new();
            for val in update.iter() {
                if forbidden.contains(val) {
                    return false;
                }
                if let Some(forbidden_to_add) = rules.get(val) {
                    for forbid in forbidden_to_add {
                        forbidden.insert(forbid);
                    }
                }
            }
            true
        })
        .map(|update| update[update.len() / 2])
        .sum();
    Ok(result.to_string())
}

fn parse(input: String) -> (HashMap<u16, Vec<u16>>, Vec<Vec<u16>>) {
    let mut rules = HashMap::new();
    let mut lines = input.lines();
    for rule in lines.by_ref() {
        if rule.is_empty() {
            break;
        }
        if let [a, b] = rule
            .split('|')
            .map(|s| s.parse::<u16>().unwrap())
            .collect::<Vec<_>>()[..]
        {
            let entry = rules.entry(b).or_insert(vec![]);
            entry.push(a);
        }
    }
    let mut updates = Vec::new();
    for update in lines {
        updates.push(
            update
                .split(',')
                .map(|s| s.parse::<u16>().unwrap())
                .collect::<Vec<_>>(),
        );
    }
    (rules, updates)
}

pub fn part2(input: String) -> Result<String> {
    let (rules, mut updates) = parse(input);
    let result: u16 = updates
        .iter_mut()
        .filter(|update| {
            let mut forbidden = HashSet::new();
            for val in update.iter() {
                if forbidden.contains(val) {
                    return true;
                }
                if let Some(forbidden_to_add) = rules.get(val) {
                    for forbid in forbidden_to_add {
                        forbidden.insert(forbid);
                    }
                }
            }
            false
        })
        .map(|update| {
            update.sort_by(|a, b| {
                if let Some(val) = rules.get(a) {
                    if val.contains(b) {
                        return Ordering::Greater;
                    }
                }
                if let Some(val) = rules.get(b) {
                    if val.contains(a) {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum();
    Ok(result.to_string())
}
