use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    num::ParseIntError,
    rc::Rc,
    str::FromStr,
    time::Instant,
};

static INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Valve {
    pub name: String,
    pub flow_rate: usize,
    pub tunnels: Vec<String>,
    // TODO: On Outside, so that clones do not hurt?
    pub distance_to: HashMap<String, usize>,
}

impl FromStr for Valve {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (definition, tunnels) = s.split_once(';').unwrap();
        let (pretext, flow_rate) = definition.split_once('=').unwrap();

        let (_, tunnels) = tunnels.split_once("valve").unwrap();
        // Skip leading 's' of 'valves' if there are multiple
        let tunnels: Vec<_> = tunnels[1..]
            .split(',')
            .map(|tunnel| String::from(tunnel.trim()))
            .collect();

        Ok(Valve {
            name: pretext[6..8].to_string(),
            flow_rate: flow_rate.parse::<usize>().unwrap(),
            tunnels,
            distance_to: HashMap::new(),
        })
    }
}

#[derive(Debug)]
struct Path {
    pub current_valve: Rc<RefCell<Valve>>,
    pub accumulated_flow: usize,
    pub valves_to_visit: Vec<Rc<RefCell<Valve>>>,
    pub remaining_minutes: usize,
    pub previous_valves: Vec<String>,
}

struct Iteration {
    tunnels: Vec<String>,
    current_distance: usize,
}

fn calculate_distances(
    valves: &Vec<Rc<RefCell<Valve>>>,
    valves_map: &HashMap<String, Rc<RefCell<Valve>>>,
) {
    let to_find_for_all: HashSet<String> =
        HashSet::from_iter(valves.iter().map(|valve| valve.borrow().name.clone()));

    // TODO: Optimize by dijkstra?
    for valve in valves {
        let mut to_find = to_find_for_all.clone();
        let mut current_valve = valve.borrow_mut();

        to_find.remove(&current_valve.name);

        // TODO: Maybe move inside function
        let tunnels_to_follow: Vec<_> = current_valve.tunnels.to_vec();

        // TODO: Maybe move inside function
        for tunnel in &tunnels_to_follow {
            to_find.remove(tunnel);
            current_valve.distance_to.insert(tunnel.clone(), 1);
        }

        let mut to_follow: Vec<Iteration> = vec![Iteration {
            tunnels: tunnels_to_follow,
            current_distance: 1,
        }];

        while let Some(iteration) = to_follow.pop() {
            for j in 0..iteration.tunnels.len() {
                let current = valves_map
                    .get(iteration.tunnels[j].as_str())
                    .unwrap()
                    .borrow();

                let tunnels_to_follow: Vec<_> = current
                    .tunnels
                    .iter()
                    .filter(|tunnel| to_find.contains(*tunnel))
                    .cloned()
                    .collect();

                for tunnel in &tunnels_to_follow {
                    to_find.remove(tunnel);
                    current_valve
                        .distance_to
                        .insert(tunnel.clone(), iteration.current_distance + 1);
                }

                to_follow.push(Iteration {
                    tunnels: tunnels_to_follow,
                    current_distance: iteration.current_distance + 1,
                });
            }
        }
    }
}

fn find_max_releasable_pressure(
    start_valve: String,
    valves: &[Rc<RefCell<Valve>>],
    valves_map: &HashMap<String, Rc<RefCell<Valve>>>,
) -> usize {
    let relevant_valves: Vec<_> = valves
        .iter()
        .filter(|valve| valve.borrow().flow_rate > 0)
        .map(Rc::clone)
        .collect();

    let mut paths: Vec<Path> = Vec::from([Path {
        current_valve: Rc::clone(valves_map.get(&start_valve).unwrap()),
        valves_to_visit: relevant_valves,
        accumulated_flow: 0,
        remaining_minutes: 30,
        previous_valves: vec![],
    }]);

    let mut max: usize = 0;

    while let Some(path) = paths.pop() {
        let current_valve = path.current_valve.borrow();

        for (i, path_valve) in path.valves_to_visit.iter().enumerate() {
            let valve = path_valve.borrow();
            let steps_to_valve = *current_valve.distance_to.get(&valve.name).unwrap();

            // It takes one more minute to open the valve.
            let cost = steps_to_valve + 1;
            let remaining_minutes = path.remaining_minutes.saturating_sub(cost);
            let accumulated_flow = path.accumulated_flow + (valve.flow_rate * remaining_minutes);

            if path.valves_to_visit.len() == 1 || remaining_minutes == 0 {
                if accumulated_flow > max {
                    max = accumulated_flow;
                    // path.previous_valves.push(valve.name.clone());
                    // println!("{:?}", path.previous_valves);
                }

                continue;
            }

            let mut valves_to_visit = path.valves_to_visit.clone();
            valves_to_visit.remove(i);

            // let mut previous_valves = path.previous_valves.clone();
            // previous_valves.push(current_valve.name.clone());

            paths.push(Path {
                current_valve: Rc::clone(path_valve),
                remaining_minutes,
                accumulated_flow,
                valves_to_visit,
                previous_valves: vec![],
            });
        }
    }

    max
}

pub fn part_one() {
    let now = Instant::now();

    let valves: Vec<_> = INPUT
        .lines()
        .map(|line| line.parse::<Valve>().unwrap())
        .map(|valve| Rc::new(RefCell::new(valve)))
        .collect();

    let start_valve = String::from("AA");
    // let start_valve = valves[0].borrow().name.clone();

    let valves_map: HashMap<String, Rc<RefCell<Valve>>> = valves
        .iter()
        .map(|valve| (valve.borrow().name.clone(), Rc::clone(valve)))
        .collect();

    println!("setup: {:.2?}", now.elapsed());

    calculate_distances(&valves, &valves_map);

    println!("Distances: {:.2?}", now.elapsed());

    let max = find_max_releasable_pressure(start_valve, &valves, &valves_map);

    // 1838 too low, 1879, 1880 too high
    // => 1857 => What am i missing?
    println!("{max}");
}
