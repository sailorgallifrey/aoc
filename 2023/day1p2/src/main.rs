use std::fs;

fn main() {
    let mut total: u32 = 0;
    for line in fs::read_to_string("./data.txt")
        .expect("Couldn't read file.")
        .lines()
    {
        let mut found_nums: Vec<char> = vec![];
        let len: usize = line.len();

        for index in 0..len {
            let found_num: (char, bool) = get_number(&line[index..len]);
            if found_num.1 {
                found_nums.push(found_num.0);
            }
        }

        if found_nums.is_empty() {
            println!("{:?}", line);
            continue;
        }
        let num_str = String::from_iter(vec![
            found_nums.first().unwrap(),
            found_nums.last().unwrap(),
        ]);
        let num = num_str.parse::<u32>().unwrap();

        total = total + num;
    }

    println!("{:?}", total);
}

fn get_number(s: &str) -> (char, bool) {
    let num_prefixes = std::collections::HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let first_char = s.chars().nth(0).unwrap();

    if first_char.is_digit(10) {
        return (first_char, true);
    }

    for (p, n) in num_prefixes {
        if s.starts_with(p) {
            return (n, true);
        }
    }

    ('n', false)
}
