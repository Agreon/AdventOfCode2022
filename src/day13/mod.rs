use std::vec;

static INPUT: &'static str = include_str!("input-test.txt");

#[derive(Debug)]
enum Element {
    Value(u8),
    Array(Vec<Element>),
}

fn parse_array(arr: &str) -> Vec<Element> {
    let mut vector_stack: Vec<Vec<Element>> = vec![];
    let mut current_vec: Vec<Element> = vec![];
    let mut current_value: Option<u8> = None;

    // Skip first '['
    let chars = arr.chars().skip(1);
    for character in chars {
        match character {
            '[' => {
                vector_stack.push(current_vec);
                current_vec = vec![];
            }
            ']' => {
                match current_value {
                    // previous value was an array
                    None => {}
                    Some(value) => {
                        current_vec.push(Element::Value(value));
                        current_value = None;
                    }
                };

                match vector_stack.pop() {
                    // We reached the end of the line
                    None => {}
                    Some(mut vector) => {
                        vector.push(Element::Array(current_vec));
                        current_vec = vector;
                    }
                };
            }
            ',' => match current_value {
                // previous value was an array
                None => {}
                Some(value) => {
                    current_vec.push(Element::Value(value));
                    current_value = None;
                }
            },
            value => {
                let digit = value.to_digit(10).unwrap() as u8;
                current_value = Some(current_value.unwrap_or(0) * 10 + digit);
            }
        }
    }

    return current_vec;
}

fn parse(pair: &str) -> (Vec<Element>, Vec<Element>) {
    let (left, right) = pair.split_once('\n').unwrap();
    let left = parse_array(left);
    let right = parse_array(right);

    (left, right)
}

fn pair_in_order(pair: &(Vec<Element>, Vec<Element>)) -> bool {
    return true;
}

pub fn part_one() {
    let pairs = INPUT.split("\n\n").map(|block| parse(block));

    for (i, pair) in pairs.enumerate() {
        println!("{:?}:{:?}", i, pair_in_order(&pair));
    }
}
