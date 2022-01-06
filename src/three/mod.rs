use crate::utils::read_input;

pub fn run() {
    let contents = read_input("three");
    // part_one(&contents);
    part_two(&contents);
}

fn part_one(input: &str) {
    let mut input_length = 0;
    let mut gamma: Vec<u32> = vec![];
    let mut epsilon: Vec<u32> = vec![];

    let r = input.lines().fold(vec![0; 12], |mut a, v| {
        v.chars()
            .collect::<Vec<char>>()
            .iter()
            .enumerate()
            .for_each(|(i, c)| a[i] += c.to_digit(10).unwrap());
        input_length += 1;
        a
    });

    r.iter().for_each(|v| {
        if v > &(input_length / 2) {
            gamma.push(1);
            epsilon.push(0);
        } else {
            gamma.push(0);
            epsilon.push(1);
        }
    });
    let gamma = binary_vector_to_integer(gamma);
    let epsilon = binary_vector_to_integer(epsilon);

    println!("power consumption: {}", gamma * epsilon)
}

fn part_two(input: &str) {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let oxygen = binary_vector_to_integer(find_one_input(lines.clone(), 0, true));
    let co2 = binary_vector_to_integer(find_one_input(lines.clone(), 0, false));

    println!("{}", oxygen * co2);
}

fn binary_vector_to_integer(vec: Vec<u32>) -> u32 {
    let len = vec.len() - 1;
    vec.iter().enumerate().fold(0, |a, (i, v)| match v {
        0 => a + 0,
        1 => a + (1 * 2_u32.pow(len as u32 - i as u32)),
        _ => a,
    })
}

fn find_one_input(l: Vec<Vec<u32>>, i: usize, is_most: bool) -> Vec<u32> {
    let len: u32 = l.len().try_into().unwrap();

    if len == 1 || i > 11 {
        return l[0].clone();
    }

    let most_common = if l.iter().fold(0, |a, v| a + v[i]) * 2 >= len {
        1
    } else {
        0
    };
    let least_common = if most_common == 1 { 0 } else { 1 };
    let target = if is_most { most_common } else { least_common };

    let filtered_lines = l.into_iter().filter(|l| l[i] == target).collect();

    find_one_input(filtered_lines, i + 1, is_most)
}
