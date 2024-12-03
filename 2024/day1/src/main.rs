use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Couldn't read file.");
    let (a, b): (Vec<isize>, Vec<isize>) = parse_data(data);

    problem1(&a, &b);
    problem2(&a, &b);
}

fn problem2(a: &[isize], b: &[isize]) {
    let r: isize = a.iter().fold(0isize, |r, v| r + (v * count_occurrences(v, b)));

    println!("{:?}", r)
}

fn count_occurrences(v: &isize, b: &[isize]) -> isize {
    b.iter().filter(|&v1| *v1 == *v).count() as isize
}

fn problem1(a: &[isize], b: &[isize]) {
    let r: isize = a
        .iter()
        .enumerate()
        .fold(0isize, |r, (i, v)| r + (v - b[i]).abs());

    println!("{:?}", r)
}

fn parse_data(data: String) -> (Vec<isize>, Vec<isize>) {
    let mut a = Vec::new();
    let mut b = Vec::new();
    data.lines().for_each(|l| {
        let d: Vec<isize> = l
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        a.push(d[0]);
        b.push(d[1]);
    });

    a.sort();
    b.sort();

    (a, b)
}
