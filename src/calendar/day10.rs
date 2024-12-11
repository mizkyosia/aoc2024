use std::collections::HashSet;

use crate::Solution;

type Num = i32;

const NEIGHBORS: [(Num, Num); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn pathfind(
    pos: (Num, Num),
    grid: &Vec<Vec<Num>>,
    res: &mut HashSet<(Num, Num)>,
    current: Num,
) -> Num {
    let mut counter = 0;
    for n in NEIGHBORS {
        let np = (pos.0 + n.0, pos.1 + n.1);
        if np.0 >= 0
            && np.1 >= 0
            && np.0 < grid[0].len() as i32
            && np.1 < grid.len() as i32
            && grid[np.1 as usize][np.0 as usize] == current + 1
        {
            // println!(
            //     "Positon : {np:?}, number : {}, current : {current}",
            //     grid[np.1 as usize][np.0 as usize]
            // );
            if current + 1 == 9 {
                res.insert(np);
                counter += 1;
            } else {
                counter += pathfind(np, grid, res, current + 1);
            }
        }
    }
    counter
}

pub fn solve(input: String) -> Solution {
    let mut starts: Vec<(Num, Num)> = Vec::new();
    let mut x: Num = 0;
    let mut y: Num = -1;

    let grid: Vec<Vec<Num>> = input
        .split_terminator('\n')
        .map(|l| {
            y += 1;
            x = 0;
            l.split_terminator("")
                .filter_map(|c| {
                    if c != "" {
                        let z = c.parse::<Num>().unwrap();
                        if z == 0 {
                            starts.push((x, y));
                        }
                        x += 1;
                        Some(z)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let mut res1 = 0;
    let mut res2 = 0;
    for point in starts.iter() {
        let mut set = HashSet::new();
        res2 += pathfind(*point, &grid, &mut set, 0);
        res1 += set.len();
    }

    Solution(res1.to_string(), res2.to_string())
}
