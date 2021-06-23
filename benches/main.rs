use criterion::{criterion_group, criterion_main, Criterion};
use crossme::problems::*;
use crossme::solve;

fn criterion_benchmark(c: &mut Criterion) {
	let mut pb = |name, f: fn() -> (Vec<Vec<usize>>, Vec<Vec<usize>>)| {
		let (rows, cols) = f();
		c.bench_function(name, |b| b.iter(|| solve(&rows, &cols)));
	};

	pb("def_2_29", def_2_29);
	pb("def_4_1", def_4_1);
	pb("def_5_392", def_5_392);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
