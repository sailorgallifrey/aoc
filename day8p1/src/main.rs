use std::collections::HashMap;
use std::fs;
use std::ops::Add;

#[derive(Hash, Eq, PartialEq, Clone)]
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

#[derive(Clone, Eq, PartialEq)]
struct Address {
    chars: [char; 3],
}

impl From<[char; 3]> for Address {
    fn from(value: [char; 3]) -> Self {
        Address { chars: value }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Map {
    address: Address,
    destinations: HashMap<Direction, Address>,
}

struct Problem {
    directions: Vec<Direction>,
    maps: Vec<Map>,
}
fn main() {
    let file = fs::read_to_string("./data-test.txt").expect("Couldn't read file.");
    let lines: Vec<&str> = file.lines().collect();

    let problem: Problem = get_problem(lines);

    let result: u32 = get_result(problem);

    println!("{:?}", result);
}

fn get_problem(lines: Vec<&str>) -> Problem {
    let directions: Vec<Direction> = lines[0]
        .chars()
        .map(|c| c.into())
        .collect::<Vec<Direction>>()
        .try_into()
        .unwrap();

    let mut maps: Vec<Map> = vec![];

    for i in 2..lines.len() {
        let parts: Vec<&str> = lines[i].split(" = ").collect();
        let address: [char; 3] = parts[0].chars().collect::<Vec<char>>().try_into().unwrap();

        let destinations: Vec<Address> = parts[1]
            .replace("(", "")
            .replace(")", "")
            .split(", ")
            .map(|s| Address {
                chars: s.chars().collect::<Vec<char>>().try_into().unwrap(),
            })
            .collect();

        maps.push(Map {
            address: address.try_into().unwrap(),
            destinations: HashMap::from([
                (Direction::L, destinations[0].clone()),
                (Direction::R, destinations[1].clone()),
            ]),
        });
    }

    Problem { directions, maps }
}

fn get_result(problem: Problem) -> u32 {
    let zzz: Address = Address {
        chars: ['Z', 'Z', 'Z'],
    };

    let target: Address = problem.maps[0].clone().address;
    let found: Vec<(Map, Direction)> = vec![];


    return 0;
}
