static INPUT: &'static str = include_str!("input.txt");

fn count_visible_trees(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().map(|line| line.trim().as_bytes()).collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut visible: u64 = ((width + height) * 2) as u64 - 4;

    let mut left_boundary: Vec<Vec<usize>> = vec![vec![0; 64]; height];
    let mut right_boundary: Vec<Vec<usize>> = vec![vec![width - 1; 64]; height];

    let mut north_boundary = vec![0; width];

    let mut north_max = Vec::from(lines[0]);
    let mut south_max = Vec::from(lines[height - 1]);

    for y in 1..(height - 1) {
        // West
        let mut west_max = lines[y][0];
        for x in 1..(width - 1) {
            if lines[y][x] > west_max {
                visible += 1;
                west_max = lines[y][x];
                left_boundary[y][west_max as usize] = x;

                // Early abort
                if west_max == 9 {
                    break;
                }
            }
        }

        // East
        let mut east_max = lines[y][width - 1];
        for x in ((left_boundary[y][west_max as usize] + 1)..=(width - 2)).rev() {
            if lines[y][x] > east_max {
                visible += 1;
                east_max = lines[y][x];
                right_boundary[y][east_max as usize] = x;

                // No need to search further left if we are at the row maximum already.
                if east_max == west_max {
                    break;
                }
            }
        }

        // Top -> Bottom
        for x in 1..(width - 1) {
            if lines[y][x] > north_max[x] {
                north_max[x] = lines[y][x];
                // Boundary for lower
                north_boundary[x] = y;

                // To make sure we don't count trees multiple times.
                if x > left_boundary[y][north_max[x] as usize]
                    && x < right_boundary[y][north_max[x] as usize]
                {
                    visible += 1;
                }
            }
        }
    }

    // Bottom -> Top
    for y in (1..(height - 1)).rev() {
        for x in 1..(width - 1) {
            if north_boundary[x] < y && lines[y][x] > south_max[x] {
                south_max[x] = lines[y][x];

                // To make sure we don't count trees multiple times.
                if x > left_boundary[y][south_max[x] as usize]
                    && x < right_boundary[y][south_max[x] as usize]
                {
                    visible += 1;
                }
            }
        }
    }

    return visible;
}

fn count_visible_trees_simple(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut visible: u64 = ((width + height) * 2) as u64 - 4;

    let mut seen: Vec<Vec<bool>> = vec![vec![false; width]; height];

    let mut north_max = Vec::from(lines[0]);

    for y in 1..(height - 1) {
        // West
        let mut west_max = lines[y][0];
        for x in 1..(width - 1) {
            if lines[y][x] > west_max {
                visible += 1;
                west_max = lines[y][x];
                seen[y][x] = true;

                // Early abort
                if west_max == 9 {
                    break;
                }
            }
        }

        // East
        let mut east_max = lines[y][width - 1];
        for x in (1..=(width - 2)).rev() {
            if lines[y][x] > east_max {
                east_max = lines[y][x];

                if seen[y][x] == false {
                    visible += 1;
                    seen[y][x] = true;
                }

                // No need to search further left if we are at the row maximum already.
                if east_max == west_max {
                    break;
                }
            }
        }

        // Top -> Bottom
        for x in 1..(width - 1) {
            if lines[y][x] > north_max[x] {
                north_max[x] = lines[y][x];

                if seen[y][x] == false {
                    visible += 1;
                    seen[y][x] = true;
                }
            }
        }
    }

    // Bottom -> Top
    let mut south_max: Vec<u8> = Vec::from(lines[height - 1]);
    for y in (1..(height - 1)).rev() {
        for x in 1..(width - 1) {
            if lines[y][x] > south_max[x] {
                south_max[x] = lines[y][x];

                if seen[y][x] == false {
                    visible += 1;
                    seen[y][x] = true;
                }
            }
        }
    }

    return visible;
}

fn count_visible_trees_vertical_vec(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().map(|line| line.trim().as_bytes()).collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut visible: u64 = ((width + height) * 2) as u64 - 4;

    let mut seen: Vec<Vec<bool>> = vec![vec![false; width]; height];

    let mut vertical: Vec<Vec<u8>> = vec![Vec::with_capacity(height); width];

    let mut north_max = Vec::from(lines[0]);
    let mut south_max: Vec<u8> = Vec::from(lines[height - 1]);

    for y in 0..=(height - 1) {
        for x in 0..width {
            vertical[x].push(lines[y][x]);
        }

        if y == 0 || y == height - 1 {
            continue;
        }

        // West
        let mut west_max = lines[y][0];
        for x in 1..(width - 1) {
            if lines[y][x] > west_max {
                visible += 1;
                west_max = lines[y][x];
                seen[y][x] = true;

                // Early abort
                if west_max == 9 {
                    break;
                }
            }
        }

        // East
        let mut east_max = lines[y][width - 1];
        for x in (1..=(width - 2)).rev() {
            if lines[y][x] > east_max {
                // TODO: Not if already seen because we are at left max
                visible += 1;
                east_max = lines[y][x];
                seen[y][x] = true;

                // No need to search further left if we are at the row maximum already.
                if east_max == west_max {
                    break;
                }
            }
        }
    }

    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            if vertical[x][y] > north_max[x] {
                north_max[x] = vertical[x][y];

                if seen[y][x] == false {
                    visible += 1;
                    seen[y][x] = true;
                }

                // Early abort
                if north_max[x] == 9 {
                    break;
                }
            }
        }

        for y in (1..(height - 1)).rev() {
            if vertical[x][y] > south_max[x] {
                south_max[x] = vertical[x][y];

                if seen[y][x] == false {
                    visible += 1;
                    seen[y][x] = true;
                }

                // Early abort
                if south_max == north_max {
                    break;
                }
            }
        }
    }

    return visible;
}

