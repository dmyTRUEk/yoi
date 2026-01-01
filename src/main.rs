//! yoi

#![deny(
	unreachable_patterns,
	unused_results,
)]

use clap::Parser;



#[derive(Parser, Debug)]
#[clap(
	about,
	author,
	version,
	help_template = "\
		{before-help}{name} v{version}\n\
		\n\
		{about}\n\
		\n\
		Author: {author}\n\
		\n\
		{usage-heading} {usage}\n\
		\n\
		{all-args}{after-help}\
	",
)]
struct CliArgs {
	// TODO
	// #[arg(short='i', long, default_value_t=false)]
	// input_at_the_end: bool,

	program: Vec<String>,
}



fn main() {
	let CliArgs {
		program,
	} = CliArgs::parse();

	let program_stack = eval(&program.join(" "));

	println!("{:?}", program_stack.stack); // TODO
}



fn eval(program_str: &str) -> ProgramStack {
	let tokens: Vec<Token> = program_str
		.split(" ")
		.map(Token::from)
		.collect();

	let mut program_stack = ProgramStack::new();

	for token in tokens {
		program_stack.exec_mut(token);
		// dbg!(&program_stack);
		eprintln!("program stack: {:?}", program_stack.stack);
	}

	program_stack
}



#[derive(Debug, PartialEq)]
struct ProgramStack {
	stack: Vec<StackElement>,
}
impl ProgramStack {
	fn new() -> Self {
		Self { stack: vec![] }
	}
	fn exec_mut(&mut self, token: Token) {
		exec(self, token);
	}
	#[allow(dead_code)] // it is for tests
	fn exec_val(mut self, token: Token) -> Self {
		exec(&mut self, token);
		self
	}
}
impl From<&[String]> for ProgramStack {
	fn from(stack_elements: &[String]) -> Self {
		Self {
			stack: stack_elements.iter().map(|se| StackElement::from(se.as_str())).collect(),
		}
	}
}
impl From<StackElement> for ProgramStack {
	fn from(stack_element: StackElement) -> Self {
		Self { stack: vec![stack_element] }
	}
}
impl<const N: usize> From<[StackElement; N]> for ProgramStack {
	fn from(stack_elements: [StackElement; N]) -> Self {
		Self { stack: stack_elements.to_vec() }
	}
}



#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
enum StackElement {
	Int(i64),
	ArrInt(Vec<i64>),
	TokenLiteral(Box<Token>),
}
impl From<&str> for StackElement {
	fn from(value: &str) -> Self {
		use StackElement::*;
		// dbg!(value);
		if let Ok(n) = value.parse::<i64>() {
			Int(n)
		}
		else if value.contains(",") {
			ArrInt(
				value.split(",").map(|n| n.parse().unwrap()).collect()
			)
		}
		else {
			unimplemented!()
		}
	}
}



#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
enum Token {
	Literal(StackElement),

