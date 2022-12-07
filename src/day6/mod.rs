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

fn find_distinct_sequence_binary(length: usize) -> usize {
    let mut dist: u32;
    let converted_length: u32 = length.try_into().unwrap();

    for (i, window) in INPUT.as_bytes().windows(length).enumerate() {
        dist = 0;

        for character in window {
            dist |= 1 << (*character - b'a');
        }

        if dist.count_ones() == converted_length {
            return i + length;
        }
    }

    panic!("Did not find 4 distinct chars");
}

// is faster
pub fn part_one() -> usize {
    let test = find_distinct_sequence_binary(4);

    println!("{:?}", test);
    test
}

// TODO: Is slower
pub fn part_two() -> usize {
    let test = find_distinct_sequence(14);

    println!("{:?}", test);
    test
}
