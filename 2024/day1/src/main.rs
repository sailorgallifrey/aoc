use std::{fs, i32};

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Couldn't read file.");
    let (a, b): (Vec<i32>, Vec<i32>) = parse_data(data);


    problem1(&a, &b);
    problem2(&a, &b);
}

fn problem2(a: &Vec<i32>, b: &Vec<i32>) {
    let r: i32 = a.iter()
        .fold(0i32, |r, v| r + (v *count_occurances(v, b)));

    println!("{:?}", r)
}

fn count_occurances(v: &i32, b: &Vec<i32>) -> i32 {
    b.iter()
    .filter(|&v1| *v1 == *v)
    .count() as i32
}

fn problem1(a: &Vec<i32>, b: &Vec<i32>) {

    let r: i32 = a.iter().enumerate()
        .fold(0i32, |r, (i, v)| r + (v - b[i]).abs());

    println!("{:?}", r)
}

fn parse_data(data: String) -> (Vec<i32>, Vec<i32>) {
    let mut a = Vec::new();
    let mut b = Vec::new();
    data.lines().for_each(|l| {
        let d: Vec<i32> = l.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        a.push(d[0]);
        b.push(d[1]);
    });

    a.sort();
    b.sort();

    (a, b)
}
