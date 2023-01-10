use std::{str::FromStr, string::ParseError, time::Instant};

static INPUT: &str = include_str!("input.txt");

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

enum Turn {
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }
}

fn parse_instructions(input: &str) -> (Vec<usize>, Vec<Turn>) {
    let mut steps = Vec::with_capacity(input.len());
    let mut turns = Vec::with_capacity(input.len());

    let mut current_steps: usize = 0;
    for character in input.chars() {
        if character.is_ascii_digit() {
            current_steps = (current_steps * 10) + character.to_digit(10).unwrap() as usize;
        } else {
            if character == 'L' {
                turns.push(Turn::Left)
            } else {
                turns.push(Turn::Right)
            };

            steps.push(current_steps);

            current_steps = 0;
        }
    }

    if current_steps > 0 {
        steps.push(current_steps);
    }

    (steps, turns)
}

#[derive(PartialEq)]
enum Field {
    Floor,
    Wall,
}

impl Field {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Field::Floor,
            '#' => Field::Wall,
            _ => unimplemented!(),
        }
    }
}

struct Row {
    pub start: usize,
    pub end: usize,
    pub values: Vec<Field>,
}

impl FromStr for Row {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut found_start = false;
        let mut start = 0;
        let end = s.len() - 1;
        let mut values = Vec::with_capacity(s.len());

        for (i, character) in s.chars().enumerate() {
            if character != ' ' {
                if !found_start {
                    start = i;
                    found_start = true;
                }

                values.push(Field::from_char(character));
            }
        }

        Ok(Row { start, end, values })
    }
}

struct Position {
    pub x: usize,
    pub y: usize,
}

/**
* Einfach 2D Array mit None, Way oder Hindernis als Pixel
* Optimierungs-Idee 2:
* - Double-LinkedList nur mit Hindernissen für jede row und spalte
* => Dann muss man nicht jeden Pixel überprüfen
* => Und auch nicht schauen ob man das Ende erreicht hat
* - => Funktioniert nur, wenn es keine Räume mit Aussparungen dazwischen gibt
*
* Räume mit Aussparungen machen eigentlich bei dem Wrap-Anforderungen keinen Sinn
*
* =>
*/
pub fn part_one() -> usize {
    move_in_2d(INPUT)
}

// 2.4ms
pub fn move_in_2d(input: &str) -> usize {
    let (map, instructions) = input.split_once("\n\n").unwrap();

    let rows: Vec<Row> = map.lines().map(|line| line.parse().unwrap()).collect();

    let (steps, turns) = parse_instructions(instructions);

    let mut current_position = Position {
        x: rows[0].start,
        y: 0,
    };
    let mut current_direction = Direction::Right;

    for (i, steps) in steps.iter().enumerate() {
        let remaining_moves = *steps;

        match current_direction {
            Direction::Right => {
                let current_row = &rows[current_position.y];

                for _ in 0..remaining_moves {
                    let next_x = if current_position.x == current_row.end {
                        current_row.start
                    } else {
                        current_position.x + 1
                    };

                    match &current_row.values[next_x - current_row.start] {
                        Field::Floor => current_position.x = next_x,
                        Field::Wall => break,
                    }
                }
            }
            Direction::Left => {
                let current_row = &rows[current_position.y];
                for _ in 0..remaining_moves {
                    // TODO: Maybe just make add method for row and clamp at edges?
                    let next_x = if current_position.x == current_row.start {
                        current_row.end
                    } else {
                        current_position.x - 1
                    };

                    match &current_row.values[next_x - current_row.start] {
                        Field::Floor => current_position.x = next_x,
                        Field::Wall => break,
                    }
                }
            }
            Direction::Down => {
                for _ in 0..remaining_moves {
                    let mut next_y = if current_position.y == rows.len() - 1 {
                        0
                    } else {
                        current_position.y + 1
                    };

                    if current_position.x < rows[next_y].start {
                        let mut wrap_idx = 0;
                        // TODO: We need columns if we don't want to iterate
                        for (j, row) in rows.iter().enumerate() {
                            if row.start <= current_position.x {
                                wrap_idx = j;
                                break;
                            }
                        }
                        next_y = wrap_idx;
                    } else if current_position.x > rows[next_y].end {
                        let mut wrap_idx = 0;
                        // TODO: We need columns if we don't want to iterate
                        for (j, row) in rows.iter().enumerate() {
                            if row.end >= current_position.x {
                                wrap_idx = j;
                                break;
                            }
                        }
                        next_y = wrap_idx;
                    }

                    match &rows[next_y].values[current_position.x - rows[next_y].start] {
                        Field::Floor => current_position.y = next_y,
                        Field::Wall => break,
                    }
                }
            }
            Direction::Up => {
                for _ in 0..remaining_moves {
                    let mut next_y = if current_position.y == 0 {
                        rows.len() - 1
                    } else {
                        current_position.y - 1
                    };

                    if current_position.x < rows[next_y].start {
                        let mut wrap_idx = 0;
                        // TODO: We need columns if we don't want to iterate
                        for j in (0..rows.len()).rev() {
                            if rows[j].start <= current_position.x {
                                wrap_idx = j;
                                break;
                            }
                        }
                        next_y = wrap_idx;
                    } else if current_position.x > rows[next_y].end {
                        let mut wrap_idx = 0;
                        // TODO: We need columns if we don't want to iterate
                        for j in (0..rows.len()).rev() {
                            if rows[j].end >= current_position.x {
                                wrap_idx = j;
                                break;
                            }
                        }
                        next_y = wrap_idx;
                    }

                    match &rows[next_y].values[current_position.x - rows[next_y].start] {
                        Field::Floor => current_position.y = next_y,
                        Field::Wall => break,
                    }
                }
            }
        }

        // We have one more step than turn
        if let Some(turn) = turns.get(i) {
            match turn {
                Turn::Left => current_direction = current_direction.turn_left(),
                Turn::Right => current_direction = current_direction.turn_right(),
            }
        }
    }

    let mut result = ((current_position.y + 1) * 1000) + ((current_position.x + 1) * 4);
    match current_direction {
        Direction::Right => result += 0,
        Direction::Down => result += 1,
        Direction::Left => result += 2,
        Direction::Up => result += 3,
    }

    result
}

#[cfg(test)]
mod tests {
    use super::move_in_2d;
    static INPUT_TEST: &str = include_str!("input-test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one_test_data() {
        assert_eq!(move_in_2d(INPUT_TEST), 6032)
    }

    #[test]
    fn test_part_one() {
        assert_eq!(move_in_2d(INPUT), 191010)
    }
}
