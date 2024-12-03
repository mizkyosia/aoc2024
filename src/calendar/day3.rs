
use regex::Regex;

use crate::Solution;

pub fn solve(input: String) -> Solution {
    let reg = Regex::new(r"(?:(don't)()|(do)()|mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();
    let mut res1: i32 = 0;
    let mut res2: i32 = 0;

    let mut enabled = true;



    for (_, [num1, num2]) in reg.captures_iter(&input).map(|c| c.extract()) {
        match num1 {
            "do" => enabled = true,
            "don't" => enabled = false,
            _ => {
                let v = num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
                res1 += v;
                if enabled {
                    res2 += v;
                } 
            }
        }
    }

    Solution(res1.to_string(), res2.to_string())
}
