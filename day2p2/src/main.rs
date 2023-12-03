use std::fs;
use std::collections::HashMap;
use std::ops::Add;

fn main() {

    let mut min_sum: u32 = 0;

    for line in fs::read_to_string("./data.txt")
        .expect("Couldn't read file.")
        .lines()
    {
        let game_line: Vec<&str> = line.split(':').collect();
        min_sum = min_sum + tabulate_sets(game_line);
    }
    println!("{:?}", min_sum)
}

fn tabulate_sets(game_line: Vec<&str>) -> u32 {
    let mut min: HashMap<&str, u32> = HashMap::new();
    for set in game_line.last().unwrap().split(";") {

        for f in set.split(',') {
            let parts: Vec<&str> = f.trim().split(' ').collect();
            //println!("{:?}", f);
            let num: u32 = parts.first().unwrap().parse::<u32>().expect("Could not parse integer");
            let color: &str = parts.last().unwrap().to_owned();

            if !min.contains_key(color) {
                min.insert(color, num);
            } else if min.get(color).unwrap() < &num {
                min.insert(color, num);
            }
        }
    }
    return min.values().cloned().reduce(|a, b| a * b)
        .expect("Could not tablulate values for mins");
}

