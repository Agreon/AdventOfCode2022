static INPUT: &'static str = include_str!("input.txt");

fn find_distinct_sequence(length: usize) -> usize {
    let mut distinct: Vec<u8> = Vec::with_capacity(length);

    for (i, character) in INPUT.bytes().enumerate() {
        if distinct.iter().all(|d| *d != character) {
            if distinct.len() == length - 1 {
                return i + 1;
            }
        } else {
            distinct.clear();
        }

        distinct.push(character);
    }

    panic!("Did not find 4 distinct chars");
}

fn find_distinct_sequence_binary(length: usize) -> usize {
    let converted_length: u32 = length.try_into().unwrap();

    let characters = INPUT
        .bytes()
        .map(|character| character - b'a')
        .collect::<Vec<u8>>();

    let mut dist: u32;
    for i in 0..characters.len() {
        dist = 0;

        for j in i..(i + length) {
            let before = dist;
            dist |= 1 << characters[j];

            if before == dist {
                break;
            }
        }

        if dist.count_ones() == converted_length {
            return i + length;
        }
    }

    panic!("Did not find {:?} distinct chars", length);
}

// is faster
pub fn part_one() -> usize {
    let test = find_distinct_sequence(4);

    println!("{:?}", test);
    test
}

// TODO: Is slower
pub fn part_two() -> usize {
    let test = find_distinct_sequence(14);

    println!("{:?}", test);
    test
}
