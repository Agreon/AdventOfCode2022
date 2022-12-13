use std::{collections::VecDeque, fmt::Error, num::ParseIntError, str::FromStr};

static INPUT: &'static str = include_str!("input.txt");

#[derive(Debug)]
enum Operator {
    Plus,
    Multiply,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Plus),
            "*" => Ok(Operator::Multiply),
            _ => Err(String::from("Unknown operator")),
        }
    }
}

#[derive(Debug)]
enum Operand {
    Current,
    Value(u128),
}

impl FromStr for Operand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Current),
            value => match value.parse() {
                Err(err) => Err(err),
                Ok(value) => Ok(Operand::Value(value)),
            },
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    operands: (Operand, Operand),
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let first_operand = parts.next().unwrap().parse::<Operand>().unwrap();
        let operator = parts.next().unwrap().parse::<Operator>().unwrap();
        let second_operand = parts.next().unwrap().parse::<Operand>().unwrap();

        Ok(Operation {
            operator,
            operands: (first_operand, second_operand),
        })
    }
}

impl Operation {
    pub fn execute(&self, input: u128) -> u128 {
        let first_value = match self.operands.0 {
            Operand::Current => input,
            Operand::Value(val) => val,
        };
        let second_value = match self.operands.1 {
            Operand::Current => input,
            Operand::Value(val) => val,
        };

        match self.operator {
            Operator::Plus => first_value + second_value,
            Operator::Multiply => first_value * second_value,
        }
    }
}

#[derive(Debug)]
struct Test {
    pub test: u128,
    pub monkey_on_true: usize,
    pub monkey_on_false: usize,
}

impl FromStr for Test {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        // TODO: Beautify
        let test: u128 = lines[0].split_whitespace().last().unwrap().parse().unwrap();
        let monkey_on_true: usize = lines[1].split_whitespace().last().unwrap().parse().unwrap();
        let monkey_on_false: usize = lines[2].split_whitespace().last().unwrap().parse().unwrap();

        Ok(Test {
            test,
            monkey_on_true,
            monkey_on_false,
        })
    }
}

impl Test {
    pub fn get_monkey_for_input(&self, input: u128) -> usize {
        match input % self.test {
            0 => self.monkey_on_true,
            _ => self.monkey_on_false,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    pub inspections: usize,
    pub items: VecDeque<u128>,
    pub operation: Operation,
    pub test: Test,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("Test");

        let upper_lines: Vec<_> = parts.next().unwrap().lines().collect();

        let items: VecDeque<u128> = upper_lines[1]
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|item| item.trim().parse::<u128>().unwrap())
            .collect();

        let operation_input = upper_lines[2].split_once('=').unwrap().1.trim();

        Ok(Monkey {
            inspections: 0,
            items,
            operation: operation_input.parse().unwrap(),
            test: parts.next().unwrap().parse().unwrap(),
        })
    }
}

impl Monkey {
    pub fn inspect_next_item(&mut self) -> Option<(usize, u128)> {
        let item = self.items.pop_front();
        match item {
            None => None,
            Some(item) => {
                let mut new_value = self.operation.execute(item);
                new_value /= 3;

                self.inspections += 1;

                Some((self.test.get_monkey_for_input(new_value), new_value))
            }
        }
    }

    pub fn add_item(&mut self, item: u128) {
        self.items.push_back(item);
    }
}

pub fn monkey_business(input: &str, rounds: usize) -> usize {
    let mut monkeys: Vec<_> = input
        .split("\n\n")
        .map(|part| part.parse::<Monkey>().unwrap())
        .collect();

    println!("{:?}", monkeys);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            loop {
                let result = monkeys[i].inspect_next_item();
                match result {
                    None => break,
                    Some((monkey_target, value)) => {
                        monkeys[monkey_target].add_item(value);
                    }
                }
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspections);

    return monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .product();
}

pub fn part_one() {
    let result_product = monkey_business(INPUT, 20);
    println!("{:?}", result_product);
}
