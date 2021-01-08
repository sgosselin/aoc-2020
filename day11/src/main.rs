use std::env;
use std::io::prelude::*;
use std::fs::File;

#[derive(PartialEq, Copy, Clone)]
enum Item {
    Empty,
    Floor,
    Occupied,
}

impl Item {
    pub fn from_char(c: char) -> Option<Item> {
        match c {
            'L' =>
                Some(Item::Empty),
            '.' =>
                Some(Item::Floor),
            '#' =>
                Some(Item::Occupied),
            _ =>
                None,
        }
    }
}

#[derive(Clone)]
struct Grid {
    w: usize,
    h: usize,
    data: Vec<Item>,
}

impl Grid {
    pub fn from(s: &String) -> Result<Grid, &str> {
        let h = s.matches('\n').count();
        if h == 0 {
            return Err("grid height must be > 0");
        }

        let w = (s.chars().count() / h) - 1;
        if w == 0 {
            return Err("grid width must be > 0");
        }

        let mut data:Vec<Item> = vec![Item::Floor; w * h];
        let mut data_ind = 0;

        for c in s.chars() {
            if let Some(item) = Item::from_char(c) {
                data[data_ind] = item;
                data_ind += 1;
            }
        }

        return Ok(Grid {
            w: w,
            h: h,
            data: data,
        });
    }

    pub fn get_num_occupied(&self) -> usize {
        self.data.iter().filter(|x| *x == &Item::Occupied).count()
    }

    pub fn is_equal(&self, grid: &Grid) -> bool {
        assert_eq!(self.w, grid.w);
        assert_eq!(self.h, grid.h);

        for i in 0..self.data.len() {
            if self.data[i] != grid.data[i] {
                return false;
            }
        }

        return true;
    }

    pub fn print(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                match self.data[y * self.w + x] {
                    Item::Empty =>
                        print!("L"),
                    Item::Floor =>
                        print!("."),
                    Item::Occupied =>
                        print!("#"),
                }
            }
            print!("\n");
        }
    }

    pub fn run_simulation_p1(&self, next_grid: &mut Grid) {
        assert_eq!(self.w, next_grid.w);
        assert_eq!(self.h, next_grid.h);

        let dirs = vec![
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];

        for y in 0..self.h {
            for x in 0..self.w {
                let ind = y * self.w + x;

                let item = self.data[ind];
                if item == Item::Floor {
                    next_grid.data[ind] = item;
                    continue;
                }

                let mut num_occupied = 0;
                for dir in dirs.iter() {
                    let nx = (x as i32) + dir.0;
                    let ny = (y as i32) + dir.1;
                    if nx < 0 || nx >= (self.w as i32) || ny < 0 || ny >= (self.h as i32) {
                        continue;
                    }

                    let nx_usize = nx as usize;
                    let ny_usize = ny as usize;
                    if self.data[ny_usize * self.w + nx_usize] == Item::Occupied {
                        num_occupied += 1;
                    }
                }

                next_grid.data[ind] = if item == Item::Empty && num_occupied == 0 {
                    Item::Occupied
                } else if item == Item::Occupied && num_occupied >= 4 {
                    Item::Empty
                } else {
                    item
                };
            }
        }
    }
}

fn do_part1(origin: &Grid) -> usize {
    let mut g0 = origin.clone();
    let mut g1 = origin.clone();

    let mut round = 0;
    loop {
        let num_occupied = if (round % 2) == 0 {
            g0.run_simulation_p1(&mut g1);
            g1.get_num_occupied()
        } else {
            g1.run_simulation_p1(&mut g0);
            g0.get_num_occupied()
        };

        if g0.is_equal(&g1) {
            return num_occupied;
        }

        round += 1;
    }
}

fn main() {
    let args:Vec<String> = env::args()
        .collect();
    if args.len() != 2 {
        println!("usage: {} [path]", args[0]);
        return;
    }

    let mut file = File::open(&args[1])
        .expect("could not open input file");

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("could not read input file");

    let input_grid = Grid::from(&buf)
        .expect("could not create grid");

    let res_part1 = do_part1(&input_grid.clone());
    println!("res_part1={}", res_part1);
}

