use crate::Solution;

pub fn has_solution(result: &i128, mut nbs: Vec<i128>, res_so_far: i128, part2: bool) -> bool {
    if nbs.is_empty() {
        return res_so_far == *result;
    }

    if res_so_far > *result {
        return false;
    }

    let n = nbs.pop().unwrap();

    has_solution(result, nbs.clone(), res_so_far + n, part2)
        || has_solution(result, nbs.clone(), res_so_far * n, part2)
        || (part2 && has_solution(result, nbs, res_so_far * (10 as i128).pow((n as f32 + 1.0).log10().ceil() as u32) + n, part2))
}

pub fn solve(input: String) -> Solution {
    let equations: Vec<(i128, Vec<i128>)> = input
        .split_terminator('\n')
        .map(|l| {
            let v: Vec<&str> = l.split_terminator(": ").collect();
            (
                v[0].parse().unwrap(),
                v[1].split(' ').map(|c| c.parse().unwrap()).collect(),
            )
        })
        .collect();

    let n = 45;
    let a: i128 = 123;
    println!("{a} {n} : {}", a* (10 as i128).pow((n as f64).log10().floor() as u32 + 1) + n);

    let mut res1: i128 = 0;
    let mut res2: i128 = 0;

    for (res, mut nbs) in equations {
        nbs.reverse();
        let n = nbs.pop().unwrap();
        if has_solution(&res, nbs.clone(), n, false) {
            res1 += res;
        }
        if has_solution(&res, nbs, n, true) {
            res2 += res;
        }
    }

    Solution(res1.to_string(), res2.to_string())
}