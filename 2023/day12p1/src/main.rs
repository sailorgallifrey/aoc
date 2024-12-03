use crate::Status::{Broken, Unknown, Working};
use std::fs;
use std::io::Write;
use itertools::{Itertools, repeat_n};
use log::debug;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Status {
    Working,
    Broken,
    Unknown,
}

impl From<char> for Status {
    fn from(value: char) -> Self {
        match value {
            '#' => Broken,
            '.' => Working,
            '?' => Unknown,
            _ => panic!("shouldn't get here"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Coord {
    i: usize,
    j: usize,
    id: u32,
}

fn main() {
    let data = fs::read_to_string("./data.txt").expect("Couldn't read file.");

    let map: Vec<(Vec<u8>, Vec<Status>)> = get_map(data);
    let mut total: u64 = 0;

    for l in map {
        total += get_arrangements(l)
    }

    println!("total: {:?}", total);
}

fn get_map(data: String) -> Vec<(Vec<u8>, Vec<Status>)> {
    data.lines().map(|l| get_parts(l)).collect()
}

fn get_parts(l: &str) -> (Vec<u8>, Vec<Status>) {
    let parts = l.split_whitespace().collect::<Vec<&str>>();

    (
        parts
            .last()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect(),
        parts
            .first()
            .unwrap()
            .chars()
            .map(|c| c.try_into().unwrap())
            .collect(),
    )
}

fn get_arrangements(map_line: (Vec<u8>, Vec<Status>)) -> u64 {
    let mut result: u64 = 0;
    let permutations: Vec<Vec<Status>> = get_permutations(map_line.clone().1);

    for permutation in permutations {
        let counts: Vec<(u8, Status)> = get_counts(permutation);
        let mut index = 0;

        let mut found: bool = true;
        for count in counts {
            if count.1 == Broken && (index >= map_line.0.len() || count.0 != map_line.0[index]) {
                found = false;
                break;
            } else if count.1 == Broken { index += 1 }
        }

        if found && index == map_line.0.len() {
            result += 1;
        }
    }

    result
}

fn get_permutations(status_list: Vec<Status>) -> Vec<Vec<Status>> {
    let mut result: Vec<Vec<Status>> = vec![];
    let unknown_count: usize = status_list
        .iter().filter(|s| **s == Unknown).count();
    let permutations: Vec<Vec<Status>> = repeat_n(1..3, unknown_count).multi_cartesian_product()
        .map(|p| p.iter().map(|n| if *n == 1 { Broken } else { Working }).collect::<Vec<Status>>())
        .collect();

    for permutation in permutations {
        let mut r: Vec<Status> = vec![];
        let mut iter = permutation.iter();
        for i in 0..status_list.len() {
            if status_list[i] == Unknown {
                r.push(iter.next().unwrap().clone().clone())
            } else {
                r.push(status_list[i].clone())
            }
        }
        result.push(r);
    }

    result
}

fn get_counts(status_list: Vec<Status>) -> Vec<(u8, Status)> {
    let mut result: Vec<(u8, crate::Status)> = vec![];
    let mut current: (u8, crate::Status) = (1, status_list.first().unwrap().clone());


    for i in 1..status_list.len() {
        if status_list[i] == current.1 {
            current = (current.0 + 1, current.1)
        } else {
            result.push(current);
            current = (1, status_list[i].clone())
        }
    }

    result.push(current);

    result
}