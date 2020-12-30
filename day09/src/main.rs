use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
/**
 * Implements circular buffer with fast lookup for item existence
 * at the expense of memory usage.
 */
struct CircularBuffer {
    beg_index: usize,
    num_items: usize,
    buf: Vec<usize>,
    set: HashSet<usize>,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> CircularBuffer {
        return CircularBuffer {
            beg_index: 0,
            num_items: 0,
            buf: vec![0; capacity],
            set: HashSet::new(),
        };
    }

    /// Returns the capacity (i.e., number of items that can be stored) of
    /// the buffer.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// Returns the size (i.e., number of items) of the buffer.
    pub fn size(&self) -> usize {
        self.num_items
    }

    /// Inserts a new item into the circular buffer.
    pub fn insert(&mut self, num: usize) {
        let mut prev_item = 0;

        let mut ind = 0;
        if self.num_items < self.buf.len() {
            ind = (self.beg_index + self.num_items) % self.buf.len();
            self.num_items += 1;
        } else {
            ind = self.beg_index;
            self.beg_index = (self.beg_index + 1) % self.buf.len();
        }

        self.set.remove(&self.buf[ind]);
        self.buf[ind] = num;
        self.set.insert(self.buf[ind]);
    }

    /// Finds the pair of items within the circular buffer that sums up to |num|.
    pub fn lookup_twosum(&self, num: usize) -> Option<(usize, usize)> {
        for i in 0..self.num_items {
            let ind = (self.beg_index + i) % self.buf.len();

            if self.buf[ind] > num {
                continue;
            } else if self.set.contains(&(num - self.buf[ind])) {
                return Some((self.buf[ind], num - self.buf[ind]));
            }
        }

        None
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

    let mut buf = CircularBuffer::new(25);
    for data in BufReader::new(f).lines() {
        let num = data.unwrap()
            .parse::<usize>()
            .unwrap();
        if buf.size() < buf.capacity() {
            buf.insert(num);
            continue;
        } else if buf.lookup_twosum(num).is_none() {
            println!("Found: {}", num);
            break;
        }

        buf.insert(num);
    }
}
