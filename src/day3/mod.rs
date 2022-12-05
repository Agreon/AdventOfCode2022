static INPUT: &'static str = include_str!("input.txt");

fn find_common_char(a: &str, b: &str) -> char {
    for start in a.chars() {
        for end in b.chars() {
            if start == end {
                return start;
            }
        }
    }

    panic!()
}

fn char_to_priority(character: char) -> u32 {
    match character {
        'A'..='Z' => character as u32 - 38,
        _ => character as u32 - 96,
    }
}

pub fn part_one() -> u32 {
    return INPUT
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(start, end)| find_common_char(start, end))
        .map(|common_char| char_to_priority(common_char))
        .sum();
}

fn find_common(group: Vec<&str>) -> char {
    for character in group[0].chars() {
        if group[1].contains(character) && group[2].contains(character) {
            return character;
        }
    }
    panic!("{:?}", group)
}

fn rucksack_beauty() -> u32 {
    let mut current_group_lines = 0;
    let mut groups: Vec<Vec<&str>> = Vec::with_capacity(128);
    let mut current_group: Vec<&str> = Vec::with_capacity(3);

    for line in INPUT.lines() {
        current_group.push(line);

        current_group_lines += 1;

        if current_group_lines == 3 {
            groups.push(current_group.clone());
            current_group = Vec::with_capacity(3);
            current_group_lines = 0;
        }
    }

    let mut priority_sum = 0;

    for group in groups {
        let common_char = find_common(group);
        priority_sum += char_to_priority(common_char);
    }

    return priority_sum;
}

// TODO: Not faster
fn rucksack_fast() -> u32 {
    let mut priority_sum = 0;

    let mut current_group_lines = 0;
    let mut possible_tokens: Vec<char> = Vec::new();

    for current_line in INPUT.lines() {
        if current_group_lines == 0 {
            possible_tokens = current_line.chars().collect();
            current_group_lines += 1;
            continue;
        }

        // let remaining_possible: Vec<char> = possible_tokens
        //     .iter() // TODO: Uncool
        //     .cloned()
        //     .filter(|t| current_line.contains(*t))
        //     .collect();

        let mut remaining_possible: Vec<char> = Vec::with_capacity(64);
        for possible in possible_tokens {
            if current_line.contains(possible) {
                remaining_possible.push(possible);
            }
        }

        possible_tokens = remaining_possible;

        current_group_lines += 1;

        if current_group_lines == 3 {
            priority_sum += char_to_priority(possible_tokens[0]);

            current_group_lines = 0;
        }
    }

    return priority_sum;
}

pub fn part_two() -> u32 {
    return rucksack_beauty();
}
