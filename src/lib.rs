#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut, unused_variables))]
#![allow(dead_code)]

use std::fmt;
//use rayon::prelude::*;
pub mod problems;

struct Lane {
    holes: u128,
    fills: u128,
    len: u8,
}

impl Lane {
    fn full(len: usize) -> Self {
        assert!(len <= 127);
        let len = len as u8;
        let full = Self::bits(0, len);
        Self { holes: full, fills: full, len }
    }

    fn empty(len: usize) -> Self {
        assert!(len <= 127);
        let len = len as u8;
        Self { holes: 0, fills: 0, len }
    }

    fn from_str(s: &str) -> Self {
        let mut self_ = Self::empty(s.len());
        for (i,c) in s.char_indices() {
            let i = i as u8;
            match c {
                '.' => self_.set_hole(i),
                '#' => self_.set_fill(i),
                ' ' => {},
                _ => panic!("unknown char: {}", c),
            }
        }
        self_
    }

    fn set_hole(&mut self, index: u8) { self.holes |= 1 << index; }
    fn set_fill(&mut self, index: u8) { self.fills |= 1 << index; }

    fn unset_hole(&mut self, index: u8) { self.holes &= !(1 << index); }
    fn unset_fill(&mut self, index: u8) { self.fills &= !(1 << index); }

    fn get_hole(&self, index: u8) -> bool { self.holes & (1 << index) != 0 }
    fn get_fill(&self, index: u8) -> bool { self.fills & (1 << index) != 0 }

    fn bits(index: u8, num: u8) -> u128 { ((1 << num)-1) << index }

    fn set_holes(&mut self, index: u8, num: u8) { self.holes |= Self::bits(index, num); }
    fn set_fills(&mut self, index: u8, num: u8) { self.fills |= Self::bits(index, num); }

    fn unset_holes(&mut self, index: u8, num: u8) { self.holes &= !Self::bits(index, num); }
    fn unset_fills(&mut self, index: u8, num: u8) { self.fills &= !Self::bits(index, num); }

    fn get_holes(&self, index: u8, num: u8) -> u128 { self.holes & Self::bits(index, num) }
    fn get_fills(&self, index: u8, num: u8) -> u128 { self.fills & Self::bits(index, num) }

    fn enumerate_pattern(&self, pattern: &Vec<usize>, iter_fn: impl FnMut(&Lane) -> ()) {
        let num_fulls: u8 = pattern.iter().sum::<usize>() as u8;
        let num_holes = self.len - num_fulls;
        let num_blocks = pattern.len() as u8;
        assert!(num_holes >= num_blocks-1, "pattern is too large");

        struct Context<'a, F: FnMut(&Lane) -> ()> {
            constraint: &'a Lane,
            current: Lane,
            pattern: &'a Vec<usize>,
            iter_fn: F,
        }
        let mut c = Context {
            constraint: &self,
            current: Lane::empty(self.len as usize),
            pattern,
            iter_fn,
        };

        inner(&mut c, 0, num_blocks, num_holes);
        fn inner<F: FnMut(&Lane) -> ()>(c: &mut Context<F>, i: u8, num_blocks: u8, num_holes: u8) {
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
                let pattern_index = c.pattern.len() as u8 - num_blocks;
                let block_len = c.pattern[pattern_index as usize] as u8;
                if c.constraint.get_holes(i, block_len) == 0 {
                    c.current.unset_holes(i, block_len);
                    c.current.set_fills(i, block_len);
                    inner(c, i+block_len, num_blocks-1, num_holes);
                }
            }
        }
    }

    // returns which bits have changed
    fn apply_pattern(&mut self, pattern: &Vec<usize>) -> u128 {
        let mut result = Lane::full(self.len as usize);
        let mut has_solution = false;

        self.enumerate_pattern(pattern, |lane| {
            result.holes &= !lane.fills;
            result.fills &= !lane.holes;
            has_solution |= true;
        });

        assert!(has_solution, "no solution found");

        let bit_changes = (self.fills ^ result.fills) |
                          (self.holes ^ result.holes);
        *self = result;

        bit_changes
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
    assert!(c != 0);
    let c = lane.apply_pattern(&vec![4,2]);
    assert!(c == 0);
    println!("final: {}", lane);
    assert_eq!(lane, Lane::from_str(".  ##     "));

    lane.set_hole(6);
    println!("spec : {}", lane);
    lane.enumerate_pattern(&vec![4,2], |lane| {
        println!("lane : {}", lane);
    });
    let c = lane.apply_pattern(&vec![4,2]);
    assert!(c != 0);
    println!("final: {}", lane);
    assert_eq!(lane, Lane::from_str(". ### . # "));
}



pub struct Grid {
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
                if self.lanes[j].get_fill(i) { lane.set_fill(j as u8); }
                if self.lanes[j].get_hole(i) { lane.set_hole(j as u8); }
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

fn pattern_immediate_solvables(len: usize, pattern: &Vec<usize>) -> usize {
    let biggest_block = pattern.iter().max().unwrap();
    let wiggle_room: usize = len - (pattern.iter().sum::<usize>() + pattern.len()-1);
    biggest_block.saturating_sub(wiggle_room)
}

fn immediate_solvable_lanes(len: usize, patterns: &Vec<Vec<usize>>) -> u128 {
    let mut solvable_lanes = 0;
    for i in 0..patterns.len() {
        if pattern_immediate_solvables(len, &patterns[i]) > 0 {
            solvable_lanes |= 1 << i;
        }
    }
    solvable_lanes
}

pub fn solve(
    rows: &Vec<Vec<usize>>,
    cols: &Vec<Vec<usize>>,
) -> Grid {
    let nrows = rows.len();
    let ncols = cols.len();

    let mut grid = Grid::new(nrows, ncols);

    fn solve_lanes(lanes: &mut Vec<Lane>, patterns: &Vec<Vec<usize>>, lanes_to_examine: u128) -> u128 {
        let mut touched_ortho_lanes: u128 = 0;
        for i in 0..lanes.len() {
            if (lanes_to_examine & (1 << i)) != 0 {
                touched_ortho_lanes |= lanes[i].apply_pattern(&patterns[i]);
            }
        }
        touched_ortho_lanes
    }

    // Look at the rows that are solvable
    let mut solvable_lanes = immediate_solvable_lanes(ncols, &rows);
    solvable_lanes = solve_lanes(&mut grid.lanes, &rows, solvable_lanes);
    grid = grid.rotate();

    // Now we have columns that we can solve
    solvable_lanes |= immediate_solvable_lanes(nrows, &cols);

    loop {
        solvable_lanes = solve_lanes(&mut grid.lanes, &cols, solvable_lanes);
        grid = grid.rotate();

        if solvable_lanes == 0 {
            break;
        }

        solvable_lanes = solve_lanes(&mut grid.lanes, &rows, solvable_lanes);
        grid = grid.rotate();
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
