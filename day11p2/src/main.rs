use crate::Type::{Empty, Expanded, Galaxy};
use std::collections::HashMap;
use std::f32::consts::E;
use std::fs;
use std::io::Write;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Type {
    Galaxy,
    Empty,
    Expanded,
}

impl From<char> for Type {
    fn from(value: char) -> Self {
        match value {
            '#' => Galaxy,
            '.' => Empty,
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

    let map: Vec<Vec<Type>> = get_map(data);
    let expanded: Vec<Vec<Type>> = expand_map(&map);
    let mut total: u64 = 0;
    let mut pair_count: u32 = 0;

    let galaxies: Vec<Coord> = get_galaxies(&expanded);
    let mut visited: Vec<u32> = vec![];

    for g1 in &galaxies {
        print!("\r{}    ", g1.id);
        std::io::stdout().flush();
        for g2 in &galaxies {
            if g1.i == g2.i && g1.j == g2.j || visited.contains(&g2.id) {
                continue;
            }
            total += ((g1.i as i64 - g2.i as i64).abs() + (g1.j as i64 - g2.j as i64).abs()) as u64;
            pair_count += 1;
        }
        visited.push(g1.id);
    }

    println!("total: {:?} pair_count: {:?}", total, pair_count);
}

fn get_map(data: String) -> Vec<Vec<Type>> {
    data.lines()
        .map(|l| l.chars().map(|c| c.try_into().unwrap()).collect())
        .collect()
}

fn expand_map(map: &Vec<Vec<Type>>) -> Vec<Vec<Type>> {
    let mut result: Vec<Vec<Type>> = vec![];
    let mut result_i: Vec<Vec<Type>> = vec![];

    for i in 0..map.len() {
        if map[i].iter().all(|j| j == &Empty) {
            result_i.push(vec![Expanded, map[i].len()])
        } else {
            result_i.push(map[i].clone());
        }
    }

    result_i = transpose2(result_i);

    for i in 0..result_i.len() {
        if result_i[i].iter().all(|j| j == &Empty) {
            result_i.push(vec![Expanded, result_i[i].len()])
        } else {
            result.push(result_i[i].clone());
        }
    }

    transpose2(result)
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn get_galaxies(map: &Vec<Vec<Type>>) -> Vec<Coord> {
    let mut result: Vec<Coord> = vec![];
    let mut id: u32 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == Galaxy {
                id += 1;
                result.push(Coord { i, j, id })
            }
        }
    }
    result
}
