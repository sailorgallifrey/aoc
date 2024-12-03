use std::fs;

#[derive(Clone, Debug)]
struct Coord {
    i: usize,
    j: usize,
    value: char,
}

fn main() {
    let data = fs::read_to_string("./data.txt").expect("Couldn't read file.");

    let map: Vec<Vec<char>> = get_map(data);
    let mut total: u32 = 0;

    let start: Coord = get_start(&map);
    let mut current: Coord = start.clone();
    let mut prior: Option<Coord> = None;
    let mut route: Vec<Coord> = vec![start.clone()];

    loop {
        let next: Coord = get_next(&map, &current.clone(), &prior);
        total += 1;
        if route.len() > 1 && current.value == start.value {
            break;
        } else {
            route.push(next.clone());
            prior = Some(current.clone());
            current = next.clone();
        }
    }

    println!("{:?}", total / 2);
}

fn get_map(data: String) -> Vec<Vec<char>> {
    data.lines().map(|l| l.chars().collect()).collect()
}
fn get_start(map: &Vec<Vec<char>>) -> Coord {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                return Coord { i, j, value: 'S' };
            }
        }
    }
    panic!("Couldn't find S");
}

fn get_next(map: &Vec<Vec<char>>, current: &Coord, prior: &Option<Coord>) -> Coord {
    for (k, l) in get_rooks_case(current.i, current.j, map[0].len(), map.len()) {
        if prior.is_some() {
            let p = prior.clone().unwrap();
            if p.i == k && p.j == l {
                continue;
            }
        }
        if current.i == k && current.j == l {
            continue;
        }
        let result: Coord = Coord {
            i: k,
            j: l,
            value: map[k][l],
        };
        if check_coord(&map, &result, &current.clone()) {
            return result;
        }
    }

    panic!("couldn't find next")
}

fn check_coord(map: &&Vec<Vec<char>>, current: &Coord, prior: &Coord) -> bool {
    let mut count: u8 = 0;

    for (k, l) in get_rooks_case(current.i, current.j, map[0].len(), map.len()) {
        if prior.i == k && prior.j == l {
            continue;
        }
        let result: Coord = Coord {
            i: k,
            j: l,
            value: map[k][l],
        };
        if connects(current, &result) {
            count += 1;
        }
    }

    count == 1 && connects(current, &prior)
}

fn connects(c1: &Coord, c2: &Coord) -> bool {
    let mut result = false;
    match c1.value {
        'S' => {
            result = up(c1, c2).contains(&c2.value)
                || down(c1, c2).contains(&c2.value)
                || left(c1, c2).contains(&c2.value)
                || right(c1, c2).contains(&c2.value)
        }
        '|' => {
            if up(c1, c2).contains(&c2.value) {
                result = true;
            } else if down(c1, c2).contains(&c2.value) {
                result = true;
            }
        }
        '-' => {
            if left(c1, c2).contains(&c2.value) {
                result = true;
            } else if right(c1, c2).contains(&c2.value) {
                result = true;
            }
        }
        'L' => {
            if right(c1, c2).contains(&c2.value) {
                result = true;
            } else if up(c1, c2).contains(&c2.value) {
                result = true;
            }
        }
        'J' => {
            if left(c1, c2).contains(&c2.value) {
                result = true;
            } else if up(c1, c2).contains(&c2.value) {
                result = true;
            }
        }
        '7' => {
            if left(c1, c2).contains(&c2.value) {
                result = true;
            } else if down(c1, c2).contains(&c2.value) {
                result = true;
            }
        }
        'F' => {
            if right(c1, c2).contains(&c2.value) {
                result = true;
            } else if down(c1, c2).contains(&c2.value) {
                result = true;
            }
        }
        _ => (),
    }

    result
}

fn up(c1: &Coord, c2: &Coord) -> [char; 4] {
    return if c2.i == c1.i - 1 && c2.j == c1.j {
        ['|', '7', 'F', 'S']
    } else {
        ['0', '0', '0', '0']
    };
}

fn down(c1: &Coord, c2: &Coord) -> [char; 4] {
    return if c2.i == c1.i + 1 && c2.j == c1.j {
        ['|', 'L', 'J', 'S']
    } else {
        ['0', '0', '0', '0']
    };
}

fn right(c1: &Coord, c2: &Coord) -> [char; 4] {
    return if c2.i == c1.i && c2.j == c1.j + 1 {
        ['-', '7', 'J', 'S']
    } else {
        ['0', '0', '0', '0']
    };
}

fn left(c1: &Coord, c2: &Coord) -> [char; 4] {
    return if c2.i == c1.i && c2.j == c1.j - 1 {
        ['-', 'L', 'F', 'S']
    } else {
        ['0', '0', '0', '0']
    };
}

// Could have just made this loop and look for adjacent stuff
// with some min/max thresholds, but I stared one way and just went with it lol :)
fn get_rooks_case(i: usize, j: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut window: Vec<(usize, usize)> = vec![];

    if i > 0 {
        window.push((i - 1, j));
    }
    if j > 0 {
        window.push((i, j - 1));
    }
    if j < w - 1 {
        window.push((i, j + 1));
    }
    if i < h - 1 {
        window.push((i + 1, j));
    }
    window
}

// '|'
// '-'
// 'L'
// 'J'
// '7'
// 'F'
// '.'
