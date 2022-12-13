use std::{cmp::Ordering, collections::BinaryHeap};

static INPUT: &'static str = include_str!("input.txt");

const MAX_VAL: usize = i32::MAX as usize;

#[derive(Debug, Clone, Eq)]
struct Point {
    pub value: u8,
    pub x: usize,
    pub y: usize,
    pub distance_from_start: usize,
    priority: usize,
    pub visited: bool,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.priority as i32 - other.priority as i32 {
            d if d > 0 => Ordering::Less,
            d if d < 0 => Ordering::Greater,
            0 => Ordering::Equal,
            _ => unreachable!(),
        }
    }
}

impl Point {
    pub fn new(x: usize, y: usize, value: u8) -> Self {
        Self {
            x,
            y,
            value,
            distance_from_start: MAX_VAL,
            priority: MAX_VAL,
            visited: false,
        }
    }

    pub fn is_valid_step_from(&self, current: &Point) -> bool {
        // Only relevant if target would actually be reached earlier than on previous way
        if self.distance_from_start < current.distance_from_start + 1 {
            return false;
        }

        // Don't step more than 1 height
        if self.value > current.value {
            return self.value - current.value <= 1;
        }

        return true;
    }

    pub fn calculate_priority(&mut self, distance_from_start: usize) {
        self.distance_from_start = distance_from_start;
        self.priority = self.distance_from_start;
    }
}

fn dijkstra(points: &mut Vec<Vec<Point>>, start: Point, target: Point) -> usize {
    let width = points[0].len();
    let height = points.len();

    // TODO: Rather reference or just coordinates?
    let mut to_check = BinaryHeap::new();
    to_check.push(start);

    let mut shortest_path = usize::MAX;

    loop {
        let current = to_check.pop();

        match current {
            None => break,
            Some(current) => {
                if points[current.y][current.x].visited {
                    continue;
                }

                if current.x == target.x && current.y == target.y {
                    shortest_path = current.distance_from_start;
                    break;
                }

                if current.x < width - 1 {
                    let right_point = &mut points[current.y][current.x + 1];
                    if right_point.is_valid_step_from(&current) {
                        right_point.calculate_priority(current.distance_from_start + 1);

                        // TODO: No clone
                        to_check.push(right_point.clone());
                    }
                }
                if current.x > 0 {
                    let left_point = &mut points[current.y][current.x - 1];
                    if left_point.is_valid_step_from(&current) {
                        left_point.calculate_priority(current.distance_from_start + 1);

                        to_check.push(left_point.clone());
                    }
                }
                if current.y < height - 1 {
                    let bottom_point = &mut points[current.y + 1][current.x];
                    if bottom_point.is_valid_step_from(&current) {
                        bottom_point.calculate_priority(current.distance_from_start + 1);

                        to_check.push(bottom_point.clone());
                    }
                }
                if current.y > 0 {
                    let top_point = &mut points[current.y - 1][current.x];
                    if top_point.is_valid_step_from(&current) {
                        top_point.calculate_priority(current.distance_from_start + 1);

                        to_check.push(top_point.clone());
                    }
                }

                points[current.y][current.x].visited = true;
            }
        }
    }

    return shortest_path;
}

pub fn part_one() {
    let lines: Vec<&[u8]> = INPUT.lines().map(|line| line.as_bytes()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut points: Vec<Vec<Point>> = vec![Vec::with_capacity(width); height];

    // TODO: Rather references
    // let mut start: Option<&Point> = None;
    // let mut target: Option<&Point> = None;
    let mut start: Option<Point> = None;
    let mut target: Option<Point> = None;

    for y in 0..height {
        for x in 0..width {
            if lines[y][x] == 83 {
                let mut point = Point::new(x, y, 'a' as u8);
                point.distance_from_start = 0;
                start = Some(point.clone());
                points[y].push(point);
            } else if lines[y][x] == 69 {
                let point = Point::new(x, y, 'z' as u8);
                target = Some(point.clone());
                points[y].push(point);
            } else {
                points[y].push(Point::new(x, y, lines[y][x]))
            }
        }
    }

    let start = start.unwrap();
    let target = target.unwrap();

    let shortest_path = dijkstra(&mut points, start, target);

    println!("Shortest {:?}", shortest_path);
}

pub fn part_two() {
    let lines: Vec<&[u8]> = INPUT.lines().map(|line| line.as_bytes()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut points: Vec<Vec<Point>> = vec![Vec::with_capacity(width); height];
    let mut starts: Vec<Point> = Vec::new();

    // TODO: Rather references
    // let mut start: Option<&Point> = None;
    // let mut target: Option<&Point> = None;
    let mut target: Option<Point> = None;

    for y in 0..height {
        for x in 0..width {
            if lines[y][x] == 'a' as u8 {
                let mut point = Point::new(x, y, 'a' as u8);
                point.distance_from_start = 0;
                starts.push(point.clone());
                points[y].push(point);
            } else if lines[y][x] == 69 {
                let point = Point::new(x, y, 'z' as u8);
                target = Some(point.clone());
                points[y].push(point);
            } else {
                points[y].push(Point::new(x, y, lines[y][x]))
            }
        }
    }

    let target = target.unwrap();

    let mut min = usize::MAX;
    for start in starts {
        let mut point_clone = points.clone();
        let new_min = dijkstra(&mut point_clone, start, target.clone());
        if new_min < min {
            min = new_min;
        }
    }

    println!("Shortest {:?}", min);
}
