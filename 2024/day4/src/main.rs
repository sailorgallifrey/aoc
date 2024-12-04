use std::fs;
use std::time::Instant;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Couldn't read file.");
    let map: Vec<Vec<char>> = get_map(data);

    let mut before = Instant::now();
    problem1(&map);
    println!("Elapsed time: {:.2?}", before.elapsed());

    before = Instant::now();
    problem2(&map);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn problem2(map: &[Vec<char>]) {
    let mut total: usize = 0usize;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            total += check_map_problem_2(map, i, j);
        }
    }

    println!("{:?}", total)
}

fn check_map_problem_2(map: &[Vec<char>], i: usize, j: usize) -> usize {
    if map[i][j] == 'A'
        && i > 0
        && j > 0
        && i < map.len() - 1
        && j < map[i].len() - 1
        && ((map[i - 1][j - 1] == 'M' && map[i + 1][j + 1] == 'S')
            || (map[i - 1][j - 1] == 'S' && map[i + 1][j + 1] == 'M'))
        && ((map[i + 1][j - 1] == 'M' && map[i - 1][j + 1] == 'S')
            || (map[i + 1][j - 1] == 'S' && map[i - 1][j + 1] == 'M'))
    {
        1
    } else {
        0
    }
}

fn problem1(map: &[Vec<char>]) {
    let xmas: [char; 4] = ['X', 'M', 'A', 'S'];
    let mut total: usize = 0usize;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            total += check_map_problem_1(map, xmas, 0, i, j);
        }
    }

    println!("{:?}", total)
}

fn check_map_problem_1(
    map: &[Vec<char>],
    xmas: [char; 4],
    found_count: usize,
    i: usize,
    j: usize,
) -> usize {
    let mut new_total = 0;

    if map[i][j] == xmas[found_count] && found_count < xmas.len() {
        new_total += check_location(map, xmas, found_count + 1, i, j, minus_1, minus_1);
        new_total += check_location(map, xmas, found_count + 1, i, j, minus_1, same);
        new_total += check_location(map, xmas, found_count + 1, i, j, minus_1, plus_1);
        new_total += check_location(map, xmas, found_count + 1, i, j, same, minus_1);
        new_total += check_location(map, xmas, found_count + 1, i, j, same, plus_1);
        new_total += check_location(map, xmas, found_count + 1, i, j, plus_1, minus_1);
        new_total += check_location(map, xmas, found_count + 1, i, j, plus_1, same);
        new_total += check_location(map, xmas, found_count + 1, i, j, plus_1, plus_1);
    }
    new_total
}

fn check_location(
    map: &[Vec<char>],
    xmas: [char; 4],
    found_count: usize,
    i: usize,
    j: usize,
    f_i: fn(usize, usize) -> (usize, bool),
    f_j: fn(usize, usize) -> (usize, bool),
) -> usize {
    let new_i = f_i(i, map.len());
    let new_j = f_j(j, map[i].len());

    if !new_i.1 || !new_j.1 {
        return 0;
    }

    if map[new_i.0][new_j.0] == xmas[found_count] {
        return if found_count == xmas.len() - 1 {
            1
        } else {
            check_location(map, xmas, found_count + 1, new_i.0, new_j.0, f_i, f_j)
        }
    }
    0
}

fn get_map(data: String) -> Vec<Vec<char>> {
    data.lines().map(|l| l.chars().collect()).collect()
}

fn plus_1(v: usize, max: usize) -> (usize, bool) {
    if v + 1 < max {
        (v + 1, true)
    } else {
        (v, false)
    }
}

fn minus_1(v: usize, _: usize) -> (usize, bool) {
    if v > 0 {
        (v - 1, true)
    } else {
        (v, false)
    }
}

fn same(v: usize, _: usize) -> (usize, bool) {
    (v, true)
}