fn visible_trees_from_inside(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().map(|line| line.trim().as_bytes()).collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut max = 0;

    for y in 0..=(height - 1) {
        for x in 0..=(width - 1) {
            let own_height = lines[y][x];

            let mut seen_west = 0;
            for w in (0..x).rev() {
                seen_west += 1;

                if lines[y][w] >= own_height {
                    break;
                }
            }

            let mut seen_east = 0;
            for w in (x + 1)..=(width - 1) {
                seen_east += 1;

                if lines[y][w] >= own_height {
                    break;
                }
            }

            let mut seen_north = 0;
            for w in (0..y).rev() {
                seen_north += 1;
                if lines[w][x] >= own_height {
                    break;
                }
            }

            let mut seen_south = 0;
            for w in (y + 1)..=(height - 1) {
                seen_south += 1;
                if lines[w][x] >= own_height {
                    break;
                }
            }

            let score = seen_west * seen_east * seen_north * seen_south;
            if score > max {
                max = score;
            }
        }
    }

    return max;
}

fn visible_trees_from_inside_one_way_caching(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map(|line| line.trim().as_bytes()).collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut west_cache: Vec<Vec<usize>> = vec![vec![1; width]; height];
    let mut north_cache: Vec<Vec<usize>> = vec![vec![1; width]; height];

    let mut max = 0;

    for y in 0..=(height - 1) {
        for x in 0..=(width - 1) {
            let own_height = lines[y][x];

            let (mut seen_west, search_from) = if x != 0 && own_height > lines[y][x - 1] {
                (west_cache[y][x - 1], x - west_cache[y][x - 1])
            } else {
                (0, x)
            };

            for w in (0..search_from).rev() {
                seen_west += 1;

                if lines[y][w] >= own_height {
                    break;
                }
            }
            west_cache[y][x] = seen_west;

            let mut seen_east = 0;
            for w in (x + 1)..=(width - 1) {
                seen_east += 1;

                if lines[y][w] >= own_height {
                    break;
                }
            }

            let (mut seen_north, search_from) = if y != 0 && own_height > lines[y - 1][x] {
                (north_cache[y - 1][x], y - north_cache[y - 1][x])
            } else {
                (0, y)
            };

            for w in (0..search_from).rev() {
                seen_north += 1;
                if lines[w][x] >= own_height {
                    break;
                }
            }
            north_cache[y][x] = seen_north;

            let mut seen_south = 0;
            for w in (y + 1)..=(height - 1) {
                seen_south += 1;
                if lines[w][x] >= own_height {
                    break;
                }
            }

            let score = seen_west * seen_east * seen_north * seen_south;
            if score > max {
                max = score;
            }
        }
    }

    return max;
}

pub fn part_one() -> u64 {
    // ~ 1.35ms Created with false premises
    // return count_visible_trees(INPUT);
    // ~ 2ms Not as efficient
    // return count_visible_trees_vertical_vec(INPUT);
    // ~ 1.3ms
    return count_visible_trees_simple(INPUT);
}

pub fn part_two() {
    // ~ 3.3ms
    visible_trees_from_inside(INPUT);
    // ~4.6ms Is not worth the optimization
    // visible_trees_from_inside_one_way_caching(INPUT);
}

#[cfg(test)]
mod tests {
    use crate::day8::count_visible_trees;
    use crate::day8::count_visible_trees_simple;

    use super::visible_trees_from_inside;

    static TEST_INPUT: &str = r"302373
    253512
    252512";

    #[test]
    fn test1() {
        assert_eq!(count_visible_trees(TEST_INPUT), 17);
        assert_eq!(count_visible_trees_simple(TEST_INPUT), 17);
    }

    static TEST_INPUT_2: &str = r"12345
    54512
    12345";

    #[test]
    fn test2() {
        assert_eq!(count_visible_trees(TEST_INPUT_2), 14);
        assert_eq!(count_visible_trees_simple(TEST_INPUT_2), 14);
    }

    static TEST_INPUT_3: &str = r"12345
    54545
    12535
    54321";

    #[test]
    fn test3() {
        assert_eq!(count_visible_trees(TEST_INPUT_3), 20);
        assert_eq!(count_visible_trees_simple(TEST_INPUT_3), 20);
    }

    static TEST_INPUT_4: &str = r"30373
    25512
    65332
    33549
    35390";

    #[test]
    fn test4() {
        assert_eq!(count_visible_trees(TEST_INPUT_4), 21);
        assert_eq!(count_visible_trees_simple(TEST_INPUT_4), 21);
    }

    static TEST_INPUT_5: &str = r"30373
    25502
    65352
    35249
    35190";

    #[test]
    fn test5() {
        assert_eq!(count_visible_trees(TEST_INPUT_5), 22);
        assert_eq!(count_visible_trees_simple(TEST_INPUT_5), 22);
    }

    static TEST_INPUT_6: &str = r"20102
    22112
    10211";

    #[test]
    fn test6() {
        assert_eq!(count_visible_trees(TEST_INPUT_6), 14);
        assert_eq!(count_visible_trees_simple(TEST_INPUT_6), 14);
    }

    static TEST_INPUT_7: &str = r"20112
    22113
    10201";

    #[test]
    fn test7() {
        assert_eq!(count_visible_trees(TEST_INPUT_7), 14);
        assert_eq!(count_visible_trees_simple(TEST_INPUT_7), 14);
    }

    static TEST_INPUT_8: &str = r"30373
    25512
    65332
    33549
    35390";

    #[test]
    fn test8() {
        assert_eq!(visible_trees_from_inside(TEST_INPUT_8), 8);
    }
}
