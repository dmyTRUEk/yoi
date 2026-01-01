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
enum StackElement {
	Int(i64),
	VecInt(Vec<i64>),
}
impl From<&str> for StackElement {
	fn from(value: &str) -> Self {
		use StackElement::*;
		// dbg!(value);
		if let Ok(n) = value.parse::<i64>() {
			Int(n)
		}
		else if value.contains(",") {
			VecInt(
				value.split(",").map(|n| n.parse().unwrap()).collect()
			)
		}
		else {
			todo!()
		}
	}
}



enum Token {
	Literal(StackElement),

	Join,
	Sort,
}
impl Token {
}
impl From<&str> for Token {
	fn from(token_str: &str) -> Self {
		use Token::*;
		// dbg!(token_str);
		match token_str {
			"join" => Join,
			"sort" => Sort,
			_ => Literal(StackElement::from(token_str))
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
		Join => {
			let top = program_stack.stack.pop().unwrap();
			let pretop = program_stack.stack.pop().unwrap();
			let new_top = match (pretop, top) {
				(Int(pt), Int(t)) => {
					VecInt(vec![pt, t])
				}
				(VecInt(mut pt), VecInt(mut t)) => {
					pt.append(&mut t);
					VecInt(pt)
				}
				_ => panic!()
			};
			program_stack.stack.push(new_top);
		}
		Sort => {
			let top = program_stack.stack.last_mut().unwrap();
			match top {
				Int(_) => panic!(),
				VecInt(v) => {
					v.sort();
				}
			}
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
				ProgramStack::from(VecInt(vec![1, 2])),
				ProgramStack::from([Int(1), Int(2)]).exec_val(Join)
			)
		}
		#[test]
		fn vi_vi() {
			assert_eq!(
				ProgramStack::from(VecInt(vec![1,2,3,4])),
				ProgramStack::from([VecInt(vec![1,2]), VecInt(vec![3,4])]).exec_val(Join)
			)
		}
	}

	mod sort {
		use super::*;
		#[test]
		fn _0_1_2_3_4_5_6_7_8_9() {
			assert_eq!(
				ProgramStack::from(VecInt(vec![0,1,2,3,4,5,6,7,8,9])),
				ProgramStack::from(VecInt(vec![5,9,1,3,4,0,8,7,2,6])).exec_val(Sort)
			)
		}
	}
}





#[cfg(test)]
mod program_exec {
	use super::*;
	mod token {
		use super::*;
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
	}
}

