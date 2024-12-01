
use std::collections::HashMap;

use crate::Solution;

pub fn solve(input: String) -> Solution {
    let lines: Vec<Vec<&str>> = input
        .split_terminator('\n')
        .map(|l| l.split_terminator("   ").collect())
        .collect();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        left.push(line[0].parse().unwrap());
        right.push(line[1].parse().unwrap());
    }

    let mut res1 = 0;

    left.sort();
    right.sort();

    for i in 0..left.len() {
        res1 += (left[i] - right[i]).abs();
    }

    // ------------------------- PART 2 -------------------------

    let mut cache:HashMap<i32, i32> = HashMap::new();

    let res2 = left
        .iter()
        .map(|x| {
            if let Some(y) = cache.get(x) {
                *x * *y
            } else {
                let y = right.iter().filter(|y| **y == *x).count() as i32;
                cache.insert(*x, y);
                *x * y
            }
        })
        .reduce(|x, y| x + y)
        .unwrap();

    Solution(res1.to_string(), res2.to_string())
}
