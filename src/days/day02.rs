use std::fmt::format;

use anyhow::Result;

pub fn part1(input: String) -> Result<String> {
    let lists = parse(input);
    let result: u16 = lists
        .iter()
        .map(|x| {
            let (safe, increase) = is_safe(&x[0], &x[1]);
            if !safe {
                return 0;
            }
            for i in 1..(x.len() - 1) {
                let (safe, new_increase) = is_safe(&x[i], &x[i + 1]);
                if !safe || increase != new_increase {
                    return 0;
                }
            }
            1
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

pub fn part2(input: String) -> Result<String> {
    let lists = parse(input);
    let result: Vec<bool> = lists
        .iter()
        .map(|x| {
            // check first and second
            let look = format!("{:?}", x);
            let (safe, mut increase) = is_safe(&x[0], &x[1]);
            let (safe2, increase2) = is_safe(&x[1], &x[2]);
            let mut skip = false;
            let mut damp = true;
            if !safe {
                if safe2 {
                    increase = increase2
                }
                damp = false;
            }
            // [77 74 78 80]
            // 77 -> 5 = 1
            // 5 -> 9 = 2
            // 9 -> 10 = 3
            if increase != increase2 {
                let (_, increase3) = is_safe(&x[2], &x[3]);
                if increase == increase3 {
                    let (safe2_5, _) = is_safe(&x[0], &x[2]);
                    if safe2_5 {
                        damp = false;
                        skip = true;
                    } else {
                        return false;
                    }
                } else {
                    increase = increase3;
                    damp = false;
                }
            } else if !safe2 {
                let (safe2_5, _) = is_safe(&x[0], &x[2]);
                if safe2_5 {
                    damp = false;
                    skip = true;
                } else {
                    return false;
                }
            }

            // check the rest
            for i in 1..(x.len() - 1) {
                if skip {
                    skip = false;
                    continue;
                }
                let (safe, new_increase) = is_safe(&x[i], &x[i + 1]);
                if safe && increase == new_increase {
                    continue;
                }
                if damp {
                    // can we skip current index one?
                    // :(
                    // I check -1 and +1 to see if I can skip the current one
                    // [-1 ,current +1]
                    // But the next one can check it himself
                    // more for you
                    //                   r
                    // [-2, -1, current, +1, +2]
                    // probably
                    // @Geode <3 yes I get it now <3
                    let (safe, new_increase) = is_safe(&x[i - 1], &x[i + 1]);
                    if safe && increase == new_increase || i >= x.len() - 2 {
                        damp = false;
                        continue;
                    }
                    let (safe, new_increase) = is_safe(&x[i], &x[i + 2]);
                    if safe && increase == new_increase {
                        damp = false;
                        skip = true;
                        continue;
                    }
                    return false;
                } else {
                    return false;
                }
            }
            // println!("{:?}", x);
            true
        })
        .collect();
    println!("{:?}", result);
    Ok(result.iter().filter(|x| **x).count().to_string())
}
