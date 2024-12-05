use std::fs;
use std::io::BufRead;
use std::time::Instant;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Couldn't read file.");
    let patterns: Vec<Vec<Vec<char>>> = get_map(data);

    let mut before = Instant::now();
    problem1(&patterns);
    println!("Elapsed time: {:.2?}", before.elapsed());

    // before = Instant::now();
    // problem2(&patterns);
    // println!("Elapsed time: {:.2?}", before.elapsed());
}

fn problem1(patterns: &Vec<Vec<Vec<char>>>) {
    let mut total: usize = 0usize;
    for pattern in patterns {
        let vert: isize = find_plane(transpose(pattern.clone()));
        let horz: isize = find_plane(pattern.clone());

        if horz > 0 {
            total += (horz * 100) as usize;
        } else if vert > 0 {
            total += vert as usize
        }
    }
    println!("{:?}", total)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn find_plane(pattern: Vec<Vec<char>>) -> isize {
    let mut row: isize = -1;
    for i in 0..pattern.len() - 1 {
        let mut diff: usize = 0usize;
        for n in 0..(pattern.len() - i - 1) {
            if (i as isize - n as isize) < 0 {continue;}
            if pattern[i - n] != pattern[i + n + 1] {
                diff += 1usize;
            }
        }
        if diff == 0 {
            row = i as isize + 1;
        }
    }
    row
}

fn get_map(data: String) -> Vec<Vec<Vec<char>>> {
    let mut result: Vec<Vec<Vec<char>>> = vec![];
    let mut current: Vec<Vec<char>> = vec![];
    data
        .lines()
        .for_each(|l| {
            if l.len() == 0 {
                if current.len() > 0 {
                    result.push(current.clone())
                }
                current = vec![]
            } else {
                current.push(l.chars().collect())
            }
        });
    result.push(current.clone()); // Push last pattern
    result
}