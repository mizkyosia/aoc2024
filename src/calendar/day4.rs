use crate::Solution;

const OFFSETS: [[i32; 2]; 8] = [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0],
    [1, 1],
    [1, -1],
    [-1, 1],
    [-1, -1],
];

pub fn check_line(
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    res: &mut i32,
    chars: &Vec<Vec<&str>>,
    mut to_match: Vec<&str>,
) {
    let ch = to_match.pop().unwrap();

    let x = x + dx;
    let y = y + dy;

    if x < 0 || y < 0 || x >= chars[0].len() as i32 || y >= chars.len() as i32 {
        return;
    }

    if ch == chars[y as usize][x as usize] {
        if !to_match.is_empty() {
            check_line(x, y, dx, dy, res, chars, to_match.clone());
        } else {
            *res += 1;
        }
    }
}

pub fn solve(input: String) -> Solution {
    let chars: Vec<Vec<&str>> = input
        .split('\n')
        .map(|l| l.split("").filter(|c| c.len() > 0).collect())
        .collect();

    let mut res1 = 0;
    let mut res2 = 0;

    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            match chars[y][x] {
                "X" => {
                    for [dx, dy] in OFFSETS {
                        check_line(
                            x as i32,
                            y as i32,
                            dx,
                            dy,
                            &mut res1,
                            &chars,
                            vec!["S", "A", "M"],
                        );
                    }
                }
                "A" => {
                    if (x > 0 && y > 0 && x < chars[y].len() - 1 && y < chars.len() - 1)
                        && ((chars[y - 1][x - 1] == "M" && chars[y + 1][x + 1] == "S")
                            || (chars[y + 1][x + 1] == "M" && chars[y - 1][x - 1] == "S"))
                        && ((chars[y - 1][x + 1] == "M" && chars[y + 1][x - 1] == "S")
                            || (chars[y + 1][x - 1] == "M" && chars[y - 1][x + 1] == "S"))
                    {
                        res2 += 1;
                    }
                }
                _ => {}
            }
        }
    }

    Solution(res1.to_string(), res2.to_string())
}
