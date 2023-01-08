use std::{str::FromStr, string::ParseError};

static INPUT: &str = include_str!("input.txt");
static INPUT_ROCKS: &str = include_str!("input-rocks.txt");

#[derive(Clone, Debug)]
struct Rock {
    // TODO: Instead of matrix, maybe save single Vec with coordinates
    // TODO: Other op=full-length bit-lines for rocks, to make & easy for collisions
    pub points: Vec<Vec<bool>>,
    pub height: usize,
    pub width: usize,
}

impl FromStr for Rock {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        let mut points = Vec::with_capacity(lines.len());

        for line in lines.iter().rev() {
            points.push(Vec::with_capacity(line.len()));
            let point_idx = points.len() - 1;

            for c in line.chars() {
                points[point_idx].push(c == '#');
            }
        }

        Ok(Rock {
            points,
            height: lines.len(),
            width: lines[0].len(),
        })
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Chamber {
    pub points: Vec<Vec<bool>>,
    pub max_height: usize,
}

const CHAMBER_WIDTH: usize = 7;
const MAX_ROCK_HEIGHT: usize = 4;
const ROCK_START_DIFF: usize = 3;

impl Chamber {
    pub fn new() -> Self {
        // The floor takes an additional row
        let mut points = vec![vec![false; CHAMBER_WIDTH]; MAX_ROCK_HEIGHT + ROCK_START_DIFF + 1];
        // Set initial floor
        points[0] = vec![true; CHAMBER_WIDTH];

        Chamber {
            points,
            max_height: 1,
        }
    }
}

pub fn part_one() {
    let mut chamber = Chamber::new();

    let rocks = INPUT_ROCKS
        .split("\n\n")
        .map(|block| block.parse::<Rock>().unwrap())
        .collect::<Vec<_>>();
    let mut rocks = rocks.iter().cycle();

    let stream = INPUT
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            _ => Direction::Right,
        })
        .collect::<Vec<_>>();
    let mut stream = stream.iter().cycle();

    let mut rock_count = 0;

    loop {
        rock_count += 1;

        // No need to clone if the rock-structure is separated from current position.
        let rock = rocks.next().unwrap();
        // Anchor is lower left pixel
        let mut rock_y = chamber.max_height + ROCK_START_DIFF;
        let mut rock_x = 2;

        loop {
            // As long as we are above the highest rock, no need for sophisticated collision checks.
            if rock_y - 1 > chamber.max_height {
                match stream.next().unwrap() {
                    Direction::Left => {
                        if rock_x > 0 {
                            rock_x -= 1
                        }
                    }
                    Direction::Right => {
                        if rock_x + rock.width < CHAMBER_WIDTH {
                            rock_x += 1
                        }
                    }
                }

                rock_y -= 1;

                continue;
            }

            let mut next_rock_x = rock_x;
            match stream.next().unwrap() {
                Direction::Left => {
                    if rock_x > 0 {
                        // println!("LEFT");
                        next_rock_x = rock_x - 1;
                    }
                }
                Direction::Right => {
                    if rock_x + rock.width < CHAMBER_WIDTH {
                        // println!("RIGHT");

                        next_rock_x = rock_x + 1;
                    }
                }
            };

            if next_rock_x != rock_x {
                let mut would_hit_rock = false;
                'outer: for y in 0..rock.points.len() {
                    for x in 0..rock.width {
                        if rock.points[y][x] && chamber.points[y + rock_y][x + next_rock_x] {
                            would_hit_rock = true;
                            break 'outer;
                        }
                    }
                }
                if !would_hit_rock {
                    rock_x = next_rock_x;
                }
            }

            // TODO: Do more sophisticated check
            let next_rock_y = rock_y - 1;

            let mut would_intersect = false;
            'outer: for y in 0..rock.points.len() {
                for x in 0..rock.width {
                    if rock.points[y][x] && chamber.points[y + next_rock_y][x + rock_x] {
                        would_intersect = true;
                        break 'outer;
                    }
                }
            }

            if would_intersect {
                for y in 0..rock.points.len() {
                    for x in 0..rock.width {
                        if rock.points[y][x] {
                            chamber.points[y + rock_y][x + rock_x] = true;
                        }
                    }
                }

                // Increase max height
                let rock_diff_to_chamber_max =
                    (rock_y + rock.height).saturating_sub(chamber.max_height);
                if rock_diff_to_chamber_max != 0 {
                    chamber.max_height += rock_diff_to_chamber_max;

                    chamber
                        .points
                        .extend(vec![vec![false; CHAMBER_WIDTH]; rock_diff_to_chamber_max]);
                }

                break;
            } else {
                rock_y = next_rock_y;
            }
        }

        if rock_count == 2022 {
            break;
        }
    }

    println!("Height: {:?}", chamber.max_height - 1)
}
