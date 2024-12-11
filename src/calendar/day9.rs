use crate::Solution;

type Num = i64;

pub fn solve(input: String) -> Solution {
    let mut memory: Vec<Num> = Vec::new();
    let mut test: Vec<(Num, Num)> = Vec::new();
    let mut file = true;
    let mut id = 0;

    input.split_terminator("").for_each(|x| {
        if x == "" {
            return;
        }

        let y = x.parse::<Num>().unwrap();
        for _ in 0..y {
            if file {
                memory.push(id);
            } else {
                memory.push(-1);
            }
        }

        if file {
            test.push((id, y));
            id += 1;
        } else {
            test.push((-1, y));
        }

        file = !file;
    });

    println!("{:?}", test);
    let mut memory2 = memory.clone();

    let mut right_ptr = memory.len() - 1;
    for i in 0..=right_ptr {
        if memory[i] == -1 {
            while memory[right_ptr] == -1 {
                right_ptr -= 1;
            }
            if i > right_ptr {
                break;
            }
            memory.swap(i, right_ptr);
        }
    }

    let mut right_ptr = memory2.len() - 1;
    let mut file_space = 0;

    while right_ptr > 0 {
        let mut swapped = false;
        for i in 0..=right_ptr {
            let mut free_space = 0;
            while i + free_space < memory2.len() && memory2[i + free_space] == -1 {
                free_space += 1;
            }

            if free_space == 0 {
                continue;
            }
            while right_ptr > 0 && memory2[right_ptr] == -1 {
                right_ptr -= 1;
            }

            if i > right_ptr {
                continue;
            }
            file_space = 0;
            let file_id = memory2[right_ptr];
            while memory2[right_ptr - file_space] == file_id {
                file_space += 1;
            }

            // println!("Free : {free_space}, file : {file_space}, {file_id}, pos: {i}, pointer: {right_ptr}");
            // println!("{memory2:?}");

            if free_space >= file_space {
                for j in 0..file_space {
                    memory2.swap(i + j, right_ptr - j);
                }
                swapped = true;
            }
        }
        if !swapped {
            right_ptr -= file_space;
        }
    }

    // println!("Finished : {memory2:?}");

    let mut res1: i128 = 0;
    let mut res2: i128 = 0;
    for i in 0..memory.len() {
        if memory[i] != -1 {
            res1 += i as i128 * memory[i] as i128;
        }
        if memory2[i] != -1 {
            res2 += i as i128 * memory2[i] as i128;
        }
    }

    Solution(res1.to_string(), res2.to_string())
}
// Wrong : 7289235768529
