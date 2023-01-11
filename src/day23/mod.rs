static INPUT: &str = include_str!("input.txt");

/**
 * Naive: Just 2D-Map with values
 * => Has problem if map is increased much
 * => Find max and mins requires whole map-scan
 *
 * Alternative:
 * => Save Positions of Elves in Data-Structure like Quad-Tree
 * => Reuse if we already found a position
 */

#[derive(Clone)]
struct Position {
    pub x: usize,
    pub y: usize,
}

struct Elf {
    pub position: Position,
    pub target: Option<Position>,
}

enum Direction {
    North,
    South,
    West,
    East,
}

pub fn move_elves(input: &str, iterations: usize) -> usize {
    let lines: Vec<_> = input.lines().collect();

    let width = lines[0].len();
    let height = lines.len();

    let map_width = width + (iterations * 2);
    let map_height = height + (iterations * 2);

    let mut map = vec![vec![false; map_width]; map_height];

    let mut elves: Vec<Elf> = vec![];

    for y in 0..lines.len() {
        for (x, character) in lines[y].chars().enumerate() {
            if character == '#' {
                map[y + iterations][x + iterations] = true;
                elves.push(Elf {
                    position: Position {
                        x: x + iterations,
                        y: y + iterations,
                    },
                    target: None,
                })
            }
        }
    }

    let mut directions = Vec::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    for _ in 0..iterations {
        let mut targets = vec![vec![0; map_width]; map_height];

        for elf in &mut elves {
            let position = &elf.position;

            let neighbors = [
                map[position.y - 1][position.x - 1],
                map[position.y - 1][position.x],
                map[position.y - 1][position.x + 1],
                map[position.y][position.x + 1],
                map[position.y + 1][position.x + 1],
                map[position.y + 1][position.x],
                map[position.y + 1][position.x - 1],
                map[position.y][position.x - 1],
            ];

            if !neighbors.iter().any(|neighbor| *neighbor) {
                elf.target = None;
                continue;
            }

            let mut found_direction: Option<Direction> = None;
            for direction in &directions {
                match direction {
                    Direction::North => {
                        if !neighbors[0] && !neighbors[1] && !neighbors[2] {
                            found_direction = Some(Direction::North);
                            break;
                        }
                    }
                    Direction::East => {
                        if !neighbors[2] && !neighbors[3] && !neighbors[4] {
                            found_direction = Some(Direction::East);
                            break;
                        }
                    }
                    Direction::South => {
                        if !neighbors[4] && !neighbors[5] && !neighbors[6] {
                            found_direction = Some(Direction::South);
                            break;
                        }
                    }
                    Direction::West => {
                        if !neighbors[6] && !neighbors[7] && !neighbors[0] {
                            found_direction = Some(Direction::West);
                            break;
                        }
                    }
                }
            }

            let target = match found_direction {
                None => continue,
                Some(Direction::North) => Position {
                    x: position.x,
                    y: position.y - 1,
                },
                Some(Direction::East) => Position {
                    x: position.x + 1,
                    y: position.y,
                },
                Some(Direction::South) => Position {
                    x: position.x,
                    y: position.y + 1,
                },
                Some(Direction::West) => Position {
                    x: position.x - 1,
                    y: position.y,
                },
            };

            targets[target.y][target.x] += 1;
            elf.target = Some(target);
        }

        if elves.iter().all(|elf| elf.target.is_none()) {
            break;
        }

        // Move to targets
        for elf in &mut elves {
            match &elf.target {
                None => continue,
                Some(target) => {
                    if targets[target.y][target.x] == 1 {
                        map[elf.position.y][elf.position.x] = false;
                        map[target.y][target.x] = true;
                        elf.position = target.clone();
                    }
                }
            }
        }

        directions.rotate_left(1);
    }

    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;

    for elf in &elves {
        max_x = elf.position.x.max(max_x);
        max_y = elf.position.y.max(max_y);

        min_x = elf.position.x.min(min_x);
        min_y = elf.position.y.min(min_y);
    }

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let free_tiles = (width * height) - elves.len();

    println!("{free_tiles}");

    free_tiles
}

