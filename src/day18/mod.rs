use std::{cell::RefCell, collections::HashMap, str::FromStr, string::ParseError, time::Instant};

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq)]
struct Coordinate {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub value: usize,
}

impl FromStr for Coordinate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(|value| value.parse::<usize>().unwrap());

        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let z = parts.next().unwrap();

        let value = x * 100 + y * 10 + z;

        Ok(Coordinate { y, x, z, value })
    }
}

impl Coordinate {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Coordinate {
            x,
            y,
            z,
            value: x * 100 + y * 10 + z,
        }
    }
}

#[derive(Debug)]
struct Face {
    pub coordinates: [Coordinate; 4],
    // Used to categorize the face for indexing.
    pub value: usize,
}

impl Face {
    pub fn new(coordinates: [Coordinate; 4]) -> Self {
        Face {
            value: coordinates.iter().map(|c| c.value).sum(),
            coordinates,
        }
    }
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.coordinates.len() {
            if self.coordinates[i] != other.coordinates[i] {
                return false;
            }
        }
        true
    }
}

struct Cube {
    pub faces: [Face; 6],
}

impl FromStr for Cube {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let position = s.parse::<Coordinate>().unwrap();

        let faces = [
            Face::new([
                Coordinate::new(position.x, position.y, position.z),
                Coordinate::new(position.x, position.y, position.z + 1),
                Coordinate::new(position.x, position.y + 1, position.z),
                Coordinate::new(position.x, position.y + 1, position.z + 1),
            ]),
            Face::new([
                Coordinate::new(position.x, position.y, position.z),
                Coordinate::new(position.x, position.y, position.z + 1),
                Coordinate::new(position.x + 1, position.y, position.z),
                Coordinate::new(position.x + 1, position.y, position.z + 1),
            ]),
            Face::new([
                Coordinate::new(position.x, position.y, position.z),
                Coordinate::new(position.x, position.y + 1, position.z),
                Coordinate::new(position.x + 1, position.y, position.z),
                Coordinate::new(position.x + 1, position.y + 1, position.z),
            ]),
            Face::new([
                Coordinate::new(position.x, position.y + 1, position.z),
                Coordinate::new(position.x, position.y + 1, position.z + 1),
                Coordinate::new(position.x + 1, position.y + 1, position.z),
                Coordinate::new(position.x + 1, position.y + 1, position.z + 1),
            ]),
            Face::new([
                Coordinate::new(position.x + 1, position.y, position.z),
                Coordinate::new(position.x + 1, position.y, position.z + 1),
                Coordinate::new(position.x + 1, position.y + 1, position.z),
                Coordinate::new(position.x + 1, position.y + 1, position.z + 1),
            ]),
            Face::new([
                Coordinate::new(position.x, position.y, position.z + 1),
                Coordinate::new(position.x, position.y + 1, position.z + 1),
                Coordinate::new(position.x + 1, position.y, position.z + 1),
                Coordinate::new(position.x + 1, position.y + 1, position.z + 1),
            ]),
        ];

        Ok(Cube { faces })
    }
}

// 11ms/3.7ms
pub fn part_one() {
    let now = Instant::now();

    let cubes: Vec<_> = INPUT
        .lines()
        .map(|line| line.parse::<Cube>().unwrap())
        .collect();

    let max_faces = cubes.len() * 6;

    let mut existing_faces: HashMap<usize, RefCell<Vec<&Face>>> = HashMap::with_capacity(max_faces);

    println!("setup: {:.2?}", now.elapsed());
    println!("Faces: {max_faces}");

    let mut free_faces = max_faces;

    for face in cubes.iter().flat_map(|c| &c.faces) {
        match existing_faces.get(&face.value) {
            None => {
                existing_faces.insert(face.value, RefCell::new(vec![face]));
            }
            Some(face_collection) => {
                let mut face_collection = face_collection.borrow_mut();

                match face_collection.iter().find(|other| ***other == *face) {
                    Some(_) => free_faces -= 2,
                    None => face_collection.push(face),
                }
            }
        }
    }

    println!("Free {free_faces}");
}
