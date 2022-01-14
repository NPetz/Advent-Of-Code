use crate::utils::read_input;

pub fn run() {
    println!("\nDay 7");

    let contents = read_input("seven");

    println!("\nPart I");

    part_one(&contents);

    println!("\nPart II");

    part_two(&contents);
}

fn part_one(input: &str) {
    let mut hs: Vec<i32> = input.split(',').map(|c| c.parse().unwrap()).collect();
    hs.sort_unstable();

    let mut count = 0;
    let mut lc = 1;
    let mut rc = 1;
    let mut l = 0;
    let mut r = hs.len() - 1;

    while l != r {
        while hs[l] == hs[l + 1] {
            lc += 1;
            l += 1;
        }

        while hs[r] == hs[r - 1] {
            rc += 1;
            r -= 1;
        }

        let ll = hs[l];
        let lr = hs[r];

        if lc <= rc {
            let nl = hs[l + 1];
            count += (nl - ll) * lc;
            lc += 1;
            l += 1;
        } else {
            let nr = hs[r - 1];
            count += (lr - nr) * rc;
            rc += 1;
            r -= 1;
        }
    }
    println!("{count}");
}

fn part_two(input: &str) {
    let mut hs: Vec<i32> = input.split(',').map(|c| c.parse().unwrap()).collect();
    hs.sort_unstable();

    // let mut lc = 1;
    // let mut rc = 1;
    // let mut l = 0;
    // let mut r = hs.len() - 1;

    // while l != r {
    //     while (hs[l] == hs[l + 1]) {
    //         lc += 1;
    //         l += 1;
    //     }

    //     while (hs[r] == hs[r - 1]) {
    //         rc += 1;
    //         r -= 1;
    //     }
    // }
}

// fn sum_first_x(x: i32) -> i32 {
//     x * (x + 1) / 2
// }
