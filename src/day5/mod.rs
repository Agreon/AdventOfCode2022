static INPUT: &'static str = include_str!("input.txt");

#[derive(Debug)]
struct Harbour {
    pub stacks: Vec<Vec<char>>,
    pub multi_pick: bool,
}

impl Harbour {
    pub fn new(stack_capacity: usize, stack_amount: usize, multi_pick: bool) -> Self {
        let mut stacks = Vec::with_capacity(stack_amount);

        for _ in 0..stack_amount {
            stacks.push(Vec::with_capacity(stack_capacity));
        }

        Harbour { stacks, multi_pick }
    }

    pub fn add_crate(&mut self, stack: usize, crte: char) {
        self.stacks[stack].push(crte);
    }

    pub fn apply_instruction(&mut self, instruction: &Instruction) {
        let from_stack = &mut self.stacks[instruction.from];

        let mut crates = from_stack
            .drain((from_stack.len() - instruction.amount)..from_stack.len())
            .collect::<Vec<char>>();

        if self.multi_pick == false {
            crates.reverse();
        }

        self.stacks[instruction.to].extend(crates);
    }

    pub fn get_top_row(&self) -> String {
        let mut row = String::with_capacity(self.stacks.capacity());
        for stack in &self.stacks {
            row.push(stack.last().unwrap().clone());
        }
        return row;
    }
}

#[derive(Debug)]
struct Instruction {
    pub from: usize,
    pub to: usize,
    pub amount: usize,
}

fn create_harbour_from_initial_state(state: &str, enable_multi_pick: bool) -> Harbour {
    let lines: Vec<&str> = state.lines().rev().skip(1).collect();

    // The towers can increase above their original in height
    let stack_capacity = lines.len() * 3;
    // /4, because <space> + [ + <char> + ] per row. And +1 because <space> is missing for the first row.
    let stack_amount = (lines[0].len() + 1) / 4;

    let mut harbour = Harbour::new(stack_capacity, stack_amount, enable_multi_pick);

    for line in lines {
        let chars: Vec<char> = line.chars().collect();

        for stack in 0..stack_amount {
            let character = chars[1 + (stack * 4)];

            if character.is_whitespace() == false {
                harbour.add_crate(stack, character);
            }
        }
    }

    return harbour;
}

fn prepare_instructions<'a>(instructions: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    return instructions
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|parts| {
            let amount = parts[1].parse::<usize>().unwrap();
            // TODO: Rather one more array to save the -1 here?
            let from = parts[3].parse::<usize>().unwrap() - 1;
            let to = parts[5].parse::<usize>().unwrap() - 1;

            Instruction { from, to, amount }
        });
}

pub fn part_one() -> String {
    let (state, instructions) = INPUT.split_once("\n\n").unwrap();

    let mut harbour = create_harbour_from_initial_state(state, false);

    let instructions = prepare_instructions(instructions);

    for instruction in instructions {
        harbour.apply_instruction(&instruction);
    }

    return harbour.get_top_row();
}

pub fn part_two() -> String {
    let (state, instructions) = INPUT.split_once("\n\n").unwrap();

    let mut harbour = create_harbour_from_initial_state(state, true);

    let instructions = prepare_instructions(instructions);

    for instruction in instructions {
        harbour.apply_instruction(&instruction);
    }

    return harbour.get_top_row();
}
