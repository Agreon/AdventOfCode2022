use std::collections::HashSet;

static INPUT: &'static str = include_str!("input.txt");

// ~1.8ms
pub fn part_one() -> u64 {
    let lines: Vec<_> = INPUT
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, steps)| (dir, steps.parse::<u8>().unwrap()))
        .collect();

    let grid_width = 600;
    let grid_height = 600;
    let mut visited_distinct = 1;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid_width]; grid_height];

    let mut head_x = grid_width / 2;
    let mut head_y = grid_height / 2;

    let mut tail_x = head_x;
    let mut tail_y = head_y;

    visited[tail_x][tail_y] = true;

    for (dir, steps) in lines {
        for _ in 0..steps {
            match dir {
                "L" => head_x -= 1,
                "R" => head_x += 1,
                "U" => head_y -= 1,
                "D" => head_y += 1,
                _ => panic!("Unexpected direction"),
            }

            let diff_x: i32 = head_x as i32 - tail_x as i32;
            let diff_y: i32 = head_y as i32 - tail_y as i32;

            if diff_x > 1 || diff_x < -1 || diff_y > 1 || diff_y < -1 {
                if diff_x > 0 {
                    tail_x += 1;
                } else if diff_x < 0 {
                    tail_x -= 1;
                }

                if diff_y > 0 {
                    tail_y += 1;
                } else if diff_y < 0 {
                    tail_y -= 1;
                }

                if visited[tail_x][tail_y] == false {
                    visited[tail_x][tail_y] = true;
                    visited_distinct += 1;
                }
            }
        }
    }

    return visited_distinct;
}

#[derive(Clone)]
struct Part {
    x: usize,
    y: usize,
}

// Idea: every part before the tail is itself a tail of the next part.
// ~4.2ms
pub fn part_two() -> u64 {
    let lines: Vec<_> = INPUT
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, steps)| (dir, steps.parse::<u8>().unwrap()))
        .collect();

    let grid_width = 600;
    let grid_height = 600;
    let mut visited_distinct = 1;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid_width]; grid_height];

    let mut head_x = grid_width / 2;
    let mut head_y = grid_height / 2;

    let mut parts: Vec<Part> = vec![
        Part {
            x: head_x,
            y: head_y
        };
        9
    ];

    visited[head_x as usize][head_y as usize] = true;

    for (dir, steps) in lines {
        for _ in 0..steps {
            match dir {
                "L" => head_x -= 1,
                "R" => head_x += 1,
                "U" => head_y -= 1,
                "D" => head_y += 1,
                _ => panic!("Unexpected direction"),
            }

            let mut previous = (head_x, head_y);

            for part in &mut parts {
                let diff_x: i32 = previous.0 as i32 - part.x as i32;
                let diff_y: i32 = previous.1 as i32 - part.y as i32;

                if diff_x == 2 || diff_x == -2 || diff_y == 2 || diff_y == -2 {
                    if diff_x > 0 {
                        part.x += 1;
                    } else if diff_x < 0 {
                        part.x -= 1;
                    }

                    if diff_y > 0 {
                        part.y += 1;
                    } else if diff_y < 0 {
                        part.y -= 1;
                    }
                }

                previous = (part.x, part.y);
            }

            let tail = &parts[parts.len() - 1];
            if visited[tail.x][tail.y] == false {
                visited[tail.x][tail.y] = true;
                visited_distinct += 1;
            }
        }
    }

    println!("{:?}", visited_distinct);

    return visited_distinct;
}
