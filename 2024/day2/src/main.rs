use std::{fs, i32};
use std::time::Instant;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Couldn't read file.");
    let reports: Vec<Vec<i32>> = parse_data(data);

    let mut before = Instant::now();
    problem1(reports.clone());
    println!("Elapsed time: {:.2?}", before.elapsed());


    before = Instant::now();
    problem2(reports.clone());
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn problem2(reports: Vec<Vec<i32>>) {
    let r = reports
        .iter()
        .filter(|r| check_report(r, 0))
        .count();

    println!("{:?}", r)
}

fn check_report(r: &Vec<i32>, buffer_used: u8) -> bool {
    let mut positive = false;
    let mut keep = false;
    for i in 0..r.len() - 1 {
        let diff = r[i + 1] - r[i];
        let current_diff_positive = if diff > 0 { true } else { false };

        if i == 0 {
            positive = current_diff_positive;
        }

        if positive != current_diff_positive || diff == 0 || diff.abs() > 3 {
            keep = false;
            if buffer_used < 1 {
                r.iter().enumerate().for_each(|(i, _)| {
                    let mut r1 = r.to_vec();
                    r1.remove(i);
                    if check_report(&r1, buffer_used + 1) {
                        keep = true;
                    }
                });
            }
            break;
        } else {
            keep = true;
        }
    }
    // println!("{:?} {:?}", r, keep);

    keep
}

fn problem1(reports: Vec<Vec<i32>>) {
    let r = reports
        .iter()
        .filter(|r| {
            let mut current_diff = 0;
            let mut keep = true;
            for (i, level) in r.iter().enumerate().skip(1) {
                let diff = r[i - 1] - level;
                if diff == 0 || diff > 3 || diff < -3 {
                    keep = false;
                    break;
                }
                if i == 1 {
                    current_diff = diff;
                } else {
                    if current_diff > 0 && diff < 0 || current_diff < 0 && diff > 0 {
                        keep = false;
                        break;
                    } else {
                        current_diff = diff;
                        if i == r.len() - 1 {
                            keep = true;
                            break;
                        }
                    }
                }
            }
            // println!("{:?} {:?}", r, keep);

            keep
        })
        .count();

    println!("{:?}", r)
}

fn parse_data(data: String) -> Vec<Vec<i32>> {
    data.lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}
