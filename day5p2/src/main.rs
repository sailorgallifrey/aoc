use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("./data.txt").expect("Couldn't read file.");
    let lines: Vec<&str> = file.lines().collect();

    let seed_ranges: Vec<Vec<u64>> = get_seeds(lines.first().unwrap());
    let maps: HashMap<String, Vec<Vec<u64>>> = get_maps(lines);

    let result: u64 = get_result(seed_ranges, maps);

    println!("{:?}", result);
}

fn get_seeds(s: &str) -> Vec<Vec<u64>> {
    s.split(": ")
        .last()
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<u64>().expect("Unable to parse seed id"))
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|c| c.to_vec())
        .collect()
}

fn get_maps(lines: Vec<&str>) -> HashMap<String, Vec<Vec<u64>>> {
    let mut result: HashMap<String, Vec<Vec<u64>>> = HashMap::new();
    let mut map_name: String = String::from("");
    let mut map_vec: Vec<Vec<u64>> = vec![];
    for line in lines.iter().skip(1) {
        if line.contains("map:") {
            map_name = line.replace(" map:", "");
        } else if !map_name.is_empty() && line.is_empty() {
            result.insert(map_name.clone(), map_vec.clone());
            map_vec = vec![];
        } else if !line.trim().is_empty() {
            let nums: Vec<u64> = line
                .split(' ')
                .map(|n| n.parse::<u64>().expect("failed to parse map number"))
                .collect();

            map_vec.push(nums)
        }
    }

    result.insert(map_name.clone(), map_vec.clone());
    result
}

fn get_result(seed_ranges: Vec<Vec<u64>>, map: HashMap<String, Vec<Vec<u64>>>) -> u64 {
    let mut lowest_location: u64 = u64::MAX;

    let seed_map = &map["seed-to-soil"];
    let soil_map = &map["soil-to-fertilizer"];
    let fert_map = &map["fertilizer-to-water"];
    let water_map = &map["water-to-light"];
    let light_map = &map["light-to-temperature"];
    let temp_map = &map["temperature-to-humidity"];
    let humid_map = &map["humidity-to-location"];

    for seed_range in seed_ranges {
        println!("{:?}", seed_range);
        for s in 0..seed_range[1] {
            let seed = seed_range[0] + s;
            let soil: u64 = get_value(&seed, &seed_map);
            let fertilizer: u64 = get_value(&soil, soil_map);
            let water: u64 = get_value(&fertilizer, fert_map);
            let light: u64 = get_value(&water, water_map);
            let temp: u64 = get_value(&light, light_map);
            let humid: u64 = get_value(&temp, temp_map);
            let location: u64 = get_value(&humid, humid_map);

            if lowest_location > location {
                lowest_location = location
            }
        }
    }

    lowest_location
}

fn get_value(lookup: &u64, map: &Vec<Vec<u64>>) -> u64 {
    for m in map {
        let last = m[1] + m[2];
        if *lookup >= m[1] && *lookup <= last {
            return m[0] + (*lookup - m[1])
        }
    }
    lookup.to_owned()
}

