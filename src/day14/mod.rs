use std::{num::ParseIntError, str::FromStr};

static INPUT: &'static str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
    Start,
}

impl Tile {
    pub fn draw(&self) {
        match self {
            Tile::Air => print!("."),
            Tile::Rock => print!("#"),
            Tile::Sand => print!("o"),
            Tile::Start => print!("+"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(',').unwrap();

        Ok(Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

struct Grid {
    pub grid: Vec<Vec<Tile>>,
    pub sand_start: Point,
    pub lowest_rock_y: usize,
    min_x: usize,
    max_x: usize,
}

impl FromStr for Grid {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rock_points = s
            .split('\n')
            .map(|line| {
                line.split("->")
                    .map(|step| step.parse::<Point>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let max_x = rock_points
            .iter()
            .flat_map(|f| f)
            .map(|point| point.x)
            .max()
            .unwrap();
        let min_x = rock_points
            .iter()
            .flat_map(|f| f)
            .map(|point| point.x)
            .min()
            .unwrap();
        let max_y = rock_points
            .iter()
            .flat_map(|f| f)
            .map(|point| point.y)
            .max()
            .unwrap();

        let max_x = max_x + 200;

        // TODO: Try optimization to not create 700 empty spaces
        let mut grid: Vec<Vec<Tile>> = vec![vec![Tile::Air; max_x]; max_y + 4];

        for line in rock_points {
            for pair in line.windows(2) {
                let left = &pair[0];
                let right = &pair[1];

                if right.x > left.x {
                    for x in left.x..=right.x {
                        grid[left.y][x] = Tile::Rock;
                    }
                    continue;
                } else if left.x > right.x {
                    for x in right.x..=left.x {
                        grid[left.y][x] = Tile::Rock;
                    }
                    continue;
                }

                if right.y > left.y {
                    for y in left.y..=right.y {
                        grid[y][left.x] = Tile::Rock;
                    }
                } else if left.y > right.y {
                    for y in right.y..=left.y {
                        grid[y][left.x] = Tile::Rock;
                    }
                }
            }
        }

        grid[0][500] = Tile::Start;

        Ok(Grid {
            grid,
            sand_start: Point { x: 500, y: 0 },
            lowest_rock_y: max_y,
            min_x,
            max_x,
        })
    }
}

impl Grid {
    pub fn draw(&self) {
        for y in 0..self.grid.len() {
            print!("{:?}", y);
            for i in (self.min_x)..(self.max_x) {
                self.grid[y][i].draw()
            }
            println!();
        }
    }

    pub fn add_sand(&mut self) -> Option<Point> {
        let mut current_position = self.sand_start;

        loop {
            if self.grid[current_position.y + 1][current_position.x] == Tile::Air {
                current_position = Point {
                    x: current_position.x,
                    y: current_position.y + 1,
                };
            } else if self.grid[current_position.y + 1][current_position.x - 1] == Tile::Air {
                current_position = Point {
                    x: current_position.x - 1,
                    y: current_position.y + 1,
                };
            } else if self.grid[current_position.y + 1][current_position.x + 1] == Tile::Air {
                current_position = Point {
                    x: current_position.x + 1,
                    y: current_position.y + 1,
                };
            } else {
                break;
            }

            if current_position.y >= self.lowest_rock_y {
                return None;
            }
        }

        self.grid[current_position.y][current_position.x] = Tile::Sand;

        return Some(current_position);
    }

    pub fn add_floor(&mut self) {
        let place_floor_at = self.lowest_rock_y + 2;
        for x in 0..self.grid[0].len() {
            self.grid[place_floor_at][x] = Tile::Rock;
        }
    }

    pub fn add_sand_until_blocked(&mut self) -> Option<Point> {
        let mut current_position = self.sand_start;

        loop {
            if self.grid[current_position.y + 1][current_position.x] == Tile::Air {
                current_position = Point {
                    x: current_position.x,
                    y: current_position.y + 1,
                };
            } else if self.grid[current_position.y + 1][current_position.x - 1] == Tile::Air {
                current_position = Point {
                    x: current_position.x - 1,
                    y: current_position.y + 1,
                };
            } else if self.grid[current_position.y + 1][current_position.x + 1] == Tile::Air {
                current_position = Point {
                    x: current_position.x + 1,
                    y: current_position.y + 1,
                };
            } else {
                if current_position.y == self.sand_start.y
                    && current_position.x == self.sand_start.x
                {
                    return None;
                }

                break;
            }
        }

        self.grid[current_position.y][current_position.x] = Tile::Sand;

        return Some(current_position);
    }
}

pub fn part_one() {
    let mut grid = INPUT.parse::<Grid>().unwrap();
    grid.draw();

    let mut sand_count = 0;

    loop {
        match grid.add_sand() {
            None => break,
            Some(_) => sand_count += 1,
        }
    }
    grid.draw();

    println!("Count {:?}", sand_count);
}

pub fn part_two() {
    let mut grid = INPUT.parse::<Grid>().unwrap();
    grid.add_floor();

    let mut sand_count = 0;

    loop {
        sand_count += 1;
        match grid.add_sand_until_blocked() {
            None => break,
            Some(_) => {}
        }
    }

    println!("Count {:?}", sand_count);
}
