use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn calc_part1(arr: &[usize]) -> (usize, usize) {
    let mut n1 = 0;
    let mut n3 = 0;
    let mut prev = 0;

    for curr in arr {
        match (*curr - prev) {
            1 => n1 += 1,
            3 => n3 += 1,
            _ => {},
        }
        prev = *curr;
    }

    return (n1, n3);
}

/*
 * Let's define a recurrence.  F(n), total number of distinct ways we can
 * arrange the adapters (A0, ..., An).  The solution for F(n) can then be
 * calculated from its sub-problems.
 *
 *      F(n)
 *          = F(n - 1)      if (V(A_n) - V(A_n-1)) <= 3
 *          + F(n - 2)      if (V(A_n) - V(A_n-2)) <= 3
 *          + F(n - 3)      if (V(A_n) - V(A_n-3)) <= 3
 *
 * The solution of part 2 is then F(n), and can be calculated by applying
 * a bottom-up dynamic programming approach.
 */
fn calc_part2(arr: &[usize]) -> usize {
    let mut sol:Vec<usize> = vec![0; arr.len()];

    sol[0] = 1;
    for i in 1..arr.len() {
        let mut res = 0;

        for j in 1..=3 {
            if i >= j && (arr[i] - arr[i - j]) <= 3 {
                res += sol[i - j];
            }
        }

        sol[i] = res;
    }

    return sol[sol.len() - 1];
}

fn main() {
    let args:Vec<String> = env::args()
        .collect();
    if args.len() != 2 {
        println!("usage: {} [path]", args[0]);
        return;
    }

    let f = File::open(&args[1])
        .expect("could not open input file");
    let mut arr:Vec<usize> = BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect();
    // Sort the input and add the outlet and the device joltages.
    arr.sort();
    arr.insert(0, 0);
    arr.insert(arr.len(), arr.last().unwrap() + 3);

    let (n1, n3) = calc_part1(&arr);
    println!("answer part1: n1={}, n3={}, res={}", n1, n3, n1 * n3);

    let num_order = calc_part2(&arr);
    println!("answer part2: {}", num_order);
}
