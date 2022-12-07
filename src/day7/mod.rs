use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

static INPUT: &'static str = include_str!("input.txt");

#[derive(Debug)]
struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    size: u64,
}

fn find_directories() -> Vec<Rc<RefCell<Node>>> {
    let mut directories: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut current_parent: Option<Rc<RefCell<Node>>> = None;

    for line in INPUT.lines() {
        let mut parts = line.split_whitespace();
        let first = parts.next().unwrap();

        // Skip dir listings
        if first == "dir" {
            continue;
        }

        // Command input
        if first == "$" {
            // Either cd <x> or ls
            let command = parts.next().unwrap();

            // Skip list command
            if command == "ls" {
                continue;
            }
            let argument = parts.next().unwrap();

            if argument == ".." {
                // TODO: Beautify
                current_parent = match current_parent.as_ref() {
                    Some(current_parent) => match current_parent.borrow().parent.as_ref() {
                        Some(upper) => Some(Rc::clone(&upper)),
                        _ => None,
                    },
                    _ => None,
                };
                continue;
            }

            let parent = match current_parent.as_ref() {
                Some(current_parent) => Some(Rc::clone(&current_parent)),
                _ => None,
            };

            let new_directory = Rc::new(RefCell::new(Node { parent, size: 0 }));

            current_parent = Some(Rc::clone(&new_directory));

            directories.push(new_directory);

        // Is a file listing
        } else {
            let size = first.parse::<u64>().unwrap();

            let mut parents = match current_parent.as_ref() {
                Some(par) => Vec::from([Rc::clone(&par)]),
                _ => Vec::from([]),
            };

            // Add size to all parent directories
            let mut i = 0;
            loop {
                if i >= parents.len() {
                    break;
                }

                let current = Rc::clone(&parents[i]);
                current.borrow_mut().size += size;

                match current.borrow().parent.as_ref() {
                    Some(parent) => parents.push(Rc::clone(&parent)),
                    _ => {}
                }

                i += 1;
            }
        }
    }

    return directories;
}

pub fn part_one() -> u64 {
    let directories = find_directories();

    let total = directories
        .iter()
        .map(|dir| dir.borrow().size)
        .filter(|size| *size < 100_000)
        .sum();

    println!("{:?}", total);

    return total;
}

pub fn part_two() -> u64 {
    let directories = find_directories();

    let main_dir_size = directories[0].borrow().size;
    let free_space = 70_000_000 - main_dir_size;
    let to_free_up = 30_000_000 - free_space;

    let min_dir_size = directories
        .iter()
        .map(|dir| dir.borrow().size)
        .filter(|size| *size > to_free_up)
        .min()
        .unwrap();

    return min_dir_size;
}
