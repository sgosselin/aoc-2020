use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn calc_character_set(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();

    for c in s.chars() {
        set.insert(c);
    }

    set
}

fn count_num_questions(path: &str) -> usize {
    let f = File::open(path)
        .expect("could not open the input file");

    let mut res = 0;
    let mut buf = String::new();

    for data in BufReader::new(f).lines() {
        let line = data
            .expect("could not read line from input");
        if line == "" {
            let set = calc_character_set(&buf);
            res += set.len();
            buf.clear();
        } else {
            buf.push_str(&line);
        }
    }

    return res;
}

fn update_frequency_map(s: &str, count: &mut HashMap<char, usize>) {
    for c in s.chars() {
        let it = count.entry(c).or_insert(0);
        *it += 1;
    }
}

fn count_num_questions_part2(path: &str) -> usize {
    let f = File::open(path)
        .expect("could not open the input file");

    let mut res = 0;
    let mut freqs:HashMap<char, usize> = HashMap::new();
    let mut num_groups = 0;

    for data in BufReader::new(f).lines() {
        let line = data
            .expect("could not read line from input");
        if line != "" {
            update_frequency_map(&line, &mut freqs);
            num_groups += 1;
        } else {
            // Count the number of answers that showed up in each group.
            for (_, num) in &freqs {
                if *num == num_groups {
                    res += 1
                }
            }
            // Reset the counters.
            freqs.clear();
            num_groups = 0;
        }
    }

    res
}
fn main() {
    let args:Vec<String> = env::args()
        .collect();
    if args.len() != 2 {
        println!("usage: {} [path]", args[0]);
        return;
    }

    let res_part1 = count_num_questions(&args[1]);
    println!("res (part 1): {}", res_part1);
    let res_part2 = count_num_questions_part2(&args[1]);
    println!("res (part 2): {}", res_part2);
}
