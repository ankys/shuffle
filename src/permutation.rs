// permutation
pub mod shuffle_permutation {

pub type State = String;
pub type Operation = isize;

const STR_NUM: Operation = 6;
// const STATE_NUM: usize = 120;
pub const STATE_CODE_MAX: usize = 1000000;
pub const OPERATIONS: std::ops::Range<Operation> = 0..(STR_NUM - 1);

pub fn get_initial_state() -> State {
	"123456".to_string()
}
pub fn get_terminal_state() -> State {
	"654321".to_string()
}
pub fn operate(s: &State, op: Operation) -> State {
	let op = op as usize;
	format!("{}{}{}{}", &s[..op], &s[op + 1..op + 2], &s[op..op + 1], &s[op + 2..])
}
pub fn encode(s: &State) -> usize {
	s.parse().unwrap()
}
pub fn inverse(op: Operation) -> Operation {
	return op;
}

pub fn parse_state(str: String) -> Result<State, String> {
	if str.len() != STR_NUM as usize {
		return Err("invalid string (1)".to_string());
	}
	if let Err(_e) = str.parse::<usize>() {
		return Err("invalid string (2)".to_string());
	}
	return Ok(str);
}
pub fn dump_state(s: &State) -> String {
	return s.to_string();
}

}
