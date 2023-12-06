use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct Round {
    time: u64,
    distance: u64,
}

fn main() {
    let file = fs::read_to_string("./data.txt").expect("Couldn't read file.");
    let lines: Vec<&str> = file.lines().collect();

    let round: Round = get_round(lines);

    let result = get_result_for_round(&round);

    println!("{:?}", result);
}

fn get_result_for_round(round: &Round) -> u64 {
    let mut ways: u64 = 0;
    let mut s: u64 = 0;

    let mut d: u64 = 0;
    while ways == 0 || d > round.distance {
        d = s * (round.time - s);
        if d > round.distance {
            ways += 1;
        }

        s += 1;
    }
    ways
}

fn get_round(lines: Vec<&str>) -> Round {
    let time: u64 = lines[0]
        .split(':')
        .last()
        .unwrap()
        .replace(" ", "")
        .parse::<u64>().expect("Unable to parse time");

    let dist: u64 = lines[1]
        .split(':')
        .last()
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .expect("Unable to parse distance");

    Round{time, distance:dist}
}
