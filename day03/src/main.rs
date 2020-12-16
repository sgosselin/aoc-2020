use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct Map {
    map: String,
    
    pub w: usize,
    pub h: usize,
}

impl Map {
    /// Create a Map from a file.
    pub fn from_file(path: String) -> Map {
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf);

        let count = buf.matches('\n').count();
        let h:usize = buf.matches('\n').count();
        let w:usize = buf.chars().count() / h;

        // Create the map and trim the end-of-line characters from the
        // buffer, these items are not part of the real map. The |w|
        // parameter also needs to be adjusted.
        buf.retain(|c| c != '\n');
        return Map {
            map: buf,
            w: w - 1,
            h: h,
        }
    }

    /// Access the (x, y)-th item.
    pub fn access(&self, x: usize, y: usize) -> char {
        let map_x = x % self.w;
        let map_y = y % self.h;

        return self.map.chars()
            .nth(map_y * self.w + map_x)
            .unwrap();
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} [path]", args[0]);
        return;
    }
   
    // Create the map from its file.
    let map = Map::from_file(args[1].to_string());

    let slopes = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut res:u64 = 1;

    for (slope_x, slope_y) in slopes.iter() {
        let mut x = 0;
        let mut y = 0;
        let mut num_trees = 0;

        while y < map.h {
            if map.access(x, y) == '#' {
                num_trees += 1;
            }
            x += slope_x;
            y += slope_y;
        }

        println!("slope({}, {}) => {}", slope_x, slope_y, num_trees);

        res *= num_trees;
    }

    println!("answer: {}", res);
}
