// cube2
pub mod shuffle_cube2 {

type Char = u32;
pub type State = [Char; 7];
// pub type Operation = isize;
pub type Operation = i8;

// const STATE_NUM: usize = 3674160;
pub const STATE_CODE_MAX: usize = 85766121;
pub const OPERATIONS: std::ops::Range<Operation> = 0..6;

pub fn get_initial_state() -> State {
	// [0, 3, 6, 9, 12, 15, 18]
	let str = "123111222333444555666456";
	parse_state(str.to_string()).unwrap()
}
pub fn get_terminal_state() -> State {
	// "123456789012345678901234567890".to_string() // invalid(1)
	// "123456789012345678901234".to_string() // invalid(1)
	// "123111222333444555666444".to_string() // invalid(1)
	// "123111222333444555666465".to_string() // invalid(2)
	// "123111222333444555666564".to_string() // no sol
	// "123111222333444555666456".to_string() // -
	// "ABCDDABBBCFFDDAEEECFFAEC".to_string() // 55
	let str = "ABCAADEEBCEFDDFABCEFCBDF"; // 6612426
	// "123114552356446123563246".to_string() // 6612426
	parse_state(str.to_string()).unwrap()
}
pub fn operate(s: &State, op: Operation) -> State {
	fn inc(c: Char) -> Char {
		if c % 3 == 2 { c - 2 } else { c + 1 }
	}
	fn dec(c: Char) -> Char {
		if c % 3 == 0 { c + 2 } else { c - 1 }
	}
	match op {
		0 => [s[5], s[6], s[2], s[3], s[4], s[1], s[0]],
		1 => [inc(s[6]), s[1], inc(s[4]), s[3], dec(s[0]), s[5], dec(s[2])],
		2 => [dec(s[4]), s[1], s[2], dec(s[5]), inc(s[3]), inc(s[0]), s[6]],
		3 => [s[6], s[5], s[2], s[3], s[4], s[0], s[1]],
		4 => [inc(s[4]), s[1], inc(s[6]), s[3], dec(s[2]), s[5], dec(s[0])],
		5 => [dec(s[5]), s[1], s[2], dec(s[4]), inc(s[0]), inc(s[3]), s[6]],
		_ => [s[0], s[1], s[2], s[3], s[4], s[5], s[6]], // error
	}
}
pub fn encode(s: &State) -> usize {
	// 0123 -> 1*4^2+2*4^1+3*4^0 = 27
	let code1 = s[1] as u32;
	let code2 = code1 * 21 + s[2] as u32;
	let code3 = code2 * 21 + s[3] as u32;
	let code4 = code3 * 21 + s[4] as u32;
	let code5 = code4 * 21 + s[5] as u32;
	let code6 = code5 * 21 + s[6] as u32;
	return code6 as usize;
}
pub fn inverse(op: Operation) -> Operation {
	return if op < 3 { op + 3 } else { op - 3 };
}

// const MAP_INDEX0: [usize; 24] = [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,17,18,19,20,21,22,23];
const MAP_INDEX1: [usize; 24] = [ 0, 1, 2,21,23,22,13, 9, 8, 3,11,16, 5,19, 6, 4,17,18,12, 7,20,14,15,10];
const MAP_INDEX2: [usize; 24] = [ 0, 1, 2, 9,15,12,14,19, 8, 7,23,10,18, 6,21,22,11,16,17,13,20, 3, 5, 4];
const MAP_BLOCK: [[usize; 3]; 21] = [
	[3, 5, 4],
	[4, 3, 5],
	[5, 4, 3],
	[3, 2, 1],
	[1, 3, 2],
	[2, 1, 3],
	[0, 2, 4],
	[4, 0, 2],
	[2, 4, 0],
	[0, 5, 1],
	[1, 0, 5],
	[5, 1, 0],
	[0, 4, 5],
	[5, 0, 4],
	[4, 5, 0],
	[3, 1, 5],
	[5, 3, 1],
	[1, 5, 3],
	[3, 4, 2],
	[2, 3, 4],
	[4, 2, 3],
];
pub fn parse_state(str: String) -> Result<State, String> {
	fn sub1(str: &[char], c1: char, c2: char) -> Result<char, String> {
		for i in 0..7 {
			let d0 = str[3 * i + 3];
			let d1 = str[3 * i + 4];
			let d2 = str[3 * i + 5];
			if [d0, d1] == [c1, c2] || [d0, d1] == [c2, c1] { return Ok(d2); }
			if [d1, d2] == [c1, c2] || [d1, d2] == [c2, c1] { return Ok(d0); }
			if [d2, d0] == [c1, c2] || [d2, d0] == [c2, c1] { return Ok(d1); }
		}
		return Err("invalid string (1)".to_string());
	}

	if str.len() != 24 {
		return Err("invalid string (1)".to_string());
	}
	let tmp1 = str.chars().collect::<Vec<_>>();
	let tmp2 = MAP_INDEX1.iter().map(|&i| tmp1[i]).collect::<Vec<_>>();
	let c0 = tmp2[0];
	let c1 = tmp2[1];
	let c2 = tmp2[2];
	let c3 = sub1(&tmp2, c1, c2)?;
	let c4 = sub1(&tmp2, c2, c0)?;
	let c5 = sub1(&tmp2, c0, c1)?;
	let cs = [c0, c1, c2, c3, c4, c5];
	// println!("{:?}", cs);

	let mut tmp3 = [0; 24];
	for k in 0..6 {
		let mut count = 0;
		for i in 0..24 {
			if tmp2[i] == cs[k] {
				tmp3[i] = k;
				count += 1;
			}
		}
		if count != 4 {
			return Err("invalid string (1)".to_string());
		}
	}
	let mut s = [0; 7];
	for i in 0..7 {
		let co = MAP_BLOCK.iter().position(|&b| b == tmp3[3 * i + 3 .. 3 * i + 6]);
		let c = co.ok_or("invalid string (2)".to_string())?;
		s[i] = c as Char;
	}
	return Ok(s);
}
pub fn dump_state(s: &State) -> String {
	let mut blocks = Vec::new();
	blocks.push([0, 1, 2]);
	for i in 0..7 {
		let c = s[i] as usize;
		blocks.push(MAP_BLOCK[c]);
	}
	#[allow(non_snake_case)]
	let ABCSU: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
	let mut tmp3 = Vec::new();
	for block in blocks {
		for j in 0..3 {
			tmp3.push(ABCSU[block[j]]);
		}
	}
	let str1 = MAP_INDEX2.iter().map(|&i| tmp3[i]).collect::<String>();

	#[allow(non_snake_case)]
	let ABCSL: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
	#[allow(non_snake_case)]
	let NUMS: Vec<char> = "0123456789".chars().collect();
	let mut cs2 = Vec::new();
	for i in 0..7 {
		let c = s[i] as usize;
		cs2.push(ABCSL[c / 3]);
		cs2.push(NUMS[c % 3]);
	}
	let str2 = cs2.iter().collect::<String>();

	return format!("{}[{}]", str1, str2);
}

}
