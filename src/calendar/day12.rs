use itertools::*;
use std::collections::HashSet;

use crate::Solution;

type Num = i32;

const NEIGHBORS: [(Num, Num); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn fences(
    pos: (Num, Num),
    plant_type: &str,
    plants: &Vec<Vec<&str>>,
    visited: &mut HashSet<(Num, Num)>,
    vertices: &mut Vec<(Num, Num)>,
    perimeter: &mut Num,
    area: &mut Num,
) {
    if !visited.contains(&pos) {
        *area += 1;
        visited.insert(pos);

        let mut convex: Vec<(Num, Num)> = Vec::new();
        let mut concave: Vec<(Num, Num)> = Vec::new();

        for offset in NEIGHBORS {
            let np = (pos.0 + offset.0, pos.1 + offset.1);

            if np.0 < 0
                || np.1 < 0
                || np.0 >= plants[0].len() as Num
                || np.1 >= plants.len() as Num
                || plants[np.1 as usize][np.0 as usize] != plant_type
            {
                *perimeter += 1;
                convex.push(offset);
            } else {
                concave.push(offset);
                fences(
                    (pos.0 + offset.0, pos.1 + offset.1),
                    plant_type,
                    plants,
                    visited,
                    vertices,
                    perimeter,
                    area,
                );
            }
        }

        for (c1, c2) in convex.iter().tuple_combinations() {
            let tp = (pos.0 + c1.0 + c2.0, pos.1 + c1.1 + c2.1);
            if tp != pos {
                // println!("Points : {:?}, plant : {}, convex", tp, plant_type);
                vertices.push(tp);
            }
        }

        for (c1, c2) in concave.iter().tuple_combinations() {
            let tp = (pos.0 + c1.0 + c2.0, pos.1 + c1.1 + c2.1);
            if tp != pos && tp.0 < 0
                || tp.1 < 0
                || tp.0 >= plants[0].len() as Num
                || tp.1 >= plants.len() as Num
                || plants[tp.1 as usize][tp.0 as usize] != plant_type
            {
                // println!("Points : {:?}, plant : {}, concave", tp, plant_type);
                vertices.push(tp);
            }
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut visited: HashSet<(Num, Num)> = HashSet::new();
    let plants: Vec<Vec<&str>> = input
        .split_terminator('\n')
        .map(|l| l.split_terminator("").filter(|x| *x != "").collect())
        .collect();

    let mut res1 = 0;
    let mut res2 = 0;

    for y in 0..plants.len() {
        for x in 0..plants[0].len() {
            let pos = (x as Num, y as Num);
            if !visited.contains(&pos) {
                let mut perimeter = 0;
                let mut area = 0;

                let mut vertices: Vec<(Num, Num)> = Vec::new();

                fences(
                    pos,
                    plants[y][x],
                    &plants,
                    &mut visited,
                    &mut vertices,
                    &mut perimeter,
                    &mut area,
                );

                // println!(
                //     "Area : {area}, per : {perimeter}, plant : {}, sides : {}",
                //     plants[y][x],
                //     vertices.len()
                // );

                res2 += area * vertices.len() as Num;
                res1 += perimeter * area;
            }
        }
    }

    Solution(res1.to_string(), res2.to_string())
}
