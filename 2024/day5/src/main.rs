use std::collections::HashMap;
use std::{cmp, fs};
use std::time::Instant;

struct Data {
    relative_location_map: HashMap<usize, (Vec<usize>, Vec<usize>)>,
    updates: Vec<Vec<usize>>
}

fn main() {
    let mut before = Instant::now();
    let s = fs::read_to_string("./input.txt").expect("Couldn't read file.");
    let data: Data = get_map(s);
    println!("Parse File Elapsed Time {:.2?}", before.elapsed());

    before = Instant::now();
    println!("Problem 1:");
    problem1(&data);
    println!("Elapsed time: {:.2?}", before.elapsed());

    println!("Problem 2:");
    before = Instant::now();
    problem2(&data);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn problem2(data: &Data) {
    let total: usize = data.updates
        .iter().filter(|u| {
        for i in 0..u.len() {
            let n: usize = u[i];
            let relative_locations: (Vec<usize>, Vec<usize>) = data.relative_location_map[&n].clone();

            for j in 0..u.len() {
                if j == i {
                    continue;
                }

                if j < i && relative_locations.1.contains(&u[j]) {
                    return true;
                }
                if j > i && relative_locations.0.contains(&u[j]) {
                    return true;
                }
            }
        }
        false
    }).map(|u| {
        let mut sorted: Vec<usize> = u.to_vec();
        sorted.sort_by(|u1, u2| {
            let relative_locations_u1: (Vec<usize>, Vec<usize>) = data.relative_location_map[u1].clone();
            let relative_locations_u2: (Vec<usize>, Vec<usize>) = data.relative_location_map[u2].clone();
            if relative_locations_u1.0.contains(u2) || relative_locations_u2.1.contains(u1) {
                cmp::Ordering::Less
            } else { cmp::Ordering::Greater }
        });
        sorted[sorted.len() / 2]
    }).sum();
    println!("{:?}", total)
}

fn problem1(data: &Data) {
    let total: usize = data.updates
        .iter().filter(|u| {
        for i in 0..u.len() {
            let n: usize = u[i];
            let relative_locations: (Vec<usize>, Vec<usize>) = data.relative_location_map[&n].clone();

            for j in 0..u.len() {
                if j == i {
                    continue;
                }

                if j < i && relative_locations.1.contains(&u[j]) {
                    return false;
                }
                if j > i && relative_locations.0.contains(&u[j]) {
                    return false;
                }
            }
        }
        true
    }).map(|u| {
       u[u.len() / 2]
    }).sum();
    println!("{:?}", total)
}

fn get_map(data: String) -> Data {
    let mut relative_location_map: HashMap<usize, (Vec<usize>, Vec<usize>)> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = vec![];

    data.lines().for_each(|l| {
        if l.contains("|"){
            let parts: Vec<usize> = l.split("|")
                .map(|p| p.parse::<usize>().expect("Failed to parse")).collect();

            if !relative_location_map.contains_key(&parts[0]) {
                relative_location_map.insert(parts[0], (vec![], vec![]));
            }

            if !relative_location_map.contains_key(&parts[1]) {
                relative_location_map.insert(parts[1], (vec![], vec![]));
            }

            let new_left_0: Vec<usize> = relative_location_map[&parts[0]].clone().0;
            let mut new_right_0: Vec<usize> = relative_location_map[&parts[0]].clone().1.to_vec();
            let mut new_left_1: Vec<usize> = relative_location_map[&parts[1]].clone().0.to_vec();
            let new_right_1: Vec<usize> = relative_location_map[&parts[1]].clone().1;

            new_right_0.push(parts[1]);
            new_left_1.push(parts[0]);

            relative_location_map.insert(parts[0], (new_left_0, new_right_0));
            relative_location_map.insert(parts[1], (new_left_1, new_right_1));
        } else if l.contains(",") {
            updates.push(l.split(",").map(|n| n.parse::<usize>().expect("Failed to parse")).collect())
        }
    });
    Data{
        relative_location_map,
        updates
    }
}