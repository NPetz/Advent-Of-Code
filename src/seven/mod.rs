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
    let mut hs: Vec<i32> = input.split(",").map(|c| c.parse().unwrap()).collect();
    hs.sort();

    let mut count = 0;
    let mut ll = 0;
    let mut lr = 0;
    let mut lc = 1;
    let mut rc = 1;
    let mut l = 0;
    let mut r = hs.len() - 1;

    while l != r {
        while (hs[l] == hs[l + 1]) {
            lc += 1;
            l += 1;
        }

        while (hs[r] == hs[r - 1]) {
            rc += 1;
            r -= 1;
        }

        ll = hs[l];
        lr = hs[r];

        if (lc <= rc) {
            let nl = hs[l + 1];
            count += (nl - ll) * lc;
            lc += 1;
            l += 1;
            ll = nl;
        } else {
            let nr = hs[r - 1];
            count += (lr - nr) * rc;
            rc += 1;
            r -= 1;
            lr = nr;
        }
    }

    println!("{count}");
}

fn part_two(input: &str) {}
