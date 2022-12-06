static INPUT: &'static str = include_str!("input.txt");

fn find_distinct_sequence(length: usize) -> usize {
    let mut distinct: Vec<char> = Vec::with_capacity(length);

    for (i, character) in INPUT.chars().enumerate() {
        if distinct.iter().all(|d| *d != character) {
            if distinct.len() == length - 1 {
                return i + 1;
            }
        } else {
            distinct = Vec::with_capacity(length);
        }

        distinct.push(character);
    }

    panic!("Did not find 4 distinct chars");
}

pub fn part_one() -> usize {
    find_distinct_sequence(4)
}

pub fn part_two() -> usize {
    find_distinct_sequence(14)
}
