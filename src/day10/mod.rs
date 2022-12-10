static INPUT: &'static str = include_str!("input.txt");

enum Cmd {
    Noop,
    Add(i32),
}

pub fn part_one() -> i32 {
    let mut total = 0;

    let mut commands = INPUT.lines().map(|line| {
        let mut parts = line.split_whitespace();

        match parts.next().unwrap() {
            "addx" => Cmd::Add(parts.next().unwrap().parse::<i32>().unwrap()),
            "noop" => Cmd::Noop,
            _ => panic!("Unknown part"),
        }
    });

    let mut x = 1;
    let mut cycles = 1;
    let mut add_next_cycle = 0;

    while true {
        if (cycles % 40) - 20 == 0 {
            total += x * cycles;
        }

        if add_next_cycle != 0 {
            x += add_next_cycle;
            add_next_cycle = 0;
            cycles += 1;
            continue;
        }

        match commands.next() {
            None => break,
            Some(command) => match command {
                Cmd::Noop => {}
                Cmd::Add(value) => add_next_cycle = value,
            },
        }

        cycles += 1;
    }

    println!("TOTAL {:?}", total);

    return total;
}
