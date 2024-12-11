use std::collections::HashMap;

use crate::Solution;

type Num = i64;

const MAX:Num = 75;

pub fn move_rock(rock: Num, cache: &mut HashMap<(Num, Num), Num>, depth: Num) -> Num {
    if depth >= MAX {
        1
    } else if let Some(v) = cache.get(&(rock, depth)) {
        *v
    } else {
        let cast = rock as f64;
        let nb_digits = cast.log10() as u32 + 1;
        let nb = if rock == 0 {
            move_rock(1, cache, depth + 1)
        } else if nb_digits % 2 == 0 {
            let n = (10 as Num).pow(nb_digits / 2);

            let a = rock / n;
            let b = rock - (a * n);

            move_rock(a, cache, depth + 1) + move_rock(b, cache, depth + 1)
        } else {
            move_rock(rock * 2024, cache, depth + 1)
        };
        cache.insert((rock, depth), nb);
        nb
    }
}

pub fn solve(input: String) -> Solution {
    let og: Vec<Num> = input
        .split_terminator(' ')
        .map(|l| l.parse().unwrap())
        .collect();

    let mut stones = og.clone();
    let mut cache: HashMap<(Num, Num), Num> = HashMap::new();

    for _ in 0..25 {
        let mut to_insert: Vec<Num> = Vec::new();
        for v in stones.iter_mut() {
            let cast = *v as f64;
            let nb_digits = cast.log10() as u32 + 1;
            if *v == 0 {
                *v = 1;
            } else if nb_digits % 2 == 0 {
                let n = (10 as Num).pow(nb_digits / 2);

                let a = *v / n;
                let b = *v - (a * n);

                *v = a;
                to_insert.push(b);
            } else {
                *v *= 2024;
            }
        }
        stones.extend(to_insert);
    }

    let res1 = stones.len();

    let mut res2 = 0;
    for v in og {
        res2 += move_rock(v, &mut cache, 0);
    }

    Solution(res1.to_string(), res2.to_string())
}