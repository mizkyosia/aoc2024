
use crate::Solution;

pub fn solve(input: String) -> Solution {
    let lines: Vec<Vec<&str>> = input
        .split_terminator('\n')
        .map(|l| l.split_terminator("   ").collect())
        .collect();

    let mut val1: Vec<i32> = Vec::new();
    let mut val2: Vec<i32> = Vec::new();

    for line in lines {
        val1.push(line[0].parse().unwrap());
        val2.push(line[1].parse().unwrap());
    }

    let mut nb = 0;

    val1.sort();
    val2.sort();

    for i in 0..val1.len() {
        nb += (val1[i] - val2[i]).abs();
    }

    // ------------------------- PART 2 -------------------------

    let res = val1
        .iter()
        .map(|x| *x * val2.iter().filter(|y| **y == *x).count() as i32)
        .reduce(|x, y| x + y)
        .unwrap();

    Solution(nb.to_string(), res.to_string())
}
