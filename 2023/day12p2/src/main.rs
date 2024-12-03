use crate::Status::{Broken, Unknown, Working};
use std::fs;
use memoize::memoize;

#[derive(Hash, Clone, Debug, Eq, PartialEq, Copy)]
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

fn main() {
    let data = fs::read_to_string("./data.txt").expect("Couldn't read file.");

    let map: Vec<(Vec<u8>, Vec<Status>)> = get_map(data);
    let mut total: u64 = 0;

    for l in map {
        total += get_result(l.0, l.1);
    }

    println!("total: {:?}", total);
}

fn get_map(data: String) -> Vec<(Vec<u8>, Vec<Status>)> {
    data.lines().map(get_parts).collect()
}

fn get_parts(l: &str) -> (Vec<u8>, Vec<Status>) {
    let parts = l.split_whitespace().collect::<Vec<&str>>();

    let counts: Vec<u8> = parts
    .last()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>().repeat(5);

    let part2_chars: Vec<char> = parts
        .first()
        .unwrap()
        .chars().collect();

    let mut part2: Vec<Status> = part2_chars
        .iter()
        .map(|c| <char as Into<Status>>::into(*c))
        .collect();

    for _i in 0..4 {
        part2.push(Unknown);
        for char in &part2_chars {
            part2.push((*char).into())
        }
    }

    (counts, part2)
}

#[memoize] // If we've seen this pattern before just return what we have in cache.
fn get_result(groups: Vec<u8>, springs: Vec<Status>) -> u64 {
    if groups.is_empty() {
        return if !springs.iter().any(|s| s == &Broken) {
            1
        } else { 0 };
    }

    if springs.is_empty() {
        return 0
    }

    let next_status = springs[0];
    let next_group = groups[0];

    if next_status == Working {
        working(&groups, &springs) // Next status is working and can act as a separator so we keep going
    } else if next_status == Broken {
        // Ooo we found some broken springs check if they match the next group
        broken(&groups, &springs, next_group)
    } else {
        // Ooo wildcard (See if it helps us match the next group or keep going assuming the wildcard is a separator and sum the two)
        working(&groups, &springs) + broken(&groups, &springs, next_group)
    }
}

fn working(groups: &[u8], springs: &[Status]) -> u64 {
    get_result(groups.to_owned(), springs[1..].to_vec())
}

fn broken(groups: &[u8], springs: &[Status], next_group: u8) -> u64 {
    let mut current_group: Vec<Status> = if springs.len() < next_group as usize
    {
        springs.to_owned()
    } else { springs[..next_group as usize].to_vec() };

    // Replace Unknown with Broken
    current_group = current_group.iter()
        .map(|s| if *s == Unknown { Broken } else { *s }).collect();

    // This group isn't a match
    if current_group != vec![Broken; next_group as usize] {
        return 0
    }

    // We're on the last part of the springs list
    if springs.len() == next_group as usize {
        if groups.len() == 1 { // Yay we found one
            return 1;
        } else { // Oops we still have groups left
            return 0;
        }
    }

    if [Unknown, Working].contains(&springs[next_group as usize]) {
        // If the next item in springs after this group is a separator we can keep going
        get_result(groups[1..].to_vec(), springs[next_group as usize + 1..].to_vec())
    } else { 0 }
}
