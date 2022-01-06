use std::vec;

use crate::utils::read_input;

pub fn run() {
    let contents = read_input("four");

    part_one(&contents);
    // part_two(&contents);
}

fn part_one(input: &str) {
    let mut lines = input.lines();

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut count = 0;
    let boards: Vec<Vec<u32>> = lines
        .filter(|&l| !l.is_empty())
        .fold(vec![vec![]], |mut a, l| {
            let parsed_line: Vec<u32> = l.split_whitespace().map(|c| c.parse().unwrap()).collect();

            if a[count].len() == 5 {
                count += 1;
                a.push(vec![parsed_line]);
            } else {
                a[count].push(parsed_line);
            }
            a
        })
        .into_iter()
        .map(|b| b.into_iter().flatten().collect::<Vec<u32>>())
        .collect();

    'outer: for i in 0..numbers.len() {
        for b in &boards {
            if check_bingo(&b, &numbers[0..i]) {
                println!("board:{:?} \n numbers:{:?}", &b, &numbers[0..i]);
                let points = calculate_points(&b, &numbers[0..i]);
                println!("points:{:?}", points);
                break 'outer;
            }
        }
    }
}

fn part_two(input: &str) {}

fn check_bingo(board: &Vec<u32>, numbers: &[u32]) -> bool {
    if numbers.len() < 5 {
        return false;
    }

    for i in 0..5 {
        let row = &board[5 * i..5 * i + 5];
        if row.into_iter().all(|n| numbers.contains(n)) {
            return true;
        }
    }

    for i in 0..5 {
        let col = vec![
            board[i],
            board[i + 5],
            board[i + 10],
            board[i + 15],
            board[i + 20],
        ];
        if col.into_iter().all(|n| numbers.contains(&n)) {
            return true;
        };
    }

    false
}

fn calculate_points(board: &Vec<u32>, numbers: &[u32]) -> u32 {
    board
        .into_iter()
        .filter(|&v| !numbers.contains(v))
        .sum::<u32>()
        * numbers.last().unwrap()
}
