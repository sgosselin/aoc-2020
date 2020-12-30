use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct BagIndexer {
    bag_container_re: Regex,
    bag_contained_re: Regex,
    contained_by: HashMap<String, Vec<String>>,
    container_of: HashMap<String, Vec<(String, usize)>>,
}

impl BagIndexer {
    pub fn new() -> BagIndexer {
        return BagIndexer {
            bag_container_re: Regex::new(r"(\w+ \w+) bags contain ").unwrap(),
            bag_contained_re: Regex::new(r"(\d) (\w+ \w+) bag").unwrap(),
            contained_by: HashMap::new(),
            container_of: HashMap::new(),
        };
    }

    /// Adds a rule to the indexer.
    pub fn add_rule(&mut self, rule: &str) {
        let cap_container = self.bag_container_re.captures(&rule)
            .expect("invalid rule: container bag misconstructed");
        let bag_container = &cap_container[1];

        for cap in self.bag_contained_re.captures_iter(&rule) {
            let num = &cap[1].parse::<usize>().unwrap();
            let bag = &cap[2];

            if let Some(v) = self.contained_by.get_mut(bag) {
                v.push(bag_container.to_string());
            } else {
                self.contained_by.insert(
                    bag.to_string(), vec!(bag_container.to_string()));
            }

            if let Some(v) = self.container_of.get_mut(bag_container) {
                v.push((bag.to_string(), *num));
            } else {
                self.container_of.insert(
                    bag_container.to_string(), vec!((bag.to_string(), *num)));

            }
        }
    }

    /// Returns the number of individual bags required inside |of|.
    pub fn count_num_bags_inside_of(&self, of: &str) -> usize {
        return self.count_num_bags_inside_of_helper(of) - 1;
    }

    /// Returns the number of individual bags that can eventually
    /// contains a |of| bag.
    pub fn count_num_bags_container_of(&self, of: &str) -> usize {
        let mut set:HashSet<String> = HashSet::new();
        self.count_num_bags_container_of_helper(&of.to_string(), &mut set);
        // The set also includes the requested bag, so we should exclude
        // it from the result.
        return set.len() - 1;
    }

    fn count_num_bags_inside_of_helper(&self, of: &str) -> usize {
        let mut count = 1;

        if let Some(bags) = self.container_of.get(of) {
            for bag in bags {
                count += bag.1 * self.count_num_bags_inside_of_helper(&bag.0);
            }
        }

        return count;
    }

    fn count_num_bags_container_of_helper(&self, of: &String, res_set: &mut HashSet<String>) {
        if res_set.contains(of) {
            return;
        }

        res_set.insert(of.to_string());

        if let Some(bags) = self.contained_by.get(of) {
            for bag in bags {
                self.count_num_bags_container_of_helper(&bag, res_set);
            }
        }
    }
}

fn main() {
    let args:Vec<String> = env::args()
        .collect();
    if args.len() != 2 {
        println!("usage: {} [input_path]", args[0]);
        return;
    }


    let f = File::open(&args[1])
        .expect("could not open the input file");

    let mut bag_indexer = BagIndexer::new();
    for data in BufReader::new(f).lines() {
        let rule = data.unwrap();
        bag_indexer.add_rule(&rule);
    }

    let res1 = bag_indexer.count_num_bags_container_of("shiny gold");
    println!("res part 1: {}", res1);

    let res2 = bag_indexer.count_num_bags_inside_of("shiny gold");
    println!("res part 2: {}", res2);
}

