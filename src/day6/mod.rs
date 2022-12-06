static INPUT: &'static str = include_str!("input.txt");

pub fn part_one() -> usize {
    let mut distinct: Vec<char> = Vec::with_capacity(4);

    for (i, character) in INPUT.chars().enumerate() {
        if distinct.iter().all(|d| *d != character) {
            if distinct.len() == 3 {
                return i + 1;
            }
        } else {
            distinct = Vec::with_capacity(4);
        }

        distinct.push(character);
    }

    panic!("Did not find 4 distinct chars");
}

pub fn part_two() -> usize {
    let mut distinct: Vec<char> = Vec::with_capacity(14);

    for (i, character) in INPUT.chars().enumerate() {
        if distinct.iter().all(|d| *d != character) {
            if distinct.len() == 13 {
                return i + 1;
            }
        } else {
            distinct = Vec::with_capacity(14);
        }

        distinct.push(character);
    }

    panic!("Did not find 4 distinct chars");
}
