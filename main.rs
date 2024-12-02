
mod shuffle;
mod cube2;
use shuffle::shuffle::*;
use cube2::shuffle_cube2::*;

fn print_result(cache: &Cache, result: Option<Vec<usize>>) {
	if result.is_none() {
		println!("no solution");
		return;
	}
	let mut list = result.unwrap();
	list.reverse();
	for p in list {
		let (state, code, previous) = cache.0[p];
		if previous.is_none() {
			println!("{} ({})", dump_state(&state), code);
		} else {
			let op1 = previous.unwrap().1;
			let op2 = inverse(op1);
			println!("<- {}/{} -> {} ({})", op1 + 1, op2 + 1, dump_state(&state), code);
		}
	}
}
#[allow(unreachable_code)]
#[allow(unused_variables)]
fn main() {
	let initial_state = get_initial_state();
	let terminal_state = get_terminal_state();

	// print(state_terminal, find(state_initial, state_terminal));
	// return;

	let cache = shuffle(initial_state);

	for level in 0..cache.1.len() {
		let (start, end) = cache.1[level];
		println!("level: {}, patterns: {}", level, end - start);
	}
	println!("total patterns: {}", cache.0.len());
	return;

	let result = feedback(&cache, &terminal_state);
	print_result(&cache, result);
	return;

	loop {
		eprint!("> ");
		let mut str = String::new();
		std::io::stdin().read_line(&mut str).unwrap();
		str = str.trim().to_string();

		let result = parse_state(str);
		if let Err(msg) = result.clone() {
			println!("{}", msg);
		}
		let state = result.unwrap();
		let result = feedback(&cache, &state);
		print_result(&cache, result);
	}
}
