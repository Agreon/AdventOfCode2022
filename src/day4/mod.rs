static INPUT: &'static str = include_str!("input.txt");

pub fn part_one() -> usize {
    return INPUT
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let (left_lower, left_upper) = left.split_once('-').unwrap();
            let (right_lower, right_upper) = right.split_once('-').unwrap();

            (
                left_lower.parse::<u32>().unwrap(),
                left_upper.parse::<u32>().unwrap(),
                right_lower.parse::<u32>().unwrap(),
                right_upper.parse::<u32>().unwrap(),
            )
        })
        .filter(|(left_lower, left_upper, right_lower, right_upper)| {
            (left_lower >= right_lower && left_upper <= right_upper)
                || (right_lower >= left_lower && right_upper <= left_upper)
        })
        .count();
}

pub fn part_two() -> usize {
    return INPUT
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let (left_lower, left_upper) = left.split_once('-').unwrap();
            let (right_lower, right_upper) = right.split_once('-').unwrap();

            (
                left_lower.parse::<u32>().unwrap(),
                left_upper.parse::<u32>().unwrap(),
                right_lower.parse::<u32>().unwrap(),
                right_upper.parse::<u32>().unwrap(),
            )
        })
        .filter(|(left_lower, left_upper, right_lower, right_upper)| {
            return (left_upper >= right_lower && (left_upper <= right_upper))
                || (right_upper >= left_lower && (right_upper <= left_upper));
        })
        .count();
}
