use std::collections::HashSet;
use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} [path]", args[0]);
        return;
    }

    let file = File::open(args[1].to_string()).unwrap();

    let mut set: HashSet<i32> = HashSet::new();
    for line in BufReader::new(file).lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        set.insert(num);
    }

    println!("res1: {}", solve1(2020, &set).unwrap());
    println!("res2: {}", solve2(2020, &set).unwrap());

}

fn solve1(target: i32, set: &HashSet<i32>) -> Option<i32> {
    for x in set.iter() {
        if let Some(res) = set.get(&(target - x)) {
            return Some(x * res);
        }
    }

    None
}

fn solve2(target: i32, set: &HashSet<i32>) -> Option<i32> {
    for x in set.iter() {
        for y in set.iter() {
            // There are no negative numbers, no need to look for
            // a solution if we are already above the target.
            if (x + y) > target {
                continue;
            }

            if let Some(res) = set.get(&(target - x - y)) {
                return Some(x * y * res);
            }
        }
    }

    None
}
