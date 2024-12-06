use std::collections::HashSet;

use crate::Solution;

pub fn rotate(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => panic!("Invalid direction"),
    }
}

pub fn check_loop(
    mut visited: HashSet<((i32, i32), (i32, i32))>,
    grid: Vec<Vec<u32>>,
    start_pos: (i32, i32),
    start_dir: (i32, i32),
) -> bool {
    // let mut pos = (start_pos.0 + start_dir.0, start_pos.1 + start_dir.1) ;
    let mut pos = start_pos;
    let mut dir = start_dir;

    while pos.0 >= 0 && pos.1 >= 0 && pos.0 < grid[0].len() as i32 && pos.1 < grid.len() as i32 {
        if visited.contains(&(pos, dir)) {
            return true;
        }

        visited.insert((pos, dir));

        if pos.0 + dir.0 >= 0
            && pos.1 + dir.1 >= 0
            && pos.0 + dir.0 < grid[0].len() as i32
            && pos.1 + dir.1 < grid.len() as i32
            && grid[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] == 1
        {
            dir = rotate(dir);
        } else {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
    }
    false
}

pub fn solve(input: String) -> Solution {
    let mut start_pos: (usize, usize) = (0, 0);
    let mut y: i32 = -1;

    let grid: Vec<Vec<u32>> = input
        .split_terminator('\n')
        .map(|l| {
            y += 1;
            let mut x: i32 = -1;
            l.split_terminator("")
                .map(|c| {
                    x += 1;
                    match c {
                        "#" => 1,
                        "^" => {
                            start_pos = (x as usize, y as usize);
                            0
                        }
                        _ => 0,
                    }
                })
                .collect()
        })
        .collect();

    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut visits: HashSet<(i32, i32)> = HashSet::new();

    println!("Start position : {start_pos:?}");
    let mut pos = (start_pos.0 as i32, start_pos.1 as i32);
    let mut dir: (i32, i32) = (0, -1);

    let mut res2 = 0;

    while pos.0 >= 0 && pos.1 >= 0 && pos.0 < grid[0].len() as i32 && pos.1 < grid.len() as i32 {
        visited.insert((pos, dir));
        visits.insert(pos);

        if pos.0 + dir.0 >= 0
            && pos.1 + dir.1 >= 0
            && pos.0 + dir.0 < grid[0].len() as i32
            && pos.1 + dir.1 < grid.len() as i32
            && grid[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] == 1
        {
            dir = rotate(dir);
        } else {
            // let mut g = grid.clone();
            // g[pos.1 as usize][pos.0 as usize] = 1;

            // if check_loop(visited.clone(), g, pos, rotate(dir)) {
            //     res2 += 1;
            // }
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
    }

    for i in visits.iter() {
        let mut g = grid.clone();
        g[i.1 as usize][i.0 as usize] = 1;

        if check_loop(HashSet::new(), g, (start_pos.0 as i32, start_pos.1 as i32), (0, -1)) {
            res2 += 1;
        }
    }

    Solution(visits.len().to_string(), res2.to_string())
}
