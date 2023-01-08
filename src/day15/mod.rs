use std::fmt::Debug;
use std::{num::ParseIntError, str::FromStr};

static INPUT: &'static str = include_str!("input-test.txt");

#[derive(Debug)]
struct Point<T: FromStr> {
    x: T,
    y: T,
}

impl<T: FromStr> FromStr for Point<T> {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let x = x.split_once('=').unwrap().1.parse::<T>();
        let y = y.split_once('=').unwrap().1.parse::<T>();

        match (x, y) {
            (Ok(x), Ok(y)) => Ok(Point { x, y }),
            (Err(_), Err(_)) => Err(std::fmt::Error),
            (Err(_), Ok(_)) => Err(std::fmt::Error),
            (Ok(_), Err(_)) => Err(std::fmt::Error),
        }
    }
}

#[derive(Debug)]
struct Sensor<T: FromStr> {
    position: Point<T>,
    beacon: Point<T>,
}

impl<T: FromStr> FromStr for Sensor<T> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = s.split_once(':').unwrap();
        let sensor = sensor
            .split_once("at")
            .unwrap()
            .1
            .parse::<Point<T>>()
            .unwrap();
        let beacon = beacon
            .split_once("at")
            .unwrap()
            .1
            .parse::<Point<T>>()
            .unwrap();

        Ok(Sensor {
            position: sensor,
            beacon,
        })
    }
}

fn parse_sensors(input: &str) -> Vec<Sensor<usize>> {
    let sensors: Vec<_> = input
        .lines()
        .map(|line| line.parse::<Sensor<i32>>().unwrap())
        .collect();

    let mut max_x = i32::MIN;
    let mut min_x = i32::MAX;

    let mut max_y = i32::MIN;
    let mut min_y = i32::MAX;

    for sensor in &sensors {
        println!("{:?}", sensor);
        if sensor.position.x > max_x {
            max_x = sensor.position.x;
        } else if sensor.position.x < min_x {
            min_x = sensor.position.x;
        }
        if sensor.beacon.x > max_x {
            max_x = sensor.beacon.x;
        } else if sensor.beacon.x < min_x {
            min_x = sensor.beacon.x;
        }

        if sensor.position.y > max_y {
            max_y = sensor.position.y;
        } else if sensor.position.y < min_y {
            min_y = sensor.position.y;
        }
        if sensor.beacon.y > max_y {
            max_y = sensor.beacon.y;
        } else if sensor.beacon.y < min_y {
            min_y = sensor.beacon.y;
        }
    }

    // Adjust negative position to indexable ones
    sensors
        .iter()
        .map(|sensor| Sensor::<usize> {
            position: Point::<usize> {
                x: (sensor.position.x + (min_x * -1)) as usize,
                y: (sensor.position.y + (min_y * -1)) as usize,
            },
            beacon: Point::<usize> {
                x: (sensor.beacon.x + (min_x * -1)) as usize,
                y: (sensor.beacon.y + (min_y * -1)) as usize,
            },
        })
        .collect()
}

pub fn part_one() {
    let sensors = parse_sensors(INPUT);
}
