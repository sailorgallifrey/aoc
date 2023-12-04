use std::fs;
use std::slice::Iter;

fn main() {
    

    let data = fs::read_to_string("./data.txt")
        .expect("Couldn't read file.");

    let map: Vec<Vec<char>> = get_map(data);
    let mut total: u32 = 0;

    for i in 0..map.len() {
        let mut j: usize = 0;
        while j < map[0].len() {
            let starting_char: char = map[i][j];

            if starting_char.is_numeric() {
                let (include, num_str) = check_for_number(&map, i, &mut j, starting_char);

                if include {
                    total += num_str.parse::<u32>().unwrap();
                }
                j = j + num_str.len() ;

            } else {
                j += 1;
            }
        }
    }

    println!("{:?}", total);
}

fn check_for_number(map: &Vec<Vec<char>>, i: usize, j: &mut usize, starting_char: char) -> (bool, String) {
    let mut current_char: char = starting_char;
    let mut include: bool = false;
    let mut j2: usize = *j;
    let mut num_str: String = String::from("");

    while j2 < map[0].len() {
        if j2 != *j {
            current_char = map[i][j2];
        }
        if current_char.is_numeric() {
            num_str.push(current_char);

            for (k, l) in get_queens_case(i, j2, map[0].len(), map.len()) {
                if !map[k][l].is_numeric() && !map[k][l].eq(&'.') {
                    include = true;
                    break;
                }
            }
        } else { break; }
        j2 += 1;
    }
    (include, num_str)
}

fn get_map(data: String) -> Vec<Vec<char>> {
    data.lines()
        .map(|l| l.chars().collect())
        .collect()
}

// Could have just made this loop and look for adjacent stuff
// with some min/max thresholds, but I stared one way and just went with it lol :)
fn get_queens_case(i: usize, j: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut window: Vec<(usize, usize)> = vec![];

    if i > 0 {
        if j > 0 {
            window.push((i - 1, j - 1));
        }
        window.push((i - 1, j));
        if j < w - 1{
            window.push((i - 1, j + 1))
        }
    }
    if j > 0 {
        window.push((i, j - 1));
    }
    if j < w - 1 {
        window.push((i, j + 1));
    }
    if i < h - 1 && j > 0 {
        window.push((i + 1, j - 1));
    }
    if i < h - 1 {
        window.push((i + 1, j));
    }
    if i < h - 1 && j < w - 1 {
        window.push((i + 1, j + 1));
    }
    window
}
