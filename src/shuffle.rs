// shuffle
pub mod shuffle {

use cube2::shuffle_cube2::{ State, Operation, STATE_CODE_MAX, OPERATIONS, operate, encode };
// use { State, Operation, STATE_CODE_MAX, OPERATIONS, operate, encode };

pub type Cache = (Vec<(State, usize, Option<(usize, Operation)>)>, Vec<(usize, usize)>);
pub fn shuffle(initial_state: State) -> Cache {
	let mut states = Vec::new();
	let mut levels = Vec::new();
	const BIT_LEN: usize = 64;
	let mut flags = [0 as u64; STATE_CODE_MAX / BIT_LEN + 1];
	// for state in initial_states {
	{
		let state = initial_state;
		let code = encode(&state);
		states.push((state, code, None));
		flags[code / BIT_LEN] |= 1 << (code % BIT_LEN);
	}
	let mut start = 0;
	let mut goal = states.len();
	for p in 0..STATE_CODE_MAX {
		if p == goal {
			levels.push((start, goal));
			start = goal;
			goal = states.len();
		}
		if p >= states.len() {
			break;
		}
		let state = states[p].0;
		for op in OPERATIONS {
			let state2 = operate(&state, op);
			let code2 = encode(&state2);
			if flags[code2 / BIT_LEN] & 1 << (code2 % BIT_LEN) == 0 {
				states.push((state2, code2, Some((p, op))));
				flags[code2 / BIT_LEN] |= 1 << (code2 % BIT_LEN);
			}
		}
	}
	let cache = (states, levels);
	return cache;
}
pub fn feedback(cache: &Cache, state: &State) -> Option<Vec<usize>> {
	let code = encode(state);
	let mut r = None;
	for p in 0..cache.0.len() {
		if cache.0[p].1 == code {
			r = Some(p);
			break;
		}
	}
	if r.is_none() {
		return None;
	}
	let mut list = Vec::new();
	let mut p = r.unwrap();
	list.push(p);
	for _level in 0..256 {
		let r = cache.0[p].2;
		if r.is_none() {
			break;
		}
		p = r.unwrap().0;
		list.push(p);
	}
	return Some(list);
}

}
