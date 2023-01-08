use std::{iter::Cycle, slice::Iter, str::FromStr, string::ParseError};

static INPUT: &str = include_str!("input-test.txt");
static INPUT_ROCKS: &str = include_str!("input-rocks.txt");

const CHAMBER_WIDTH: usize = 7;
const MAX_ROCK_HEIGHT: usize = 4;
const ROCK_START_DIFF_Y: usize = 3;
const ROCK_START_DIFF_X: usize = 2;

#[derive(Clone, Debug)]
struct Blueprint {
    pub values: Vec<u8>,
    pub height: usize,
    pub width: usize,
}

impl FromStr for Blueprint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let mut values: Vec<u8> = vec![];

        for line in lines.iter().rev() {
            let mut value = 0;

            for (x, c) in line.chars().rev().enumerate() {
                if c == '#' {
                    value |= 1 << x;
                }
            }

            value <<= (CHAMBER_WIDTH - lines[0].len()) - ROCK_START_DIFF_X;

            values.push(value);
        }

        Ok(Blueprint {
            values,
            height: lines.len(),
            width: lines[0].len(),
        })
    }
}

#[derive(Clone, Debug)]
struct Rock {
    pub values: Vec<u8>,
    pub height: usize,
    pub width: usize,
    pub x: usize,
    pub y: usize,
}

impl Rock {
    pub fn from_blueprint(blueprint: &Blueprint, x: usize, y: usize) -> Self {
        // For some reason, cloning the whole object beforehand is faster.
        let rock = blueprint.clone();
        Rock {
            values: rock.values,
            height: rock.height,
            width: rock.width,
            x,
            y,
        }
    }

    pub fn move_left(&mut self) {
        for i in 0..self.values.len() {
            self.values[i] <<= 1;
        }
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        for i in 0..self.values.len() {
            self.values[i] >>= 1;
        }
        self.x += 1;
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Chamber<'a> {
    values: Vec<u8>,
    stream: Cycle<Iter<'a, Direction>>,
    pub max_height: usize,
}

impl<'a> Chamber<'a> {
    pub fn new(stream: &'a [Direction]) -> Self {
        // The floor takes an additional row
        let mut values = vec![0; MAX_ROCK_HEIGHT + ROCK_START_DIFF_Y + 1];
        // Set initial floor
        values[0] = 0b0111_1111;

        Chamber {
            values,
            max_height: 1,
            stream: stream.iter().cycle(),
        }
    }

    pub fn move_rock_horizontal(&mut self, rock: &mut Rock) {
        match self.stream.next().unwrap() {
            Direction::Left => {
                if rock.x == 0 {
                    return;
                }

                // As long as we are above the highest rock, no need for sophisticated collision checks.
                if rock.y - 1 > self.max_height {
                    rock.move_left();
                    return;
                }

                for i in 0..rock.values.len() {
                    if (rock.values[i] << 1 & self.values[rock.y + i]).count_ones() != 0 {
                        return;
                    }
                }

                rock.move_left();
            }
            Direction::Right => {
                if rock.x + rock.width == CHAMBER_WIDTH {
                    return;
                }

                // As long as we are above the highest rock, no need for sophisticated collision checks.
                if rock.y - 1 > self.max_height {
                    rock.move_right();

                    return;
                }

                for i in 0..rock.values.len() {
                    if (rock.values[i] >> 1 & self.values[rock.y + i]).count_ones() != 0 {
                        return;
                    }
                }

                rock.move_right();
            }
        }
    }

    pub fn rock_can_fall(&self, rock: &Rock) -> bool {
        let next_rock_y = rock.y - 1;
        for i in 0..rock.values.len() {
            if (rock.values[i] & self.values[next_rock_y + i]).count_ones() != 0 {
                return false;
            }
        }

        true
    }

    pub fn add_rock_to_structure(&mut self, rock: &Rock) {
        for i in 0..rock.values.len() {
            self.values[rock.y + i] |= rock.values[i];
        }

        // Increase max height
        let rock_diff_to_chamber_max = (rock.y + rock.height).saturating_sub(self.max_height);
        if rock_diff_to_chamber_max != 0 {
            self.max_height += rock_diff_to_chamber_max;
            self.values.extend(vec![0; rock_diff_to_chamber_max]);
        }
    }

    pub fn add_rock(&mut self, blueprint: &Blueprint) {
        // Anchor is lower left pixel
        let mut rock = Rock::from_blueprint(
            blueprint,
            ROCK_START_DIFF_X,
            self.max_height + ROCK_START_DIFF_Y,
        );

        loop {
            self.move_rock_horizontal(&mut rock);

            if !self.rock_can_fall(&rock) {
                break;
            }

            rock.y -= 1;
        }

        self.add_rock_to_structure(&rock);
    }
}

// Naive: 8ms
// Point List: 5ms
// Bitmask: 3.4ms
// Bitmask OO: 3.8ms
// TODO: Matrix? from_le_bytes
pub fn part_one() {
    let blueprints = INPUT_ROCKS
        .split("\n\n")
        .map(|block| block.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();
    let mut blueprints = blueprints.iter().cycle();

    let stream = INPUT
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            _ => Direction::Right,
        })
        .collect::<Vec<_>>();

    let mut chamber = Chamber::new(&stream);

    for _ in 0..2022 {
        chamber.add_rock(blueprints.next().unwrap());
    }

    // 3068
    println!("Height: {:?}", chamber.max_height - 1)
}
