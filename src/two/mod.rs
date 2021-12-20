use crate::utils::read_input;

pub fn run() {
    let contents = read_input("two");

    part_one(&contents);
    part_two(&contents);
}

fn part_one(input: &str) {
    let mut x = 0;
    let mut y = 0;

    for l in input.lines() {
        let mut iter = l.split_whitespace();
        let op = iter.next().unwrap();
        let v = iter.next().unwrap().parse::<u32>().unwrap();

        match op {
            "forward" => x += v,
            "up" => y -= v,
            "down" => y += v,
            _ => (),
        }
    }

    println!("{}", x * y)
}

fn part_two(input: &str) {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for l in input.lines() {
        let mut iter = l.split_whitespace();
        let op = iter.next().unwrap();
        let v = iter.next().unwrap().parse::<u32>().unwrap();

        match op {
            "forward" => {
                x += v;
                y += aim * v;
            }
            "up" => aim -= v,
            "down" => aim += v,
            _ => (),
        }
    }

    println!("{}", x * y)
}
