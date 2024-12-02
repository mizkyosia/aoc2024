use crate::Solution;

pub fn solve(input: String) -> Solution {
    let mut res2: Vec<bool> = Vec::new();
    let res1: i32 = input
        .split_terminator('\n')
        .map(|l| {
            let v: Vec<i32> = l
                .split_terminator(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let r1 = test_sequence(&v);

            if !r1.0 {
                for i in -1..=1 {
                    let i = (r1.1 + i) as usize;
                    if i >= v.len() {
                        continue;
                    }

                    let mut v1 = v.clone();
                    v1.remove(i);

                    let r2 = test_sequence(&v1);

                    if r2.0 {
                        res2.push(true);
                        return r1.0;
                    }
                }
            }

            res2.push(r1.0);
            r1.0
        })
        .fold(0, |a, b| a + b as i32);

    Solution(
        res1.to_string(),
        res2.iter().fold(0, |x, y| x + *y as i32).to_string()
    )
}

pub fn test_sequence(v: &Vec<i32>) -> (bool, i32) {
    let s = v[0] - v[v.len() - 1] > 0;

    for i in 0..(v.len() - 1) {
        let d = v[i] - v[i + 1];
        if d.abs() < 1 || d.abs() > 3 || (d > 0) != s {
            return (false, i as i32);
        }
    }
    (true, 0)
}
