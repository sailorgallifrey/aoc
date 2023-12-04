use std::fs;

fn main() {
    let mut total: u32 = 0;
    for line in fs::read_to_string("./data.txt")
        .expect("Couldn't read file.")
        .lines()
    {
        let game_line: Vec<&str> = line.split(':').collect();
        let nums: Vec<&str> = game_line.last().unwrap().split('|').collect();

        let available: Vec<u32> = nums.last().unwrap().split_whitespace()
            .map(|s| s.parse::<u32>().expect("Failed to parse available number")).collect();
        let winning: Vec<u32> = nums.first().unwrap().split_whitespace()
            .map(|s| s.parse::<u32>().expect("Failed to parse card number"))
            .filter(|n| available.contains(n)).collect();


        let size: usize = winning.len();
        if size > 0 {
            let score = 2_u32.pow((size - 1) as u32);
            total += score
        }
    }

    println!("{:?}", total);
}
