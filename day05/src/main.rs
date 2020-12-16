use std::cmp::max;
use std::collections::HashSet;
use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
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

    pub fn from(row: usize, col: usize) -> Seat {
        return Seat {
            row: row,
            col: col,
        };
    }

    pub fn calc_id(&self) -> usize {
        8 * self.row + self.col
    }

    pub fn max_id() -> usize {
        8 * 127 + 7
    }
}

fn find_missing_id(set_id: &HashSet<usize>) -> Option<usize> {
    // The challenge mentions the first and last rows are not
    // available. As such, we pick a reasonable range for the
    // row and columns.
    let min_row = 10;
    let max_row = 100;
    let min_col = 0;
    let max_col = 7;

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            let id = Seat::from(row, col).calc_id();
            if set_id.get(&id).is_none() {
                return Some(id);
            }
        }
    }

    None
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
    let mut set_id = HashSet::new();

    for data in BufReader::new(f).lines() {
        if let Ok(line) = data {
            let st = Seat::from_str(&line);
            let id = st.calc_id();
            max_id = max(max_id, id);
            set_id.insert(id);
        }
    }

    println!("found {} ids", set_id.len());
    println!("max id: {}", max_id);
    println!("missing id: {:?}", find_missing_id(&set_id));
}