	Abs,
	AtIndex,
	Decrease,
	Digits,
	Duplicate,
	First,
	Increase,
	IndexOfMaxFirst,
	IndexOfMaxLast,
	IndexOfMinFirst,
	IndexOfMinLast,
	Join,
	Last,
	// Map,
	Max,
	Min,
	Negate,
	// TODO: range: to/from? (aka ascending/descending)
	Range0Excluding,
	Range0Including,
	Range1Excluding,
	Range1Including,
	Reverse,
	// TODO: SliceArr 0,1,2,3,4,5,6 2,5 slicearr -> 2,3,4,5
	SliceExcludingExcluding,
	SliceExcludingIncluding,
	SliceExcludingFrom,
	SliceExcludingTo,
	SliceIncludingExcluding,
	SliceIncludingIncluding,
	SliceIncludingFrom,
	SliceIncludingTo,
	Sort,
	Swap,
	// SwapN - swap with top with nth / n from top
	// SwapNM
}
impl Token {
}
impl From<&str> for Token {
	fn from(token_str: &str) -> Self {
		use Token::*;
		// dbg!(token_str);
		if let Some(token_str) = token_str.strip_prefix("'") {
			Literal(StackElement::TokenLiteral(Box::new(Token::from(token_str))))
		}
		else {
			match token_str {
				"abs" => Abs,
				"at" => AtIndex,
				"dec" => Decrease,
				"digits" => Digits,
				"dup" => Duplicate,
				"first" => First,
				"imaxf" => IndexOfMaxFirst,
				"imaxl" => IndexOfMaxLast,
				"iminf" => IndexOfMinFirst,
				"iminl" => IndexOfMinLast,
				"inc" => Increase,
				"join" => Join,
				"last" => Last,
				// "map" => Map,
				"max" => Max,
				"min" => Min,
				"neg" => Negate,
				"range0excl" => Range0Excluding,
				"range0incl" => Range0Including,
				"range1excl" => Range1Excluding,
				"range1incl" => Range1Including,
				"rev" => Reverse,
				"sliceexclexcl" => SliceExcludingExcluding,
				"sliceexclincl" => SliceExcludingIncluding,
				"sliceexclfrom" => SliceExcludingFrom,
				"sliceexclto" => SliceExcludingTo,
				"sliceinclexcl" => SliceIncludingExcluding,
				"sliceinclincl" => SliceIncludingIncluding,
				"sliceinclfrom" => SliceIncludingFrom,
				"sliceinclto" => SliceIncludingTo,
				"sort" => Sort,
				"swap" => Swap,
				_ => Literal(StackElement::from(token_str))
			}
		}
	}
}