pub fn part_one() -> usize {
    move_elves(INPUT, 10)
}

pub fn move_elves_until_finished(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    let width = lines[0].len();
    let height = lines.len();

    let size_inc = 100;

    let map_width = width + (size_inc * 2);
    let map_height = height + (size_inc * 2);

    let mut map = vec![vec![false; map_width]; map_height];

    let mut elves: Vec<Elf> = vec![];

    for y in 0..lines.len() {
        for (x, character) in lines[y].chars().enumerate() {
            if character == '#' {
                map[y + size_inc][x + size_inc] = true;
                elves.push(Elf {
                    position: Position {
                        x: x + size_inc,
                        y: y + size_inc,
                    },
                    target: None,
                })
            }
        }
    }

    let mut directions = Vec::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    let mut iterations = 0;
    loop {
        let mut targets = vec![vec![0; map_width]; map_height];

        for elf in &mut elves {
            let position = &elf.position;

            let neighbors = [
                map[position.y - 1][position.x - 1],
                map[position.y - 1][position.x],
                map[position.y - 1][position.x + 1],
                map[position.y][position.x + 1],
                map[position.y + 1][position.x + 1],
                map[position.y + 1][position.x],
                map[position.y + 1][position.x - 1],
                map[position.y][position.x - 1],
            ];

            if !neighbors.iter().any(|neighbor| *neighbor) {
                elf.target = None;
                continue;
            }

            let mut found_direction: Option<Direction> = None;
            for direction in &directions {
                match direction {
                    Direction::North => {
                        if !neighbors[0] && !neighbors[1] && !neighbors[2] {
                            found_direction = Some(Direction::North);
                            break;
                        }
                    }
                    Direction::East => {
                        if !neighbors[2] && !neighbors[3] && !neighbors[4] {
                            found_direction = Some(Direction::East);
                            break;
                        }
                    }
                    Direction::South => {
                        if !neighbors[4] && !neighbors[5] && !neighbors[6] {
                            found_direction = Some(Direction::South);
                            break;
                        }
                    }
                    Direction::West => {
                        if !neighbors[6] && !neighbors[7] && !neighbors[0] {
                            found_direction = Some(Direction::West);
                            break;
                        }
                    }
                }
            }

            let target = match found_direction {
                None => continue,
                Some(Direction::North) => Position {
                    x: position.x,
                    y: position.y - 1,
                },
                Some(Direction::East) => Position {
                    x: position.x + 1,
                    y: position.y,
                },
                Some(Direction::South) => Position {
                    x: position.x,
                    y: position.y + 1,
                },
                Some(Direction::West) => Position {
                    x: position.x - 1,
                    y: position.y,
                },
            };

            targets[target.y][target.x] += 1;
            elf.target = Some(target);
        }

        if elves.iter().all(|elf| elf.target.is_none()) {
            break;
        }

        // Move to targets
        for elf in &mut elves {
            match &elf.target {
                None => continue,
                Some(target) => {
                    if targets[target.y][target.x] == 1 {
                        map[elf.position.y][elf.position.x] = false;
                        map[target.y][target.x] = true;
                        elf.position = target.clone();
                    }
                }
            }
        }

        directions.rotate_left(1);

        iterations += 1;
    }

    iterations + 1
}

pub fn part_two() -> usize {
    move_elves_until_finished(INPUT)
}

#[cfg(test)]
mod tests {
    use super::{move_elves, move_elves_until_finished};
    static INPUT_TEST: &str = include_str!("input-test-2.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one_test_data() {
        assert_eq!(move_elves(INPUT_TEST, 10), 110)
    }

    #[test]
    fn test_part_one() {
        assert_eq!(move_elves(INPUT, 10), 4208)
    }

    #[test]
    fn test_part_two_test_data() {
        assert_eq!(move_elves_until_finished(INPUT_TEST), 20)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(move_elves_until_finished(INPUT), 1016)
    }
}
