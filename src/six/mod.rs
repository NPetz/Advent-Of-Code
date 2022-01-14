use crate::utils::read_input;

pub fn run() {
    println!("\nDay 6");

    let contents = read_input("six");

    println!("\nPart I");

    part_one(&contents);

    println!("\nPart II");

    part_two(&contents);
}

fn part_one(input: &str) {
    // naive way by iterating every day

    let mut school: Vec<u32> = input.split(",").map(|s| s.parse().unwrap()).collect();

    for _ in 0..80 {
        let mut new_fish: Vec<u32> = vec![];

        for f in school.iter() {
            if *f == 0 {
                new_fish.push(8);
                new_fish.push(6)
            } else {
                new_fish.push(f - 1)
            }
        }
        school = new_fish;
    }

    println!("{}", school.len())
}

fn part_two(input: &str) {
    // there must be a way to recursively get the number for every fish, or a formula

    let school: Vec<u32> = input.split(",").map(|s| s.parse().unwrap()).collect();

    // naive implementaion takes too long, try with recursive
    // recursive also takes too long, so let's do memoization too

    // let's keep track of how many fish are produced over 256 depending on the starting value
    // there are only 0..=6 possible starting values, so we can just use an array to track them

    // its also gonna be a huge number, so lets use u128 to keep track of it

    let mut mem = [0; 7];

    let mut count: u128 = 0;
    for f in school {
        if mem[f as usize] != 0 {
            count += mem[f as usize];
        } else {
            let n = spawn(f as u128, 256);
            mem[f as usize] = n as u128;
            count += n as u128;
        }
    }

    println!("{}", count)
}

fn spawn(f: u128, d: u128) -> u128 {
    if d == 0 || d < f {
        return 1;
    }

    if f == 0 {
        return spawn(6, d - 1) + spawn(8, d - 1);
    }

    return spawn(0, d - f);
}
