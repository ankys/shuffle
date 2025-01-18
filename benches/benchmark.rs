
extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

extern crate lib;
use lib::shuffle::shuffle::*;
use lib::cube2::shuffle_cube2::*;

fn criterion_benchmark(c: &mut Criterion) {
	let initial_state = get_initial_state();
	c.bench_function("cube2", |b| b.iter(|| shuffle(initial_state)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
