use regex::Regex;
use std::time::Instant;
use std::{fs, isize};

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Couldn't read file.");

    let mut before = Instant::now();
    problem1(&data);
    println!("Elapsed time: {:.2?}", before.elapsed());

    before = Instant::now();
    problem2(&data);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn problem2(data: &String) {
    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    let mut total: isize = 0;
    let mut include: bool = true;
    re.find_iter(data.as_str()).for_each(|m| match m.as_str() {
        "don't()" => {
            include = false;
        }
        "do()" => {
            include = true;
        }
        _ => {
            if include {
                let nums: Vec<isize> = m
                    .as_str()
                    .replace("mul(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect();
                total += nums[0] * nums[1]
            }
        }
    });
    println!("{:?}", total);
}

fn problem1(data: &String) {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let r: isize = re
        .find_iter(data.as_str())
        .map(|m| {
            let nums: Vec<isize> = m
                .as_str()
                .replace("mul(", "")
                .replace(")", "")
                .split(",")
                .map(|n| n.parse::<isize>().unwrap())
                .collect();
            nums[0] * nums[1]
        })
        .sum();
    println!("{:?}", r);
}
