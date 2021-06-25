use crossme::problems::*;
use crossme::solve;

fn main() {
	let pb = |name, f: fn() -> (Vec<Vec<usize>>, Vec<Vec<usize>>)| {
		let (rows, cols) = f();
		let grid = solve(&rows, &cols);
        println!("{}", name);
        grid.print();
	};

	pb("def_2_29", def_2_29);
	pb("def_4_1", def_4_1);
	pb("def_5_392", def_5_392);
}
