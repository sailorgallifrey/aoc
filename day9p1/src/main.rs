use std::fs;

fn main() {
    let file = fs::read_to_string("./data.txt").expect("Couldn't read file.");
    let lines: Vec<&str> = file.lines().collect();

    let problem: Vec<Vec<i64>> = get_problem(lines);

    let result: i64 = get_result(problem);

    println!("{:?}", result);
}

fn get_result(series_list: Vec<Vec<i64>>) -> i64 {
    let mut result: i64 = 0;
    for series in series_list {
        let mut decompose: Vec<Vec<i64>> = vec![series];

        let mut decompose_index: usize = 0;
        loop {
            let mut next: Vec<i64> = vec![];

            for i in 0..decompose[decompose_index].len() - 1 {
                let diff = decompose[decompose_index][i + 1] - decompose[decompose_index][i];
                next.push(diff);
            }

            decompose.push(next.clone());
            decompose_index += 1;

            if next.clone().iter().all(|n| n == &0) {
                break;
            }
        }

        decompose.reverse();
        let mut next = 0;
        for i in 1..decompose.len() {
            next = decompose[i].last().unwrap() + next;
        }

        result += next as i64;
    }

    result
}

fn get_problem(lines: Vec<&str>) -> Vec<Vec<i64>> {
    lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect()
}
