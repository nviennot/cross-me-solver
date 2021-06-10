#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut, unused_variables))]

use ndarray::Array2;

fn main() {
    println!("Hello, world!");
}

fn pattern_permutations(pattern: &Vec<usize>, len: usize) {
    let num_fulls: usize = pattern.iter().sum();
    let num_holes = len - num_fulls;
    let num_blocks = pattern.len();
    let num_symbols = num_holes + num_blocks;

    assert!(num_holes >= num_blocks-1);

    let mut lane: Vec<bool> = Vec::with_capacity(num_symbols);
    lane.resize(num_symbols, false);

    fn print_lane(pattern: &Vec<usize>, lane: &Vec<bool>) {
        let mut i = 0;
        for v in lane.iter() {
            if *v {
                for _ in 0..pattern[i] { print!("{}", "X"); }
                i+=1;
            } else {
                print!(".");
            }
        }
        println!("");
    }

    fn inner(pattern: &Vec<usize>, lane: &mut Vec<bool>, i: usize, num_blocks: usize, num_holes: usize) {
        if i == lane.len() {
            print_lane(pattern, lane);
            return;
        }

        if num_holes > 0 {
            lane[i] = false;
            inner(pattern, lane, i+1, num_blocks, num_holes-1);
        }

        if num_blocks > 0 && (i == 0 || lane[i-1] == false) {
            lane[i] = true;
            inner(pattern, lane, i+1, num_blocks-1, num_holes);
        }
    }
    inner(pattern, &mut lane, 0, num_blocks, num_holes);
}

#[test]
fn test_pattern1() {
    pattern_permutations(&vec![4,2], 10);
}

struct Grid {
    vals: Array2::<bool>,
    mask: Array2::<bool>,
}

impl Grid {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        let mut vals = Array2::<bool>::default((nrows, ncols));
        let mut mask = Array2::<bool>::default((nrows, ncols));
        Self { vals, mask }
    }

    pub fn print(&self) {
        for i in 0..self.vals.nrows() {
            for j in 0..self.vals.ncols() {
                if self.mask[[i,j]] {
                    print!("{}", if self.vals[[i,j]] { "#" } else { "." });
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    /*
    pub fn solve_row(&self, i: usize, pattern: Vec<usize>) {
        let vals = self.vals.row_mut(i);
        let mask = self.mask.row_mut(i);
    }
    */
}

fn solve(
    rows: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
) {
    let n = rows.len();
    let m = cols.len();

    let mut grid = Grid::new(n,m);

    /*
    pub fn solve_lane(&self, i: usize, pattern: Vec<usize>) {
    }
    */
}

#[test]
fn test_2_29() {
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

    solve(cols, rows);
}
