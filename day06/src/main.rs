use std::collections::HashSet;
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
            res += .len();
            buf.clear();
        } else {
            buf.push_str(&line);
        }
    }

    return res;
}

fn main() {
    let args:Vec<String> = env::args()
        .collect();
    if args.len() != 2 {
        println!("usage: {} [path]", args[0]);
        return;
    }

    let res = count_num_questions(&args[1]);
    println!("res: {}", res);
}
