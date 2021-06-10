#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut, unused_variables))]
#![allow(dead_code)]

use std::fmt;

// 11us to solve the 10x10 grid
fn def_2_29() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let rows = vec![
        vec![2,4,2],
        vec![6],
        vec![5,4],
        vec![2,2],
        vec![1,1,1,1],

        vec![1,1],
        vec![1,4,1],
        vec![1,2,1],
        vec![1,1],
        vec![1,6,1],
    ];

    let cols = vec![
        vec![1,1,2,1],
        vec![1,4,1],
        vec![3,1],
        vec![3,1,1,1],
        vec![3,2,1],

        vec![2,2,1],
        vec![3,1,1,1],
        vec![3,1],
        vec![1,4,1],
        vec![1,1,2,1],
    ];

    (rows, cols)
}

// 96ms
fn def_4_1() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let rows = vec![
        vec![4],
        vec![5],
        vec![2,3],
        vec![3,3,1],
        vec![3,3,3],

        vec![3,3,3],
        vec![3,3,2],
        vec![3,6],
        vec![3,4],
        vec![6],

        vec![1,5],
        vec![3,6],
        vec![5,3],
        vec![3,3],
        vec![2],
    ];

    let cols = rows.clone();
    (rows, cols)
}

// 170ms to solve this 40x20
fn def_5_392() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let rows = vec![
        vec![7,3],
        vec![3,2,5,2],
        vec![2,2,2,1,5],
        vec![1,2,6,7],
        vec![3,1,3,6,6],

        vec![1,2,1,1,5,2,4],
        vec![2,1,2,4,6,4,4],
        vec![3,4,2,1,6,3],
        vec![1,5,2,2,4,3],
        vec![2,1,4,2,1,5,4,2],

        vec![6,1,4,3,6,4,2],
        vec![8,2,3,4,7,3,2],
        vec![8,3,4,7,3,1],
        vec![9,1,2,5,7,3],
        vec![9,1,3,4,7,2,2],

        vec![11,1,2,5,7,1,2],
        vec![5,2,1,2,2,5,5,1,2],
        vec![2,1,1,1,2,5,3,2],
        vec![2,2,3,2,2,2],
        vec![3,4,2],
    ];

    let cols = vec![
        vec![1],
        vec![2],
        vec![2],
        vec![1,3],
        vec![2,4],

        vec![2,4],
        vec![7],
        vec![2,7,1],
        vec![1,1,8],
        vec![1,7],

        vec![2,5],
        vec![2,3],
        vec![1,3],
        vec![2,7,5],
        vec![2,4,1,2],

        vec![1,5,2,1],
        vec![2,8,1,2],
        vec![1,10,2],
        vec![1,6,3],
        vec![1,2,3,3,2],

        vec![1,3,1,7,1],
        vec![1,1,1,1,8],
        vec![2,1,3,8],
        vec![6,2,6],
        vec![7,3,4],

        vec![1,4,6,3],
        vec![2,4,8,1],
        vec![2,2,1,9],
        vec![3,1,1,9],
        vec![1,1,3,9],

        vec![2,4,8],
        vec![4,5,4,1],
        vec![4,7,1,1],
        vec![4,8,1],
        vec![5,6,1],

        vec![6,1,2,2],
        vec![9,1,2],
        vec![6,4,1],
        vec![2,2],
        vec![1],
    ];

    (rows, cols)
}


fn main() {
    let (rows, cols) = def_4_1();
    let grid = solve(&rows, &cols);
    grid.print();
}

struct Lane {
    holes: u128,
    fills: u128,
    len: usize,
}

impl Lane {
    fn full(len: usize) -> Self {
        Self { holes: u128::MAX, fills: u128::MAX, len }
    }

    fn empty(len: usize) -> Self {
        Self { holes: 0, fills: 0, len }
    }

    fn from_str(s: &str) -> Self {
        let mut self_ = Self::empty(s.len());
        for (i,c) in s.char_indices() {
            match c {
                '.' => self_.set_hole(i),
                '#' => self_.set_fill(i),
                ' ' => {},
                _ => panic!("unknown char: {}", c),
            }
        }
        self_
    }

    fn set_hole(&mut self, index: usize) { self.holes |= 1 << index; }
    fn set_fill(&mut self, index: usize) { self.fills |= 1 << index; }

    fn unset_hole(&mut self, index: usize) { self.holes &= !(1 << index); }
    fn unset_fill(&mut self, index: usize) { self.fills &= !(1 << index); }

    fn get_hole(&self, index: usize) -> bool { self.holes & (1 << index) != 0 }
    fn get_fill(&self, index: usize) -> bool { self.fills & (1 << index) != 0 }

    fn bits(index: usize, num: usize) -> u128 { ((1 << num)-1) << index }

    fn set_holes(&mut self, index: usize, num: usize) { self.holes |= Self::bits(index, num); }
    fn set_fills(&mut self, index: usize, num: usize) { self.fills |= Self::bits(index, num); }

    fn unset_holes(&mut self, index: usize, num: usize) { self.holes &= !Self::bits(index, num); }
    fn unset_fills(&mut self, index: usize, num: usize) { self.fills &= !Self::bits(index, num); }

    fn get_holes(&self, index: usize, num: usize) -> u128 { self.holes & Self::bits(index, num) }
    fn get_fills(&self, index: usize, num: usize) -> u128 { self.fills & Self::bits(index, num) }

