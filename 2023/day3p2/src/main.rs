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
            let result: (bool, u32) = check_for_ratio(&map, i, j, map[i][j]);

            if result.0 {
                total += result.1;
            }
            j += 1 ;
        }
    }

    println!("{:?}", total);
}

fn check_for_ratio(map: &Vec<Vec<char>>, i: usize, j: usize, current_char: char) -> (bool, u32) {
    if current_char.eq(&'*') {
        let nums: Vec<u32> = find_surrounding_numbers(i, j, map);

        if nums.len() == 2 {
            return (true, nums[0] * nums[1])
        } else { return (false, 0) }
    }

    (false, 0)
}

fn find_surrounding_numbers(i: usize, j: usize, map: &Vec<Vec<char>>) -> Vec<u32> {
    let qc = get_queens_case_plus_current(i, j, map[0].len(), map.len());
    let mut nums: Vec<u32> = vec![];

    let mut used_locations: Vec<(usize, usize)> = vec![];
    for (k, l) in qc {
        let mut num_str = String::from("");
        if map[k][l].is_numeric() && !used_locations.contains(&(k, l)) {
            let mut l1 = l;
            loop { // find number's left boundary
                if l1 > 0 && map[k][l1 - 1].is_numeric() {
                    l1 -= 1;
                    continue;
                } else {
                    used_locations.push((k,l1));
                    num_str.push(map[k][l1]);
                    break
                }
            }
            loop { // move right
                l1 += 1;
                if l1 < map[k].len() && map[k][l1].is_numeric() {
                    used_locations.push((k, l1));
                    num_str.push(map[k][l1]);
                } else { break }
            }

            nums.push(num_str.parse::<u32>().expect("Unable to parse number"))
        }
    }

    nums
}

fn get_map(data: String) -> Vec<Vec<char>> {
    data.lines()
        .map(|l| l.chars().collect())
        .collect()
}

// Could have just made this loop and look for adjacent stuff
// with some min/max thresholds, but I stared one way and just went with it lol :)
fn get_queens_case_plus_current(i: usize, j: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut window: Vec<(usize, usize)> = vec![];

    window.push((i, j));
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
