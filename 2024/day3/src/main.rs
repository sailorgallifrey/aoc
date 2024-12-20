use regex::{Match, Regex};
use std::time::Instant;
use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Couldn't read file.");

    let mut before = Instant::now();
    problem1(&data);
    println!("Elapsed time: {:.2?}", before.elapsed());

    before = Instant::now();
    problem2(&data);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn problem2(data: &str) {
    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").expect("Failed to parse Regex");
    let mut total: isize = 0;
    let mut include: bool = true;
    re.find_iter(data).for_each(|m| match m.as_str() {
        "don't()" => {
            include = false;
        }
        "do()" => {
            include = true;
        }
        _ => {
            if include {
                let nums = parse_nums(m);
                total += nums[0] * nums[1]
            }
        }
    });
    println!("{:?}", total);
}

fn problem1(data: &str) {
    let re = Regex::new(r"mul\(\d+,\d+\)").expect("Failed to parse Regex");
    let r: isize = re
        .find_iter(data)
        .map(|m| {
            let nums: Vec<isize> = parse_nums(m);
            nums[0] * nums[1]
        })
        .sum();
    println!("{:?}", r);
}

fn parse_nums(m: Match) -> Vec<isize> {
    let nums: Vec<isize> = m
        .as_str()
        .replace("mul(", "")
        .replace(")", "")
        .split(",")
        .map(|n| n.parse::<isize>().expect("Failed to parse int"))
        .collect();
    nums
}
