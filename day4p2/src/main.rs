use std::fs;

fn main() {
    let file = fs::read_to_string("./data.txt")
        .expect("Couldn't read file.");
    let lines: Vec<&str> = file.lines().collect();

    let mut total: u32 = 0;
    let mut count_copies: Vec<u32> = vec![];
    for i in 0..lines.len() {
        count_copies.push(1);
    }

    for i in 0..lines.len()
    {
        let current_copies = count_copies[i];

        let game_line: Vec<&str> = lines[i].split(':').collect();
        let nums: Vec<&str> = game_line.last().unwrap().split('|').collect();

        let available: Vec<u32> = nums
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Failed to parse available number"))
            .collect();
        let winning: Vec<u32> = nums
            .first()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Failed to parse card number"))
            .filter(|n| available.contains(n))
            .collect();

        let size: usize = winning.len();
        total += current_copies; // Increment the total for each copy of the current card

        for s in i + 1.. i + size + 1 {
            count_copies[s] += current_copies; // Increment the number of copies for each subsequent card up to the number of winners on this card.
        }
    }

    println!("{:?}", total);
}
