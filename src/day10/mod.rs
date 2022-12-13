static INPUT: &'static str = include_str!("input.txt");

enum Cmd {
    Noop,
    Add(i32),
}

fn parse_commands<'a>(input: &'a str) -> impl Iterator<Item = Cmd> + 'a {
    return input.lines().map(|line| {
        let mut parts = line.split_whitespace();

        match (parts.next().unwrap(), parts.next()) {
            ("noop", None) => Cmd::Noop,
            ("addx", Some(amount)) => Cmd::Add(amount.parse::<i32>().unwrap()),
            _ => unreachable!(),
        }
    });
}

pub fn part_one() -> i32 {
    let mut total = 0;

    let mut commands = parse_commands(INPUT);

    let mut x = 1;
    let mut cycles = 1;
    let mut add_next_cycle = 0;

    loop {
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

pub fn part_two() {
    let mut commands = parse_commands(INPUT);

    let mut screen = vec![vec![false; 40]; 6];

    let mut x: i32 = 1;
    let mut cycle: i32 = 1;
    let mut add_next_cycle: i32 = 0;

    loop {
        for i in (x - 1)..=(x + 1) {
            if i == (cycle - 1) % 40 {
                let y = (cycle - 1) / 40;
                let x = (cycle - 1) % 40;
                screen[y as usize][x as usize] = true;
            }
        }

        if add_next_cycle != 0 {
            x += add_next_cycle;
            add_next_cycle = 0;
            cycle += 1;
            continue;
        }

        match commands.next() {
            None => break,
            Some(command) => match command {
                Cmd::Noop => {}
                Cmd::Add(value) => add_next_cycle = value,
            },
        }

        cycle += 1;
    }

    for y in 0..screen.len() {
        for x in 0..screen[y].len() {
            if screen[y][x] {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("")
    }
}
