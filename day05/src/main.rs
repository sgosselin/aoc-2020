use std::cmp::max;
use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    pub fn from_str(s: &str) -> Seat {
        let mut row_beg = 0;
        let mut row_end = 127;
        for i in 0..7 {
            match s.chars().nth(i) {
                Some('B') => row_beg = row_beg + (row_end - row_beg) / 2 + 1,
                Some('F') => row_end = row_end - (row_end - row_beg) / 2 - 1,
                _ => {},
            }
        }
        assert_eq!(row_beg, row_end);

        let mut col_beg = 0;
        let mut col_end = 7;
        for i in 7..11 {
            match s.chars().nth(i) {
                Some('R') => col_beg = col_beg + (col_end - col_beg) / 2 + 1,
                Some('L') => col_end = col_end - (col_end - col_beg) / 2 - 1,
                _ => {},
            }
        }
        assert_eq!(col_beg, col_end);

        return Seat {
            row: row_beg,
            col: col_beg,
        };
    }

    pub fn calc_id(&self) -> usize {
        8 * self.row + self.col
    }
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

    let mut max_id = 0;

    for data in BufReader::new(f).lines() {
        if let Ok(line) = data {
            max_id = max(max_id, Seat::from_str(&line).calc_id());
        }
    }

    println!("max id: {}", max_id);
}