    fn enumerate_pattern(&self, pattern: &Vec<usize>, iter_fn: impl FnMut(&Lane) -> ()) {
        let num_fulls: usize = pattern.iter().sum();
        let num_holes = self.len - num_fulls;
        let num_blocks = pattern.len();
        assert!(num_holes >= num_blocks-1, "pattern is too large");

        struct Context<'a, F: FnMut(&Lane) -> ()> {
            constraint: &'a Lane,
            current: Lane,
            pattern: &'a Vec<usize>,
            iter_fn: F,
        }
        let mut c = Context {
            constraint: &self,
            current: Lane::empty(self.len),
            pattern,
            iter_fn,
        };

        inner(&mut c, 0, num_blocks, num_holes);
        fn inner<F: FnMut(&Lane) -> ()>(c: &mut Context<F>, i: usize, num_blocks: usize, num_holes: usize) {
            if i == c.current.len {
                (c.iter_fn)(&c.current);
                return;
            }

            if num_holes > 0 && !c.constraint.get_fill(i) {
                c.current.set_hole(i);
                c.current.unset_fill(i);
                inner(c, i+1, num_blocks, num_holes-1);
            }

            /* current.get_hole() to make sure that two block don't touch each other  */ 
            if num_blocks > 0 && (i == 0 || c.current.get_hole(i-1)) {
                let pattern_index = c.pattern.len() - num_blocks;
                let block_len = c.pattern[pattern_index];
                if c.constraint.get_holes(i, block_len) == 0 {
                    c.current.unset_holes(i, block_len);
                    c.current.set_fills(i, block_len);
                    inner(c, i+block_len, num_blocks-1, num_holes);
                }
            }
        }
    }

    fn apply_pattern(&mut self, pattern: &Vec<usize>) -> bool {
        let mut result = Lane::full(self.len);
        let mut has_solution = false;

        self.enumerate_pattern(pattern, |lane| {
            result.holes &= !lane.fills;
            result.fills &= !lane.holes;
            has_solution |= true;
        });

        assert!(has_solution, "no solution found");
        let change = *self != result;
        *self = result;
        change
    }
}

impl fmt::Display for Lane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.len {
            let c = match (self.get_hole(i), self.get_fill(i)) {
                (false, false) => ' ',
                (true, false) => '.',
                (false, true) => '#',
                (true, true) => '?',
            };
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl fmt::Debug for Lane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialEq for Lane {
    fn eq(&self, other: &Self) -> bool {
        let mask = Self::bits(0, self.len);
        self.len == other.len &&
        self.fills & mask == other.fills & mask &&
        self.holes & mask == other.holes & mask
    }
}
impl Eq for Lane {}


#[test]
fn test_pattern1() {
    let mut lane = Lane::empty(10);

    lane.set_fill(4);
    println!("init : {}", lane);
    lane.enumerate_pattern(&vec![4,2], |lane| {
        println!("lane : {}", lane);
    });
    let c = lane.apply_pattern(&vec![4,2]);
    assert_eq!(c, true);
    let c = lane.apply_pattern(&vec![4,2]);
    assert_eq!(c, false);
    println!("final: {}", lane);
    assert_eq!(lane, Lane::from_str(".  ##     "));

    lane.set_hole(6);
    println!("spec : {}", lane);
    lane.enumerate_pattern(&vec![4,2], |lane| {
        println!("lane : {}", lane);
    });
    let c = lane.apply_pattern(&vec![4,2]);
    assert_eq!(c, true);
    println!("final: {}", lane);
    assert_eq!(lane, Lane::from_str(". ### . # "));
}



struct Grid {
    lanes: Vec<Lane>,
    rotated: bool,
}

impl Grid {
    fn new(nrows: usize, ncols: usize) -> Self {
        let lanes = (0..nrows).map(|_| Lane::empty(ncols)).collect();
        Self { lanes, rotated: false }
    }

    fn from_str(strs: &[&str]) -> Self {
        let lanes = strs.iter().cloned().map(Lane::from_str).collect();
        Self { lanes, rotated: false }
    }

    fn rotate(&self) -> Self {
        let lanes = (0..self.lanes[0].len).map(|i| {
            let mut lane = Lane::empty(self.lanes.len());
            for j in 0..self.lanes.len() {
                if self.lanes[j].get_fill(i) { lane.set_fill(j); }
                if self.lanes[j].get_hole(i) { lane.set_hole(j); }
            }
            lane
        }).collect();
        Self { lanes, rotated: !self.rotated }
    }

    pub fn print(&self) {
        if self.rotated {
            self.rotate().print();
        } else {
            for lane in &self.lanes {
                for _ in 0..2 {
                    for i in 0..lane.len {
                        if lane.get_fill(i) {
                            print!("███");
                        } else {
                            print!("   ");
                        }
                    }
                    println!("");
                }
            }
        }
    }
}

fn solve(
    rows: &Vec<Vec<usize>>,
    cols: &Vec<Vec<usize>>,
) -> Grid {
    let nrows = rows.len();
    let ncols = cols.len();

    let mut grid = Grid::new(nrows, ncols);

    loop {
        let mut progress = false;

        for (pattern, lane) in rows.iter().zip(&mut grid.lanes) {
            progress |= lane.apply_pattern(pattern);
        }

        grid = grid.rotate();
        for (pattern, lane) in cols.iter().zip(&mut grid.lanes) {
            progress |= lane.apply_pattern(pattern);
        }
        grid = grid.rotate();

        if !progress { break; }
    }

    grid
}

#[test]
fn test_1_8() {
    let rows = vec![
        vec![1],
        vec![3],
        vec![1,1],
        vec![5],
        vec![1,1,1],
    ];

    let cols = vec![
        vec![2],
        vec![3],
        vec![2,2],
        vec![3],
        vec![2],
    ];

    let grid = solve(&rows, &cols);
    grid.print();
    assert_eq!(grid.lanes, Grid::from_str(&vec![
        "..#..",
        ".###.",
        ".#.#.",
        "#####",
        "#.#.#",
    ]).lanes);
}
