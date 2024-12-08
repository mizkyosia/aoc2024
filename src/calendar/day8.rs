use std::collections::HashSet;

use crate::Solution;

pub fn solve(input: String) -> Solution {
    let mut width = 0;
    let mut height = 0;
    let mut nodes: Vec<(i32, i32, &str)> = Vec::new();

    input.split_terminator('\n').for_each(|l| {
        width = 0;
        l.split_terminator("").for_each(|a| {
            if a != "" {
                if a != "." && a != "" {
                    nodes.push((width, height, a));
                }
                width += 1;
            }
        });
        height += 1;
    });

    let mut antinodes_1: HashSet<(i32, i32)> = HashSet::new();
    let mut antinodes_2: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            if nodes[i].2 != nodes[j].2 {
                continue;
            }

            let offset_x = nodes[j].0 - nodes[i].0;
            let offset_y = nodes[j].1 - nodes[i].1;

            let mut k = 0;

            while nodes[i].0 - offset_x * k < width
                && nodes[i].0 - offset_x * k >= 0
                && nodes[i].1 - offset_y * k < height
                && nodes[i].1 - offset_y * k >= 0
            {
                let node = (nodes[i].0 - offset_x * k, nodes[i].1 - offset_y * k);
                if k == 1 {
                    antinodes_1.insert(node);
                }
                antinodes_2.insert(node);
                k += 1;
            }

            k = 0;

            while nodes[j].0 + offset_x * k < height
                && nodes[j].0 + offset_x * k >= 0
                && nodes[j].1 + offset_y * k < height
                && nodes[j].1 + offset_y * k >= 0
            {
                let node = (nodes[j].0 + offset_x * k, nodes[j].1 + offset_y * k);
                if k == 1 {
                    antinodes_1.insert(node);
                }
                antinodes_2.insert(node);
                k += 1;
            }
        }
    }

    Solution(antinodes_1.len().to_string(), antinodes_2.len().to_string())
}

// 375 : too high
