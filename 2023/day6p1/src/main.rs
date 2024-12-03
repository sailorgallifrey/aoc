use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct Round {
    time: u32,
    distance: u32,
}

fn main() {
    let file = fs::read_to_string("./data.txt").expect("Couldn't read file.");
    let lines: Vec<&str> = file.lines().collect();

    let rounds: Vec<Round> = get_rounds(lines);

    let result = get_result(rounds);

    println!("{:?}", result);
}

fn get_result(rounds: Vec<Round>) -> u32 {
    let round_results: Vec<u32> = rounds.iter().map(|r| get_result_for_round(r))
        .collect::<Vec<u32>>();
    let result: u32 = round_results.iter().cloned().reduce(|w1, w2| (w1 * w2)).expect("Could not calculate round total");
    result
}

fn get_result_for_round(round: &Round) -> u32 {
    let mut ways: u32 = 0;
    let mut s: u32 = 0;

    let mut d: u32 = 0;
    while ways == 0 || d > round.distance {
        d = s * (round.time - s);
        if d > round.distance {
            ways += 1;
        }

        s += 1;
    }
    ways
}

fn get_rounds(lines: Vec<&str>) -> Vec<Round> {
    let times: Vec<u32> = lines[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().expect("failed to parse time"))
        .collect();

    lines[1]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().expect("failed to parse distance"))
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
        .map(|(i, d)| Round {
            time: times[i],
            distance: *d,
        })
        .collect()
}
