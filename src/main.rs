pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod util;

fn execute_days() {
    println!("## Day 1");
    util::with_timing(day1::part_one);
    util::with_timing(day1::part_two);

    println!("## Day 2");
    util::with_timing(day2::part_one);
    util::with_timing(day2::part_two);

    println!("## Day 3");
    util::with_timing(day3::part_one);
    util::with_timing(day3::part_two);

    println!("## Day 4");
    util::with_timing(day4::part_one);
    util::with_timing(day4::part_two);

    println!("## Day 5");
    util::with_timing(day5::part_one);
    util::with_timing(day5::part_two);

    println!("## Day 6");
    util::with_timing(day6::part_one);
    util::with_timing(day6::part_two);

    println!("## Day 7");
    util::with_timing(day7::part_one);
    util::with_timing(day7::part_two);

    println!("## Day 8");
    util::with_timing(day8::part_one);
    util::with_timing(day8::part_two);
}

fn main() {
    util::with_timing(execute_days);
}
