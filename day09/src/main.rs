use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/**
 * Implements circular buffer with fast lookup for item existence
 * at the expense of memory usage.
 */
#[derive(Debug)]
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

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn size(&self) -> usize {
        self.num_items
    }

    pub fn insert(&mut self, num: usize) {
        let ind:usize;
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

fn find_breaking_num(arr: &[usize]) -> Option<usize> {
    let mut buf = CircularBuffer::new(25);

    for num in arr {
        if buf.size() == buf.capacity() && buf.lookup_twosum(*num).is_none() {
            return Some(*num);
        }

        buf.insert(*num);
    }

    None
}

fn find_subarray_sums_to(arr: &[usize], to: usize) -> Option<(usize, usize)> {
    for i in 0..arr.len() {
        let mut sum = arr[i];
        for j in (i + 1)..arr.len() {
            sum += arr[j];
            if sum == to {
                return Some((i, j));
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

    let input_nums:Vec<usize> = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let breaking_num = find_breaking_num(&input_nums)
        .expect("could not find breaking number");

    let (i, j) = find_subarray_sums_to(&input_nums, breaking_num)
        .expect("could not find a subarray that sums to the breaking num");

    // Find the min/max.
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for i in &input_nums[i..j] {
        min = std::cmp::min(min, *i);
        max = std::cmp::max(max, *i);
    }

    println!("answer part 1: {}", breaking_num);
    println!("answer part 2: {}", min + max);
}
