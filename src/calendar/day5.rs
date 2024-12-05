use std::collections::HashSet;

use crate::Solution;

pub fn check_valid(update: &Vec<usize>, pairs: &HashSet<(usize, usize)>) -> bool {
    for i in 0..update.len() {
        for j in (i + 1)..update.len() {
            if !pairs.contains(&(update[i], update[j])) && pairs.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }
    true
}

pub fn solve_tree(
    o_update: Vec<usize>,
    node: Vec<usize>,
    pairs: &HashSet<(usize, usize)>,
    debug: i32,
) -> Option<usize> {
    for i in 0..o_update.len() {
        let mut nu = o_update.clone();
        let mut nn = node.clone();

        nn.push(nu[i]);
        nu.remove(i);

        if !check_valid(&nn, pairs) {
            return None;
        }

        if nu.len() == 0 {
            return Some(nn[nn.len() / 2]);
        }

        let next = solve_tree(nu, nn, pairs, debug + 1);
        if next.is_some() {
            return next;
        }
    }
    None
}

pub fn solve(input: String) -> Solution {
    let parsed: Vec<&str> = input.split("\n\n").collect();

    let pairs: HashSet<(usize, usize)> = parsed[0]
        .split('\n')
        .map(|l| {
            let p: Vec<&str> = l.split('|').collect();
            (p[0].parse().unwrap(), p[1].parse().unwrap())
        })
        .collect();

    let updates: Vec<Vec<usize>> = parsed[1]
        .split('\n')
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut res1 = 0;
    let mut res2 = 0;

    for update in updates {
        if check_valid(&update, &pairs) {
            res1 += update[update.len() / 2];
        } else {
            res2 += solve_tree(update.clone(), Vec::new(), &pairs, 0).unwrap();
        }
    }

    Solution(res1.to_string(), res2.to_string())
}
