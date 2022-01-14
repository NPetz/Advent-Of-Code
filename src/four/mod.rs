use std::vec;

use crate::utils::read_input;

pub fn run() {
    println!("\nDay 4");

    let contents = read_input("four");

    println!("\nPart I");

    part_one(&contents);

    println!("\nPart II");

    part_two(&contents);
}

fn part_one(input: &str) {
    let mut lines = input.lines();

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
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
            if check_bingo(b, &numbers[0..i]) {
                let points = calculate_points(b, &numbers[0..i]);
                println!("{:?}", points);
                break 'outer;
            }
        }
    }
}

fn part_two(input: &str) {
    let mut lines = input.lines();

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut count = 0;
    let mut boards: Vec<Vec<u32>> = lines
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

    let mut last_board: Vec<u32> = vec![];
    let mut last_index: usize = 0;

    for i in 0..numbers.len() {
        boards = boards
            .into_iter()
            .filter(|b| !check_bingo(b, &numbers[0..i]))
            .collect();

        if boards.len() == 1 {
            last_board = boards[0].clone();
        }
        if boards.is_empty() {
            last_index = i;
            break;
        }
    }

    let points = calculate_points(&last_board, &numbers[0..last_index]);
    println!("{:?}", points);
}

fn check_bingo(board: &[u32], numbers: &[u32]) -> bool {
    if numbers.len() < 5 {
        return false;
    }

    for i in 0..5 {
        let row = &board[5 * i..5 * i + 5];
        if row.iter().all(|n| numbers.contains(n)) {
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

fn calculate_points(board: &[u32], numbers: &[u32]) -> u32 {
    board.iter().filter(|&v| !numbers.contains(v)).sum::<u32>() * numbers.last().unwrap()
}
