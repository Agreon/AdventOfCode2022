use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");

pub fn part_one() -> usize {
    let point_map: HashMap<char, usize> =
        HashMap::from_iter([('A', 1), ('B', 2), ('C', 3), ('X', 1), ('Y', 2), ('Z', 3)]);

    // Rock, Paper, Scissors
    let win_map = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];

    let mut all_points = 0;

    for line in INPUT.lines() {
        let enemy_point = point_map.get(&line.chars().nth(0).unwrap()).unwrap();
        let own_point = point_map.get(&line.chars().nth(2).unwrap()).unwrap();
        let win_points = win_map[enemy_point.to_owned() - 1][own_point.to_owned() - 1];
        all_points += win_points + own_point;
    }

    all_points
}

pub fn part_two() -> usize {
    let point_map: HashMap<char, usize> =
        HashMap::from_iter([('A', 1), ('B', 2), ('C', 3), ('X', 1), ('Y', 2), ('Z', 3)]);

    // Rock, Paper, Scissors => Loose, Draw, Win
    let win_map = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];

    let mut all_points = 0;

    for line in INPUT.lines() {
        let enemy_point = point_map.get(&line.chars().nth(0).unwrap()).unwrap();
        let own_point = point_map.get(&line.chars().nth(2).unwrap()).unwrap();
        let win_points = win_map[enemy_point.to_owned() - 1][own_point.to_owned() - 1];
        all_points += win_points;
    }

    all_points
}

#[cfg(test)]
mod tests {
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 14163)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 12091)
    }
}
