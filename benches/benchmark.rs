
extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate src;
use src::shuffle::shuffle::*;
use src::cube2::shuffle_cube2::*;

fn criterion_benchmark(c: &mut Criterion) {
	let initial_state = get_initial_state();
	c.bench_function("encode", |b| b.iter(|| encode(&black_box(initial_state))));
	c.bench_function("operate", |b| b.iter(|| operate(&black_box(initial_state), 0)));
	// c.bench_function("cube2", |b| b.iter(|| shuffle(initial_state)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
