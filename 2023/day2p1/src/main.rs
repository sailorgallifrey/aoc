use std::fs;
use std::collections::HashMap;
use std::ops::Add;

fn main() {

    let available: HashMap<&str,u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut game_id_sum: u32 = 0;

    for line in fs::read_to_string("./data.txt")
        .expect("Couldn't read file.")
        .lines()
    {
        let game_line: Vec<&str> = line.split(':').collect();
        let game_id: String = game_line.first().unwrap().replace("Game ", "");

        if check_sets(&available, game_line) {
            game_id_sum = game_id_sum.add(game_id.parse::<u32>().expect("Could not parse game id as integer"));
        }
    }
    println!("{:?}", game_id_sum)
}

fn check_sets(available: &HashMap<&str, u32>, game_line: Vec<&str>) -> bool {
    for set in game_line.last().unwrap().split(";") {
        let mut found: HashMap<&str, u32> = HashMap::new();

        for f in set.split(',') {
            let parts: Vec<&str> = f.trim().split(' ').collect();
            //println!("{:?}", f);
            let num: u32 = parts.first().unwrap().parse::<u32>().expect("Could not parse integer");
            let color: &str = parts.last().unwrap().to_owned();

            if !found.contains_key(color) {
                found.insert(color, num);
            } else {
                found.insert(color, num + found[color]);
            }
        }

        if !check_found(&available, &mut found) { return false; }
    }
    return true;
}

fn check_found(available: &HashMap<&str, u32>, found: &mut HashMap<&str, u32>) -> bool {
    for (c, n) in found {
        let check_val: Option<&u32> = available.get(c);

        if check_val.is_none() || check_val.unwrap() < &n {
            return false;
        }
    }
    return true;
}