fn exec(program_stack: &mut ProgramStack, token: Token) {
	use StackElement::*;
	use Token::*;
	match token {
		Literal(literal) => {
			program_stack.stack.push(literal);
		}
		// TokenLiteral(_token) => { // TODO: process Literal(Token) somehow?
		// 	// nothing
		// }
		Abs => {
			let top = program_stack.stack.last_mut().unwrap();
			match top {
				Int(n) => {
					*n = n.abs();
				}
				ArrInt(v) => {
					for el in v {
						*el = el.abs();
					}
				}
				TokenLiteral(_) => panic!()
			}
		}
		AtIndex => {
			let i = program_stack.stack.pop().unwrap();
			let v = program_stack.stack.pop().unwrap();
			match (i, v) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(Int(v[i as usize]));
				}
				_ => panic!()
			}
		}
		Decrease => {
			let i = program_stack.stack.last_mut().unwrap();
			match i {
				Int(i) => {
					*i -= 1;
				}
				_ => panic!()
			}
		}
		Digits => {
			let i = program_stack.stack.pop().unwrap();
			match i {
				Int(mut i) => {
					assert!(i >= 0);
					let mut digits = vec![];
					while i > 0 {
						digits.push(i % 10);
						i /= 10;
					}
					digits.reverse();
					program_stack.stack.push(ArrInt(digits));
				}
				_ => panic!()
			}
		}
		Duplicate => {
			program_stack.stack.push(program_stack.stack.last().unwrap().clone());
		}
		First => {
			let v = program_stack.stack.pop().unwrap();
			match v {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.first().unwrap()));
				}
				_ => panic!()
			}
		}
		Increase => {
			let i = program_stack.stack.last_mut().unwrap();
			match i {
				Int(i) => {
					*i += 1;
				}
				_ => panic!()
			}
		}
		IndexOfMaxFirst => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				ArrInt(v) => {
					let mut index_of_max_first = 0;
					let (mut max, v) = v.split_first().unwrap();
					for (i, el) in v.iter().enumerate() {
						if el > max {
							max = el;
							index_of_max_first = i + 1; // +1 bc we popped first element
						}
					}
					program_stack.stack.push(Int(index_of_max_first as i64));
				}
				_ => panic!()
			}
		}
		IndexOfMaxLast => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				ArrInt(v) => {
					let mut index_of_max_last = 0;
					let (mut max, v) = v.split_first().unwrap();
					for (i, el) in v.iter().enumerate() {
						if el >= max {
							max = el;
							index_of_max_last = i + 1; // +1 bc we popped first element
						}
					}
					program_stack.stack.push(Int(index_of_max_last as i64));
				}
				_ => panic!()
			}
		}
		IndexOfMinFirst => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				ArrInt(v) => {
					let mut index_of_min_first = 0;
					let (mut min, v) = v.split_first().unwrap();
					for (i, el) in v.iter().enumerate() {
						if el < min {
							min = el;
							index_of_min_first = i + 1; // +1 bc we popped first element
						}
					}
					program_stack.stack.push(Int(index_of_min_first as i64));
				}
				_ => panic!()
			}
		}
		IndexOfMinLast => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				ArrInt(v) => {
					let mut index_of_min_last = 0;
					let (mut min, v) = v.split_first().unwrap();
					for (i, el) in v.iter().enumerate() {
						if el <= min {
							min = el;
							index_of_min_last = i + 1; // +1 bc we popped first element
						}
					}
					program_stack.stack.push(Int(index_of_min_last as i64));
				}
				_ => panic!()
			}
		}
		Join => {
			let top = program_stack.stack.pop().unwrap();
			let pretop = program_stack.stack.pop().unwrap();
			let new_top = match (pretop, top) {
				(Int(pt), Int(t)) => {
					ArrInt(vec![pt, t])
				}
				(ArrInt(mut pt), ArrInt(mut t)) => {
					pt.append(&mut t);
					ArrInt(pt)
				}
				(ArrInt(mut pt), Int(t)) => {
					pt.push(t);
					ArrInt(pt)
				}
				(Int(pt), ArrInt(mut t)) => {
					t.insert(0, pt);
					ArrInt(t)
				}
				_ => panic!()
			};
			program_stack.stack.push(new_top);
		}
		Last => {
			let v = program_stack.stack.pop().unwrap();
			match v {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.last().unwrap()));
				}
				_ => panic!()
			}
		}
		// Map => {
		// 	let f = program_stack.stack.pop().unwrap();
		// 	let v = program_stack.stack.pop().unwrap();
		// 	match (v, f) {
		// 		(ArrInt(v), TokenLiteral(f)) => {
		// 			let res = v.into_iter()
		// 				.map(|el| {
		// 					ProgramStack::new()
		// 						.exec_val(*f)
		// 						.stack
		// 						.last().unwrap()
		// 				})
		// 				.collect();
		// 			program_stack.stack.push(ArrInt(res));
		// 		}
		// 		_ => panic!()
		// 	}
		// }
		Max => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.iter().max().unwrap()));
				}
				_ => panic!()
			}
		}
		Min => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.iter().min().unwrap()));
				}
				_ => panic!()
			}
		}
		Negate => {
			let top = program_stack.stack.last_mut().unwrap();
			match top {
				Int(n) => {
					*n = -*n;
				}
				ArrInt(v) => {
					for el in v {
						*el = -*el;
					}
				}
				TokenLiteral(_) => panic!()
			}
		}
		Range0Excluding => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (0..n).collect() ));
				}
				_ => panic!()
			}
		}
		Range0Including => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (0..=n).collect() ));
				}
				_ => panic!()
			}
		}
		Range1Excluding => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (1..n).collect() ));
				}
				_ => panic!()
			}
		}
		Range1Including => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (1..=n).collect() ));
				}
				_ => panic!()
			}
		}
		Reverse => {
			let top = program_stack.stack.last_mut().unwrap();
			match top {
				ArrInt(v) => {
					v.reverse();
				}
				_ => panic!()
			}
		}
		SliceExcludingExcluding => {
			let index_to = program_stack.stack.pop().unwrap();
			let index_from = program_stack.stack.pop().unwrap();
			let v = program_stack.stack.pop().unwrap();
			match (index_from, index_to, v) {
				(Int(index_from), Int(index_to), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[index_from as usize + 1 .. index_to as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceExcludingIncluding => {
			let index_to = program_stack.stack.pop().unwrap();
			let index_from = program_stack.stack.pop().unwrap();
			let v = program_stack.stack.pop().unwrap();
			match (index_from, index_to, v) {
				(Int(index_from), Int(index_to), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[index_from as usize + 1 ..= index_to as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceExcludingFrom => {
			let i = program_stack.stack.pop().unwrap();
			let v = program_stack.stack.pop().unwrap();
			match (i, v) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[i as usize + 1 ..].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceExcludingTo => {
			let i = program_stack.stack.pop().unwrap();
			let v = program_stack.stack.pop().unwrap();
			match (i, v) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[.. i as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceIncludingExcluding => {
			let index_to = program_stack.stack.pop().unwrap();
			let index_from = program_stack.stack.pop().unwrap();
			let v = program_stack.stack.pop().unwrap();
			match (index_from, index_to, v) {
				(Int(index_from), Int(index_to), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[index_from as usize .. index_to as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceIncludingIncluding => {
			let index_to = program_stack.stack.pop().unwrap();
			let index_from = program_stack.stack.pop().unwrap();
			let v = program_stack.stack.pop().unwrap();
			match (index_from, index_to, v) {
				(Int(index_from), Int(index_to), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[index_from as usize ..= index_to as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceIncludingFrom => {
			let i = program_stack.stack.pop().unwrap();
			let v = program_stack.stack.pop().unwrap();
			match (i, v) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[i as usize ..].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceIncludingTo => {
			let i = program_stack.stack.pop().unwrap();
			let v = program_stack.stack.pop().unwrap();
			match (i, v) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[..= i as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		Sort => {
			let top = program_stack.stack.last_mut().unwrap();
			match top {
				ArrInt(v) => {
					v.sort();
				}
				_ => panic!()
			}
		}
		Swap => {
			let len = program_stack.stack.len();
			program_stack.stack.swap(len - 1, len - 2);
		}
	}
}





#[cfg(test)]
mod token_exec {
	use super::*;
	use StackElement::*;
	use Token::*;

	mod join {
		use super::*;
		#[test]
		fn int_int() {
			assert_eq!(
				ProgramStack::from(ArrInt(vec![1, 2])),
				ProgramStack::from([Int(1), Int(2)]).exec_val(Join)
			)
		}
		#[test]
		fn vi_vi() {
			assert_eq!(
				ProgramStack::from(ArrInt(vec![1,2,3,4])),
				ProgramStack::from([ArrInt(vec![1,2]), ArrInt(vec![3,4])]).exec_val(Join)
			)
		}
	}

	mod sort {
		use super::*;
		#[test]
		fn _0_1_2_3_4_5_6_7_8_9() {
			assert_eq!(
				ProgramStack::from(ArrInt(vec![0,1,2,3,4,5,6,7,8,9])),
				ProgramStack::from(ArrInt(vec![5,9,1,3,4,0,8,7,2,6])).exec_val(Sort)
			)
		}
	}
}





#[allow(non_snake_case)]
#[cfg(test)]
mod program_exec {
	use super::*;
	mod token {
		use super::*;
		mod abs {
			use super::*;
			#[test]
			fn int_pos() {
				assert_eq!(
					eval("42"),
					eval("42 abs")
				)
			}
			#[test]
			fn int_neg() {
				assert_eq!(
					eval("42"),
					eval("-42 abs")
				)
			}
			#[test]
			fn vi() {
				assert_eq!(
					eval("1,2,3"),
					eval("-1,2,-3 abs")
				)
			}
		}
		mod at_index {
			use super::*;
			#[test]
			fn _10_20_30__0() {
				assert_eq!(
					eval("10"),
					eval("10,20,30 0 at")
				)
			}
			#[test]
			fn _10_20_30__1() {
				assert_eq!(
					eval("20"),
					eval("10,20,30 1 at")
				)
			}
			#[test]
			fn _10_20_30__2() {
				assert_eq!(
					eval("30"),
					eval("10,20,30 2 at")
				)
			}
		}
		mod decrease {
			use super::*;
			#[test]
			fn _42() {
				assert_eq!(
					eval("41"),
					eval("42 dec")
				)
			}
		}
		mod digits {
			use super::*;
			#[test]
			fn _31415() {
				assert_eq!(
					eval("3,1,4,1,5"),
					eval("31415 digits")
				)
			}
		}
		mod duplicate {
			use super::*;
			#[test]
			fn int() {
				assert_eq!(
					eval("42 42"),
					eval("42 dup")
				)
			}
			#[test]
			fn vi() {
				assert_eq!(
					eval("1,2,3 1,2,3"),
					eval("1,2,3 dup")
				)
			}
		}
		mod first {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("1"),
					eval("1,2,3 first")
				)
			}
		}
		mod increase {
			use super::*;
			#[test]
			fn _42() {
				assert_eq!(
					eval("43"),
					eval("42 inc")
				)
			}
		}
		mod index_of {
			use super::*;
			mod max {
				use super::*;
				mod first {
					use super::*;
					#[test]
					fn _5_9_1_3_4_0_8_7_2_6() {
						assert_eq!(
							eval("1"),
							eval("5,9,1,3,4,0,8,7,2,6 imaxf")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("1"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 imaxf")
						)
					}
				}
				mod last {
					use super::*;
					#[test]
					fn _5_9_1_3_4_0_8_7_2_6() {
						assert_eq!(
							eval("1"),
							eval("5,9,1,3,4,0,8,7,2,6 imaxl")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("8"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 imaxl")
						)
					}
				}
			}
			mod min {
				use super::*;
				mod first {
					use super::*;
					#[test]
					fn _5_9_1_3_4_0_8_7_2_6() {
						assert_eq!(
							eval("5"),
							eval("5,9,1,3,4,0,8,7,2,6 iminf")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("3"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 iminf")
						)
					}
				}
				mod last {
					use super::*;
					#[test]
					fn _5_9_1_3_4_0_8_7_2_6() {
						assert_eq!(
							eval("5"),
							eval("5,9,1,3,4,0,8,7,2,6 iminl")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("6"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 iminl")
						)
					}
				}
			}
		}
		mod join {
			use super::*;
			#[test]
			fn int_int() {
				assert_eq!(
					eval("1,2"),
					eval("1 2 join")
				)
			}
			#[test]
			fn vi_vi() {
				assert_eq!(
					eval("1,2,3,4"),
					eval("1,2 3,4 join")
				)
			}
			#[test]
			fn vi_int() {
				assert_eq!(
					eval("1,2,3"),
					eval("1,2 3 join")
				)
			}
			#[test]
			fn int_vi() {
				assert_eq!(
					eval("1,2,3"),
					eval("1 2,3 join")
				)
			}
		}
		mod last {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("3"),
					eval("1,2,3 last")
				)
			}
		}
		// mod map {
		// 	use super::*;
		// 	#[test]
		// 	fn _1_2_3_abs() {
		// 		assert_eq!(
		// 			eval("1,2,3"),
		// 			eval("-1,2,-3 'abs map")
		// 		)
		// 	}
		// }
		mod max {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("3"),
					eval("1,2,3 max")
				)
			}
		}
		mod min {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("1"),
					eval("1,2,3 min")
				)
			}
		}
		mod negate {
			use super::*;
			#[test]
			fn int() {
				assert_eq!(
					eval("-42"),
					eval("42 neg")
				)
			}
			#[test]
			fn vi() {
				assert_eq!(
					eval("-1,-2,-3"),
					eval("1,2,3 neg")
				)
			}
		}
		mod range {
			use super::*;
			mod _0 {
				use super::*;
				mod excluding {
					use super::*;
					#[test]
					fn _5() {
						assert_eq!(
							eval("0,1,2,3,4"),
							eval("5 range0excl")
						)
					}
				}
				mod including {
					use super::*;
					#[test]
					fn _5() {
						assert_eq!(
							eval("0,1,2,3,4,5"),
							eval("5 range0incl")
						)
					}
				}
			}
			mod _1 {
				use super::*;
				mod excluding {
					use super::*;
					#[test]
					fn _5() {
						assert_eq!(
							eval("1,2,3,4"),
							eval("5 range1excl")
						)
					}
				}
				mod including {
					use super::*;
					#[test]
					fn _5() {
						assert_eq!(
							eval("1,2,3,4,5"),
							eval("5 range1incl")
						)
					}
				}
			}
		}
		mod reverse {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("3,2,1"),
					eval("1,2,3 rev")
				)
			}
		}
		mod slice {
			use super::*;
			mod excluding {
				use super::*;
				mod from {
					use super::*;
					#[test]
					fn _0_1_2_3_4_5_6_7_8_9() {
						assert_eq!(
							eval("4,5,6,7,8,9"),
							eval("0,1,2,3,4,5,6,7,8,9 3 sliceexclfrom")
						)
					}
				}
				mod to {
					use super::*;
					#[test]
					fn _0_1_2_3_4_5_6_7_8_9() {
						assert_eq!(
							eval("0,1,2"),
							eval("0,1,2,3,4,5,6,7,8,9 3 sliceexclto")
						)
					}
				}
				mod from_to {
					use super::*;
					mod excluding {
						use super::*;
						#[test]
						fn _0_1_2_3_4_5_6_7_8_9() {
							assert_eq!(
								eval("4,5"),
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceexclexcl")
							)
						}
					}
					mod including {
						use super::*;
						#[test]
						fn _0_1_2_3_4_5_6_7_8_9() {
							assert_eq!(
								eval("4,5,6"),
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceexclincl")
							)
						}
					}
				}
			}
			mod including {
				use super::*;
				mod from {
					use super::*;
					#[test]
					fn _0_1_2_3_4_5_6_7_8_9() {
						assert_eq!(
							eval("3,4,5,6,7,8,9"),
							eval("0,1,2,3,4,5,6,7,8,9 3 sliceinclfrom")
						)
					}
				}
				mod to {
					use super::*;
					#[test]
					fn _0_1_2_3_4_5_6_7_8_9() {
						assert_eq!(
							eval("0,1,2,3"),
							eval("0,1,2,3,4,5,6,7,8,9 3 sliceinclto")
						)
					}
				}
				mod from_to {
					use super::*;
					mod excluding {
						use super::*;
						#[test]
						fn _0_1_2_3_4_5_6_7_8_9() {
							assert_eq!(
								eval("3,4,5"),
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceinclexcl")
							)
						}
					}
					mod including {
						use super::*;
						#[test]
						fn _0_1_2_3_4_5_6_7_8_9() {
							assert_eq!(
								eval("3,4,5,6"),
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceinclincl")
							)
						}
					}
				}
			}
		}
		mod sort {
			use super::*;
			#[test]
			fn _0_1_2_3_4_5_6_7_8_9() {
				assert_eq!(
					eval("0,1,2,3,4,5,6,7,8,9"),
					eval("5,9,1,3,4,0,8,7,2,6 sort")
				)
			}
		}
		mod swap {
			use super::*;
			#[test]
			fn int_int() {
				assert_eq!(
					eval("2 1"),
					eval("1 2 swap")
				)
			}
			#[test]
			fn vi_vi() {
				assert_eq!(
					eval("3,4 1,2"),
					eval("1,2 3,4 swap")
				)
			}
			#[test]
			fn vi_int() {
				assert_eq!(
					eval("3 1,2"),
					eval("1,2 3 swap")
				)
			}
			#[test]
			fn int_vi() {
				assert_eq!(
					eval("2,3 1"),
					eval("1 2,3 swap")
				)
			}
		}
	}
}

