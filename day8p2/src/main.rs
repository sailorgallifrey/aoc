use std::collections::HashMap;
use std::fs;

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
enum Direction {
    L,
    R,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("found a bad direction"),
        }
    }
}

#[derive(Clone, Debug)]
struct Location {
    address: Box<str>,
    destinations: HashMap<Direction, Box<str>>,
}

struct Problem {
    directions: Vec<Direction>,
    locations: Vec<Location>,
}
fn main() {
    let file = fs::read_to_string("./data.txt").expect("Couldn't read file.");
    let lines: Vec<&str> = file.lines().collect();

    let problem: Problem = get_problem(lines);

    let result: u64 = get_result(problem);

    println!("{:?}", result);
}

fn get_problem(lines: Vec<&str>) -> Problem {
    let directions: Vec<Direction> = lines[0]
        .chars()
        .map(|c| c.into())
        .collect::<Vec<Direction>>()
        .try_into()
        .unwrap();

    let mut locations: Vec<Location> = vec![];

    for i in 2..lines.len() {
        let parts: Vec<&str> = lines[i].split(" = ").collect();
        let address: &str = parts[0];

        let destinations: Vec<Box<str>> = parts[1]
            .replace("(", "")
            .replace(")", "")
            .split(", ")
            .map(|s| Box::from(s))
            .collect();

        locations.push(Location {
            address: Box::from(address),
            destinations: HashMap::from([
                (Direction::L, destinations[0].clone()),
                (Direction::R, destinations[1].clone()),
            ]),
        });
    }

    Problem {
        directions,
        locations,
    }
}

fn get_result(problem: Problem) -> u64 {
    let counts: Vec<u64> = find_starting_locations(&problem, 'A')
        .iter().map(|l| get_count(&l, &problem)).collect();

    least_common_multiple(counts)
}

fn least_common_multiple(mut n: Vec<u64>) -> u64 {
    if n.len() == 1 {
        return n[0]
    }

    let a = n.remove(0);
    let b = least_common_multiple(n);
    a * b / greatest_common_devisor(a, b)
}

fn greatest_common_devisor(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    greatest_common_devisor(b, a % b)
}

fn get_count(loc: &Location, problem: &Problem) -> u64 {
    let mut location: Location = loc.clone();
    let mut found: bool = false;

    let mut result: u64 = 0;

    while !found {
        for d in &problem.directions {
            match d {
                Direction::L => {
                    location = find_location(&problem, &location.destinations[&Direction::L]);
                }
                Direction::R => {
                    location = find_location(&problem, &location.destinations[&Direction::R]);
                }
            }
            result += 1;

            if location.address.ends_with('Z') {
                found = true;
                break;
            }
        }
    }

    result
}

fn find_location(problem: &Problem, add: &Box<str>) -> Location {
    problem
        .locations
        .iter()
        .filter(|n| n.address == *add)
        .map(|l| l.clone())
        .collect::<Vec<Location>>()
        .first()
        .unwrap()
        .clone()
}

fn find_starting_locations(problem: &Problem, suffix: char) -> Vec<Location> {
    problem
        .locations
        .iter()
        .filter(|n| n.address.chars().last().unwrap() == suffix)
        .map(|l| l.clone())
        .collect()
}