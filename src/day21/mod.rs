use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
    string::ParseError,
};

static INPUT: &str = include_str!("input.txt");

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Sub),
            "*" => Ok(Operation::Mul),
            "/" => Ok(Operation::Div),
            _ => unimplemented!(),
        }
    }
}

struct Calculation {
    pub name: String,
    pub lhs: String,
    pub rhs: String,
    pub operation: Operation,
}

impl Calculation {
    pub fn solve(&self, lhs: usize, rhs: usize) -> usize {
        match self.operation {
            Operation::Add => lhs + rhs,
            Operation::Sub => lhs - rhs,
            Operation::Mul => lhs * rhs,
            Operation::Div => lhs / rhs,
        }
    }

    pub fn solve_lhs(&self, result: usize, rhs: usize) -> usize {
        match self.operation {
            Operation::Add => result - rhs,
            Operation::Sub => result + rhs,
            Operation::Mul => result / rhs,
            Operation::Div => result * rhs,
        }
    }

    pub fn solve_rhs(&self, result: usize, lhs: usize) -> usize {
        match self.operation {
            Operation::Add => result - lhs,
            Operation::Sub => lhs - result,
            Operation::Mul => result / lhs,
            Operation::Div => lhs / result,
        }
    }
}

pub fn part_one() {
    find_root_value(INPUT);
}

/**
 * Try:
 * - HashMap::from() may be faster?
 * - Some way of not copying all the strings?
 */
pub fn find_root_value(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let mut constants = HashMap::with_capacity(lines.len());
    let mut calculations = HashMap::with_capacity(lines.len() / 2);

    for line in lines {
        let (name, value) = line.split_once(':').unwrap();
        let value = value.trim();

        match value.parse::<usize>() {
            Ok(value) => {
                constants.insert(name, value);
            }
            Err(_) => {
                let parts: Vec<_> = value.split_whitespace().collect();

                let calculation = Calculation {
                    name: name.to_string(),
                    lhs: parts[0].to_string(),
                    operation: parts[1].parse().unwrap(),
                    rhs: parts[2].to_string(),
                };

                calculations.insert(name, calculation);
            }
        }
    }

    let mut to_calculate = Vec::with_capacity(calculations.len());

    let root = calculations.get("root").unwrap();
    let mut to_visit = VecDeque::from([root]);

    while let Some(visit) = to_visit.pop_front() {
        to_calculate.push(visit);

        if let Some(lhs) = calculations.get(visit.lhs.as_str()) {
            to_visit.push_back(lhs);
        }

        if let Some(rhs) = calculations.get(visit.rhs.as_str()) {
            to_visit.push_back(rhs);
        }
    }

    while let Some(calculation) = to_calculate.pop() {
        let lhs = constants.get(calculation.lhs.as_str()).unwrap();
        let rhs = constants.get(calculation.rhs.as_str()).unwrap();

        let value = match calculation.operation {
            Operation::Add => lhs + rhs,
            Operation::Sub => lhs - rhs,
            Operation::Mul => lhs * rhs,
            Operation::Div => lhs / rhs,
        };

        constants.insert(&calculation.name, value);
    }

    *constants.get("root").unwrap()
}

pub fn part_two() -> usize {
    find_human_input(INPUT)
}

pub fn find_human_input(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    let mut constants = HashMap::with_capacity(lines.len());
    let mut calculations = HashMap::with_capacity(lines.len() / 2);

    for line in lines {
        let (name, value) = line.split_once(':').unwrap();
        if name == "humn" {
            continue;
        }

        match value.trim().parse::<usize>() {
            Ok(value) => {
                constants.insert(name, value);
            }
            Err(_) => {
                let parts: Vec<_> = value.split_whitespace().collect();

                calculations.insert(
                    name,
                    Calculation {
                        name: name.to_string(),
                        lhs: parts[0].to_string(),
                        operation: parts[1].parse().unwrap(),
                        rhs: parts[2].to_string(),
                    },
                );
            }
        }
    }

    let mut to_calculate = Vec::with_capacity(calculations.len());

    let root = calculations.get("root").unwrap();
    let mut to_visit = VecDeque::from([(root, vec![])]);

    let mut human_path = vec![];

    while let Some((visit, path)) = to_visit.pop_front() {
        to_calculate.push(visit);
        let mut new_path = path;
        new_path.push(visit.name.clone());

        if visit.lhs == "humn" {
            human_path = new_path.clone();
        } else if let Some(lhs) = calculations.get(visit.lhs.as_str()) {
            to_visit.push_back((lhs, new_path.clone()));
        }

        if visit.rhs == "humn" {
            human_path = new_path.clone();
        } else if let Some(rhs) = calculations.get(visit.rhs.as_str()) {
            to_visit.push_back((rhs, new_path.clone()));
        }
    }
    let mut path_stack = human_path.clone();
    while let Some(calculation) = to_calculate.pop() {
        // Skip calculations in human path
        if !path_stack.is_empty() && calculation.name == path_stack[path_stack.len() - 1] {
            path_stack.pop();
            continue;
        }

        let lhs = constants.get(calculation.lhs.as_str()).unwrap();
        let rhs = constants.get(calculation.rhs.as_str()).unwrap();
        let value = calculation.solve(*lhs, *rhs);

        constants.insert(&calculation.name, value);
    }

    for step in &human_path {
        let calculation = calculations.get(step.as_str()).unwrap();
        let lhs = constants.get(calculation.lhs.as_str());
        let rhs = constants.get(calculation.rhs.as_str());

        match (lhs, rhs) {
            (None, Some(rhs)) => {
                if calculation.name == "root" {
                    constants.insert(&calculation.lhs, *rhs);
                } else {
                    let result = constants.get(calculation.name.as_str()).unwrap();
                    let lhs = calculation.solve_lhs(*result, *rhs);
                    constants.insert(&calculation.lhs, lhs);
                }
            }
            (Some(lhs), None) => {
                if calculation.name == "root" {
                    constants.insert(&calculation.rhs, *lhs);
                } else {
                    let result = constants.get(calculation.name.as_str()).unwrap();
                    let rhs = calculation.solve_rhs(*result, *lhs);
                    constants.insert(&calculation.rhs, rhs);
                }
            }
            _ => unreachable!(),
        }
    }

    *constants.get("humn").unwrap()
}

#[cfg(test)]
mod tests {
    use super::{find_human_input, find_root_value};
    static INPUT_TEST: &str = include_str!("input-test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one_test_data() {
        assert_eq!(find_root_value(INPUT_TEST), 152)
    }

    #[test]
    fn test_part_one() {
        assert_eq!(find_root_value(INPUT), 63119856257960)
    }

    #[test]
    fn test_part_two_test_data() {
        assert_eq!(find_human_input(INPUT_TEST), 301)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(find_human_input(INPUT), 3006709232464)
    }
}
