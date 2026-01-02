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
	// TODO?
	// #[arg(short='i', long, default_value_t=false)]
	// input_at_the_end: bool,

	program: Vec<String>,
}



fn main() {
	let CliArgs {
		program,
	} = CliArgs::parse();

	let program_stack = eval(&program.join(" "));

	eprintln!();
	println!("{:?}", program_stack.stack); // TODO
}



fn eval(program_str: &str) -> ProgramStack {
	let tokens: Vec<Token> = program_str
		.split(" ")
		.filter(|s| !s.is_empty())
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
	// TokenLiteral(Box<Token>),
}
impl From<&str> for StackElement {
	fn from(s: &str) -> Self {
		use StackElement::*;
		// dbg!(s);
		if let Ok(n) = s.parse::<i64>() {
			Int(n)
		}
		else if s.contains(",") {
			ArrInt(
				s.split(",").map(|n| n.parse().unwrap()).collect()
			)
		}
		else {
			unimplemented!("`{s}`")
		}
	}
}



#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
enum Token {
	Literal(StackElement),

	// TODO: add combinators: https://combinatorylogic.com/table.html

	Abs,
	AbsX,
	Add,
	AddX,
	AtIndex,
	AtIndexX,
	Bottom, // remove everything but bottom of the stack
	Decrease,
	DecreaseX,
	Digits,
	DigitsX,
	DivideInt,
	DivideIntX,
	// TODO: rename to copy?
	DuplicateFromIndex,
	// DuplicateFromIndexX,
	DuplicateToIndex,
	// DuplicateToIndexX,
	DuplicateToBottom,
	// DuplicateToBottomX,
	DuplicateTop,
	// DuplicateTopX,
	DuplicateUnder,
	// DuplicateUnderX,
	First,
	FirstX,
	Head, // everything but last
	HeadX,
	Increase,
	IncreaseX,
	IndexOfMaxFirst,
	IndexOfMaxFirstX,
	IndexOfMaxLast,
	IndexOfMaxLastX,
	IndexOfMinFirst,
	IndexOfMinFirstX,
	IndexOfMinLast,
	IndexOfMinLastX,
	// InsertAtVI, // 0,1,2,3  5 2 insertat -> 0,1,5,2,3
	// InsertAtIV, // 0,1,2,3  2 5 insertat -> 0,1,5,2,3
	IsAllEqual, // 2,2,2 -> 1
	IsAllEqualX,
	// IsAllNotEqual, // ?
	// IsAllNotEqualX,
	IsEqual, // 42 42 -> 1
	IsEqualX,
	IsNotEqual, // 42 137 -> 1
	IsNotEqualX,
	// TODO: element-wise equal/not-equal
	Join,
	JoinX,
	JoinDigits,
	JoinDigitsX,
	Last,
	LastX,
	// Map,
	Max,
	MaxX,
	Min,
	MinX,
	ModuloFake,
	ModuloFakeX,
	ModuloRemEuclid,
	ModuloRemEuclidX,
	// TODO: unify/rename with dup! ? (e2dc6d)
	//Move, // noop
	MoveFromIndex,
	MoveToIndex,
	MoveToBottom,
	Multiply,
	MultiplyX,
	Negate,
	NegateX,
	Not,
	NotX,
	Pop, // remove top stack element
	// TODO: range: to/from? (aka ascending/descending)
	Range0Excluding,
	Range0ExcludingX,
	Range0Including,
	Range0IncludingX,
	Range1Excluding,
	Range1ExcludingX,
	Range1Including,
	Range1IncludingX,
	Reverse,
	ReverseX,
	// TODO: SliceArr 0,1,2,3,4,5,6 2,5 slicearr -> 2,3,4,5
	SliceExcludingExcluding,
	SliceExcludingExcludingX,
	SliceExcludingIncluding,
	SliceExcludingIncludingX,
	SliceExcludingFrom,
	SliceExcludingFromX,
	SliceExcludingTo,
	SliceExcludingToX,
	SliceIncludingExcluding,
	SliceIncludingExcludingX,
	SliceIncludingIncluding,
	SliceIncludingIncludingX,
	SliceIncludingFrom,
	SliceIncludingFromX,
	SliceIncludingTo,
	SliceIncludingToX,
	Sort,
	SortX,
	// Split, // [1,2,3] -> 1 2 3
	// SplitAtValue, // TODO
	// SplitAtIndex, // TODO
	// SplitAtFunction, // TODO?
	// Sqrt, // TODO: for ints: ceil/floor?
	Subtract,
	SubtractX,
	Swap,
	SwapUnder, // swap two elements under top
	SwapWithIndex,
	// SwapN - swap with top with nth / n from top
	// SwapNM
	Tail, // everything but first
	TailX,
	Top, // remove everything but top of the stack
}
impl Token {
}
impl From<&str> for Token {
	fn from(token_str: &str) -> Self {
		use Token::*;
		// dbg!(token_str);
		if let Some(_token_str) = token_str.strip_prefix("'") {
			unimplemented!()
			// Literal(StackElement::TokenLiteral(Box::new(Token::from(token_str))))
		}
		else {
			match token_str {
				"abs" => Abs,
				"abs!" => AbsX,
				"add" => Add,
				"add!" => AddX,
				"alleq" => IsAllEqual,
				"alleq!" => IsAllEqualX,
				// "allne" => IsAllNotEqual,
				// "allne!" => IsAllNotEqualX,
				"at" => AtIndex,
				"at!" => AtIndexX,
				"bottom" => Bottom,
				"dec" => Decrease,
				"dec!" => DecreaseX,
				"digits" => Digits,
				"digits!" => DigitsX,
				"divint" => DivideInt,
				"divint!" => DivideIntX,
				"dup" => DuplicateTop,
				// TODO: use `-` to separate words?
				"dupfrom" => DuplicateFromIndex,
				"dupto" => DuplicateToIndex,
				"duptobottom" => DuplicateToBottom,
				"dupunder" => DuplicateUnder,
				"eq" => IsEqual,
				"eq!" => IsEqualX,
				"first" => First,
				"first!" => FirstX,
				"head" => Head,
				"head!" => HeadX,
				"imaxf" => IndexOfMaxFirst,
				"imaxf!" => IndexOfMaxFirstX,
				"imaxl" => IndexOfMaxLast,
				"imaxl!" => IndexOfMaxLastX,
				"iminf" => IndexOfMinFirst,
				"iminf!" => IndexOfMinFirstX,
				"iminl" => IndexOfMinLast,
				"iminl!" => IndexOfMinLastX,
				"inc" => Increase,
				"inc!" => IncreaseX,
				"join" => Join,
				"join!" => JoinX,
				"joindigits" => JoinDigits,
				"joindigits!" => JoinDigitsX,
				"last" => Last,
				"last!" => LastX,
				// "map" => Map,
				"max" => Max,
				"max!" => MaxX,
				"min" => Min,
				"min!" => MinX,
				"mod" => ModuloRemEuclid,
				"mod!" => ModuloRemEuclidX,
				"modf" => ModuloFake,
				"modf!" => ModuloFakeX,
				// TODO: unify/rename with dup! ? (e2dc6d)
				//"move" => Move, // noop
				"movefrom" => MoveFromIndex,
				"moveto" => MoveToIndex,
				"movetobottom" => MoveToBottom,
				"mul" => Multiply,
				"mul!" => MultiplyX,
				"ne" => IsNotEqual,
				"ne!" => IsNotEqualX,
				"neg" => Negate,
				"neg!" => NegateX,
				"not" => Not,
				"not!" => NotX,
				"pop" => Pop,
				"range0excl" => Range0Excluding,
				"range0excl!" => Range0ExcludingX,
				"range0incl" => Range0Including,
				"range0incl!" => Range0IncludingX,
				"range1excl" => Range1Excluding,
				"range1excl!" => Range1ExcludingX,
				"range1incl" => Range1Including,
				"range1incl!" => Range1IncludingX,
				"rev" => Reverse,
				"rev!" => ReverseX,
				"sliceexclexcl" => SliceExcludingExcluding,
				"sliceexclexcl!" => SliceExcludingExcludingX,
				"sliceexclincl" => SliceExcludingIncluding,
				"sliceexclincl!" => SliceExcludingIncludingX,
				"sliceexclfrom" => SliceExcludingFrom,
				"sliceexclfrom!" => SliceExcludingFromX,
				"sliceexclto" => SliceExcludingTo,
				"sliceexclto!" => SliceExcludingToX,
				"sliceinclexcl" => SliceIncludingExcluding,
				"sliceinclexcl!" => SliceIncludingExcludingX,
				"sliceinclincl" => SliceIncludingIncluding,
				"sliceinclincl!" => SliceIncludingIncludingX,
				"sliceinclfrom" => SliceIncludingFrom,
				"sliceinclfrom!" => SliceIncludingFromX,
				"sliceinclto" => SliceIncludingTo,
				"sliceinclto!" => SliceIncludingToX,
				"sort" => Sort,
				"sort!" => SortX,
				"sub" => Subtract,
				"sub!" => SubtractX,
				"swap" => Swap,
				"swapunder" => SwapUnder,
				"swapwith" => SwapWithIndex,
				"tail" => Tail,
				"tail!" => TailX,
				"top" => Top,
				_ => Literal(StackElement::from(token_str))
			}
		}
	}
}





fn exec(program_stack: &mut ProgramStack, token: Token) {
	use StackElement::*;
	use Token::*;
	// TODO(refactor): `let stack = &mut program_stack.stack;`
	// TODO(refactor): let a=...; let b=...; - swap `b` and `a`, to read l->r?
	match token {
		Literal(literal) => {
			program_stack.stack.push(literal);
		}
		// TokenLiteral(_token) => { // TODO: process Literal(Token) somehow?
		// 	// nothing
		// }
		Abs => {
			let top = program_stack.stack.last().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(Int(n.abs()));
				}
				ArrInt(v) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|n| n.abs()).collect()
					));
				}
			}
		}
		AbsX => {
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
			}
		}
		Add => {
			let a = program_stack.stack.last().unwrap();
			let b = &program_stack.stack[program_stack.stack.len()-2];
			match (a, b) {
				(Int(a), Int(b)) => {
					program_stack.stack.push(Int(a + b));
				}
				(Int(n), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|el| el + n).collect()
					));
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					program_stack.stack.push(ArrInt(
						a.iter().zip(b).map(|(a, b)| a + b).collect()
					));
				}
				_ => panic!()
			}
		}
		AddX => {
			let a = program_stack.stack.pop().unwrap();
			let b = program_stack.stack.last_mut().unwrap();
			match (a, b) {
				(Int(a), Int(b)) => {
					*b += a;
				}
				(Int(n), ArrInt(v)) => {
					for el in v {
						*el += n;
					}
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					for (a, b) in a.iter().zip(b) {
						*b += a;
					}
				}
				_ => panic!()
			}
		}
		AtIndex => {
			let a = program_stack.stack.last().unwrap();
			let b = &program_stack.stack[program_stack.stack.len()-2];
			match (a, b) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(Int(v[*i as usize]));
				}
				(ArrInt(i), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(
						i.iter().map(|i| {
							v[*i as usize]
						}).collect()
					));
				}
				_ => panic!()
			}
		}
		AtIndexX => {
			let a = program_stack.stack.pop().unwrap();
			let b = program_stack.stack.pop().unwrap();
			match (a, b) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(Int(v[i as usize]));
				}
				(ArrInt(i), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(
						i.into_iter().map(|i| {
							v[i as usize]
						}).collect()
					));
				}
				_ => panic!()
			}
		}
		Bottom => {
			program_stack.stack = vec![program_stack.stack.first().unwrap().clone()];
		}
		Decrease => {
			let top = program_stack.stack.last().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(Int(n - 1));
				}
				ArrInt(v) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|el| el - 1).collect()
					));
				}
			}
		}
		DecreaseX => {
			let top = program_stack.stack.last_mut().unwrap();
			match top {
				Int(n) => {
					*n -= 1;
				}
				ArrInt(v) => {
					for el in v {
						*el -= 1;
					}
				}
			}
		}
		Digits => {
			let i = program_stack.stack.last().unwrap();
			match i {
				Int(i) => {
					let mut i = *i;
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
		DigitsX => {
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
		DivideInt => {
			let a = program_stack.stack.last().unwrap();
			let b = &program_stack.stack[program_stack.stack.len()-2];
			match (a, b) {
				(Int(a), Int(b)) => {
					program_stack.stack.push(Int(b / a));
				}
				(Int(n), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|el| el / n).collect()
					));
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					program_stack.stack.push(ArrInt(
						a.iter().zip(b).map(|(a, b)| b / a).collect()
					));
				}
				_ => panic!()
			}
		}
		DivideIntX => {
			let a = program_stack.stack.pop().unwrap();
			let b = program_stack.stack.last_mut().unwrap();
			match (a, b) {
				(Int(a), Int(b)) => {
					*b /= a;
				}
				(Int(n), ArrInt(v)) => {
					for el in v {
						*el /= n;
					}
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					for (a, b) in a.iter().zip(b) {
						*b /= a;
					}
				}
				_ => panic!()
			}
		}
		DuplicateFromIndex => {
			let i = program_stack.stack.pop().unwrap();
			match i {
				Int(i) => {
					program_stack.stack.push(
						program_stack.stack[i as usize].clone()
					);
				}
				_ => panic!()
			}
		}
		DuplicateToIndex => {
			let i = program_stack.stack.pop().unwrap();
			match i {
				Int(i) => {
					program_stack.stack.insert(
						i as usize,
						program_stack.stack.last().unwrap().clone()
					);
				}
				_ => panic!()
			}
		}
		DuplicateToBottom => {
			program_stack.stack.insert(
				0,
				program_stack.stack.last().unwrap().clone()
			);
		}
		DuplicateTop => {
			program_stack.stack.push(program_stack.stack.last().unwrap().clone());
		}
		DuplicateUnder => {
			let len = program_stack.stack.len();
			let pretop = program_stack.stack[len-2].clone();
			program_stack.stack.insert(len-1, pretop);
		}
		First => {
			let v = program_stack.stack.last().unwrap();
			match v {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.first().unwrap()));
				}
				_ => panic!()
			}
		}
		FirstX => {
			let v = program_stack.stack.pop().unwrap();
			match v {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.first().unwrap()));
				}
				_ => panic!()
			}
		}
		Increase => {
			let top = program_stack.stack.last().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(Int(n + 1));
				}
				ArrInt(v) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|el| el + 1).collect()
					));
				}
			}
		}
		IncreaseX => {
			let top = program_stack.stack.last_mut().unwrap();
			match top {
				Int(n) => {
					*n += 1;
				}
				ArrInt(v) => {
					for el in v {
						*el += 1;
					}
				}
			}
		}
		IndexOfMaxFirst => {
			let top = program_stack.stack.last().unwrap();
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
		IndexOfMaxFirstX => {
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
			let top = program_stack.stack.last().unwrap();
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
		IndexOfMaxLastX => {
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
			let top = program_stack.stack.last().unwrap();
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
		IndexOfMinFirstX => {
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
			let top = program_stack.stack.last().unwrap();
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
		IndexOfMinLastX => {
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
		IsAllEqual => {
			let v = program_stack.stack.last().unwrap();
			let res = match v {
				ArrInt(v) => {
					v.iter().all(|el| *el == v[0])
				}
				_ => panic!()
			};
			program_stack.stack.push(Int(res as i64));
		}
		IsAllEqualX => {
			let v = program_stack.stack.pop().unwrap();
			let res = match v {
				ArrInt(v) => {
					v.iter().all(|el| *el == v[0])
				}
				_ => panic!()
			};
			program_stack.stack.push(Int(res as i64));
		}
		//IsAllNotEqual => { unimplemented!() }
		//IsAllNotEqualX => { unimplemented!() }
		IsEqual => {
			let a = program_stack.stack.last().unwrap();
			let b = &program_stack.stack[program_stack.stack.len()-2];
			let res = match (a, b) {
				(Int(a), Int(b)) => a == b,
				(ArrInt(a), ArrInt(b)) => a == b,
				_ => unimplemented!()
			};
			program_stack.stack.push(Int(res as i64));
		}
		IsEqualX => {
			let a = program_stack.stack.pop().unwrap();
			let b = program_stack.stack.pop().unwrap();
			let res = match (a, b) {
				(Int(a), Int(b)) => a == b,
				(ArrInt(a), ArrInt(b)) => a == b,
				_ => unimplemented!()
			};
			program_stack.stack.push(Int(res as i64));
		}
		IsNotEqual => {
			let a = program_stack.stack.last().unwrap();
			let b = &program_stack.stack[program_stack.stack.len()-2];
			let res = match (a, b) {
				(Int(a), Int(b)) => {
					a != b
				}
				(ArrInt(a), ArrInt(b)) => {
					a != b
				}
				_ => unimplemented!()
			};
			program_stack.stack.push(Int(res as i64));
		}
		IsNotEqualX => {
			let a = program_stack.stack.pop().unwrap();
			let b = program_stack.stack.pop().unwrap();
			let res = match (a, b) {
				(Int(a), Int(b)) => a != b,
				(ArrInt(a), ArrInt(b)) => a != b,
				_ => unimplemented!()
			};
			program_stack.stack.push(Int(res as i64));
		}
		Join => {
			let top = program_stack.stack.last().unwrap().clone();
			let pretop = program_stack.stack[program_stack.stack.len()-2].clone();
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
			};
			program_stack.stack.push(new_top);
		}
		JoinX => {
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
			};
			program_stack.stack.push(new_top);
		}
		JoinDigits => {
			let v = program_stack.stack.last().unwrap();
			match v {
				ArrInt(v) => {
					let mut n = 0;
					for digit in v {
						n *= 10;
						n += digit;
					}
					program_stack.stack.push(Int(n));
				}
				_ => panic!()
			}
		}
		JoinDigitsX => {
			let v = program_stack.stack.pop().unwrap();
			match v {
				ArrInt(v) => {
					let mut n = 0;
					for digit in v {
						n *= 10;
						n += digit;
					}
					program_stack.stack.push(Int(n));
				}
				_ => panic!()
			}
		}
		Head => {
			let v = program_stack.stack.last().unwrap();
			match v {
				ArrInt(v) => {
					program_stack.stack.push(ArrInt(v[..v.len()-1].to_vec()));
				}
				_ => panic!()
			}
		}
		HeadX => {
			let v = program_stack.stack.last_mut().unwrap();
			match v {
				ArrInt(v) => {
					let _ = v.pop().unwrap();
				}
				_ => panic!()
			}
		}
		Last => {
			let v = program_stack.stack.last().unwrap();
			match v {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.last().unwrap()));
				}
				_ => panic!()
			}
		}
		LastX => {
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
			let top = program_stack.stack.last().unwrap();
			match top {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.iter().max().unwrap()));
				}
				_ => panic!()
			}
		}
		MaxX => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.iter().max().unwrap()));
				}
				_ => panic!()
			}
		}
		Min => {
			let top = program_stack.stack.last().unwrap();
			match top {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.iter().min().unwrap()));
				}
				_ => panic!()
			}
		}
		MinX => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				ArrInt(v) => {
					program_stack.stack.push(Int(*v.iter().min().unwrap()));
				}
				_ => panic!()
			}
		}
		ModuloFake => {
			let a = program_stack.stack.last().unwrap();
			let b = &program_stack.stack[program_stack.stack.len()-2];
			match (a, b) {
				(Int(a), Int(b)) => {
					program_stack.stack.push(Int(b % a));
				}
				(Int(n), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|el| el % n).collect()
					));
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					program_stack.stack.push(ArrInt(
						a.iter().zip(b).map(|(a, b)| b % a).collect()
					));
				}
				_ => panic!()
			}
		}
		ModuloFakeX => {
			let a = program_stack.stack.pop().unwrap();
			let b = program_stack.stack.last_mut().unwrap();
			match (a, b) {
				(Int(a), Int(b)) => {
					*b %= a;
				}
				(Int(n), ArrInt(v)) => {
					for el in v {
						*el %= n;
					}
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					for (a, b) in a.iter().zip(b) {
						*b %= a;
					}
				}
				_ => panic!()
			}
		}
		ModuloRemEuclid => {
			let a = program_stack.stack.last().unwrap();
			let b = &program_stack.stack[program_stack.stack.len()-2];
			match (a, b) {
				(Int(a), Int(b)) => {
					program_stack.stack.push(Int(b.rem_euclid(*a)));
				}
				(Int(n), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|el| el.rem_euclid(*n)).collect()
					));
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					program_stack.stack.push(ArrInt(
						a.iter().zip(b).map(|(a, b)| b.rem_euclid(*a)).collect()
					));
				}
				_ => panic!()
			}
		}
		ModuloRemEuclidX => {
			let a = program_stack.stack.pop().unwrap();
			let b = program_stack.stack.last_mut().unwrap();
			match (a, b) {
				(Int(a), Int(b)) => {
					*b = b.rem_euclid(a);
				}
				(Int(n), ArrInt(v)) => {
					for el in v {
						*el = el.rem_euclid(n);
					}
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					for (a, b) in a.iter().zip(b) {
						*b = b.rem_euclid(*a);
					}
				}
				_ => panic!()
			}
		}
		MoveFromIndex => {
			let i = program_stack.stack.pop().unwrap();
			match i {
				Int(i) => {
					let el = program_stack.stack.remove(i as usize);
					program_stack.stack.push(el);
				}
				_ => panic!()
			}
		}
		MoveToIndex => {
			let i = program_stack.stack.pop().unwrap();
			match i {
				Int(i) => {
					let el = program_stack.stack.pop().unwrap();
					program_stack.stack.insert(i as usize, el);
				}
				_ => panic!()
			}
		}
		MoveToBottom => {
			let top = program_stack.stack.pop().unwrap();
			program_stack.stack.insert(0, top);
		}
		Multiply => {
			let a = program_stack.stack.last().unwrap();
			let b = &program_stack.stack[program_stack.stack.len()-2];
			match (a, b) {
				(Int(a), Int(b)) => {
					program_stack.stack.push(Int(b * a));
				}
				(Int(n), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|el| el * n).collect()
					));
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					program_stack.stack.push(ArrInt(
						a.iter().zip(b).map(|(a, b)| b * a).collect()
					));
				}
				_ => panic!()
			}
		}
		MultiplyX => {
			let a = program_stack.stack.pop().unwrap();
			let b = program_stack.stack.last_mut().unwrap();
			match (a, b) {
				(Int(a), Int(b)) => {
					*b *= a;
				}
				(Int(n), ArrInt(v)) => {
					for el in v {
						*el *= n;
					}
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					for (a, b) in a.iter().zip(b) {
						*b *= a;
					}
				}
				_ => panic!()
			}
		}
		Negate => {
			let top = program_stack.stack.last().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(Int(-n));
				}
				ArrInt(v) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|el| -el).collect()
					));
				}
			}
		}
		NegateX => {
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
			}
		}
		Not => {
			let top = program_stack.stack.last().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(Int(match n {
						0 => 1,
						1 => 0,
						_ => panic!()
					}));
				}
				ArrInt(v) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|el| match el {
							0 => 1,
							1 => 0,
							_ => panic!()
						}).collect()
					));
				}
			}
		}
		NotX => {
			let top = program_stack.stack.last_mut().unwrap();
			match top {
				Int(n) => {
					*n = match n {
						0 => 1,
						1 => 0,
						_ => panic!()
					}
				}
				ArrInt(v) => {
					for el in v {
						*el = match el {
							0 => 1,
							1 => 0,
							_ => panic!()
						}
					}
				}
			}
		}
		Pop => {
			let _ = program_stack.stack.pop().unwrap();
		}
		Range0Excluding => {
			let top = program_stack.stack.last().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (0..*n).collect() ));
				}
				_ => panic!()
			}
		}
		Range0ExcludingX => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (0..n).collect() ));
				}
				_ => panic!()
			}
		}
		Range0Including => {
			let top = program_stack.stack.last().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (0..=*n).collect() ));
				}
				_ => panic!()
			}
		}
		Range0IncludingX => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (0..=n).collect() ));
				}
				_ => panic!()
			}
		}
		Range1Excluding => {
			let top = program_stack.stack.last().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (1..*n).collect() ));
				}
				_ => panic!()
			}
		}
		Range1ExcludingX => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (1..n).collect() ));
				}
				_ => panic!()
			}
		}
		Range1Including => {
			let top = program_stack.stack.last().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (1..=*n).collect() ));
				}
				_ => panic!()
			}
		}
		Range1IncludingX => {
			let top = program_stack.stack.pop().unwrap();
			match top {
				Int(n) => {
					program_stack.stack.push(ArrInt( (1..=n).collect() ));
				}
				_ => panic!()
			}
		}
		Reverse => {
			let top = program_stack.stack.last().unwrap().clone();
			match top {
				ArrInt(mut v) => {
					v.reverse();
					program_stack.stack.push(ArrInt(v));
				}
				_ => panic!()
			}
		}
		ReverseX => {
			let top = program_stack.stack.last_mut().unwrap();
			match top {
				ArrInt(v) => {
					v.reverse();
				}
				_ => panic!()
			}
		}
		SliceExcludingExcluding => {
			let index_to = program_stack.stack.last().unwrap();
			let index_from = &program_stack.stack[program_stack.stack.len()-2];
			let v = &program_stack.stack[program_stack.stack.len()-3];
			match (index_from, index_to, v) {
				(Int(index_from), Int(index_to), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[*index_from as usize + 1 .. *index_to as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceExcludingExcludingX => {
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
			let index_to = program_stack.stack.last().unwrap();
			let index_from = &program_stack.stack[program_stack.stack.len()-2];
			let v = &program_stack.stack[program_stack.stack.len()-3];
			match (index_from, index_to, v) {
				(Int(index_from), Int(index_to), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[*index_from as usize + 1 ..= *index_to as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceExcludingIncludingX => {
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
			let i = program_stack.stack.last().unwrap();
			let v = &program_stack.stack[program_stack.stack.len()-2];
			match (i, v) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[*i as usize + 1 ..].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceExcludingFromX => {
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
			let i = program_stack.stack.last().unwrap();
			let v = &program_stack.stack[program_stack.stack.len()-2];
			match (i, v) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[.. *i as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceExcludingToX => {
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
			let index_to = program_stack.stack.last().unwrap();
			let index_from = &program_stack.stack[program_stack.stack.len()-2];
			let v = &program_stack.stack[program_stack.stack.len()-3];
			match (index_from, index_to, v) {
				(Int(index_from), Int(index_to), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[*index_from as usize .. *index_to as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceIncludingExcludingX => {
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
			let index_to = program_stack.stack.last().unwrap();
			let index_from = &program_stack.stack[program_stack.stack.len()-2];
			let v = &program_stack.stack[program_stack.stack.len()-3];
			match (index_from, index_to, v) {
				(Int(index_from), Int(index_to), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[*index_from as usize ..= *index_to as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceIncludingIncludingX => {
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
			let i = program_stack.stack.last().unwrap();
			let v = &program_stack.stack[program_stack.stack.len()-2];
			match (i, v) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[*i as usize ..].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceIncludingFromX => {
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
			let i = program_stack.stack.last().unwrap();
			let v = &program_stack.stack[program_stack.stack.len()-2];
			match (i, v) {
				(Int(i), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(v[..= *i as usize].to_vec()));
				}
				_ => panic!()
			}
		}
		SliceIncludingToX => {
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
			let top = program_stack.stack.last().unwrap().clone();
			match top {
				ArrInt(mut v) => {
					v.sort();
					program_stack.stack.push(ArrInt(v));
				}
				_ => panic!()
			}
		}
		SortX => {
			let top = program_stack.stack.last_mut().unwrap();
			match top {
				ArrInt(v) => {
					v.sort();
				}
				_ => panic!()
			}
		}
		Subtract => {
			let i = program_stack.stack.last().unwrap();
			let v = &program_stack.stack[program_stack.stack.len()-2];
			match (i, v) {
				(Int(a), Int(b)) => {
					program_stack.stack.push(Int(b - a));
				}
				(Int(n), ArrInt(v)) => {
					program_stack.stack.push(ArrInt(
						v.iter().map(|el| el - n).collect()
					));
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					program_stack.stack.push(ArrInt(
						a.iter().zip(b).map(|(a, b)| b - a).collect()
					));
				}
				_ => panic!()
			}
		}
		SubtractX => {
			let i = program_stack.stack.pop().unwrap();
			let v = program_stack.stack.last_mut().unwrap();
			match (i, v) {
				(Int(a), Int(b)) => {
					*b -= a;
				}
				(Int(n), ArrInt(v)) => {
					for el in v {
						*el -= n;
					}
				}
				// TODO: (ArrInt, Int)
				(ArrInt(a), ArrInt(b)) => {
					assert_eq!(a.len(), b.len());
					for (a, b) in a.iter().zip(b) {
						*b -= a;
					}
				}
				_ => panic!()
			}
		}
		Swap => {
			let len = program_stack.stack.len();
			program_stack.stack.swap(len - 1, len - 2);
		}
		SwapUnder => {
			let len = program_stack.stack.len();
			program_stack.stack.swap(len - 2, len - 3);
		}
		SwapWithIndex => {
			let i = program_stack.stack.pop().unwrap();
			match i {
				Int(i) => {
					let len = program_stack.stack.len();
					program_stack.stack.swap(i as usize, len - 1);
				}
				_ => panic!()
			}
		}
		Tail => {
			let v = program_stack.stack.last().unwrap();
			match v {
				ArrInt(v) => {
					program_stack.stack.push(ArrInt(
						v[1..].to_vec()
					));
				}
				_ => panic!()
			}
		}
		TailX => {
			let v = program_stack.stack.last_mut().unwrap();
			match v {
				ArrInt(v) => {
					let _ = v.remove(0);
				}
				_ => panic!()
			}
		}
		Top => {
			program_stack.stack = vec![program_stack.stack.last().unwrap().clone()];
		}
	}
}





#[cfg(test)]
mod token_exec {
	use super::*;
	use StackElement::*;
	use Token::*;

	mod join_x {
		use super::*;
		#[test]
		fn int_int() {
			assert_eq!(
				ProgramStack::from(ArrInt(vec![1, 2])),
				ProgramStack::from([Int(1), Int(2)]).exec_val(JoinX)
			)
		}
		#[test]
		fn vi_vi() {
			assert_eq!(
				ProgramStack::from(ArrInt(vec![1,2,3,4])),
				ProgramStack::from([ArrInt(vec![1,2]), ArrInt(vec![3,4])]).exec_val(JoinX)
			)
		}
	}

	mod sort_x {
		use super::*;
		#[test]
		fn _0_1_2_3_4_5_6_7_8_9() {
			assert_eq!(
				ProgramStack::from(ArrInt(vec![0,1,2,3,4,5,6,7,8,9])),
				ProgramStack::from(ArrInt(vec![5,9,1,3,4,0,8,7,2,6])).exec_val(SortX)
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
					eval("42 42"),
					eval("42 abs")
				)
			}
			#[test]
			fn int_neg() {
				assert_eq!(
					eval("-42 42"),
					eval("-42 abs")
				)
			}
			#[test]
			fn vi() {
				assert_eq!(
					eval("-1,2,-3 1,2,3"),
					eval("-1,2,-3 abs")
				)
			}
		}
		mod abs_x {
			use super::*;
			#[test]
			fn int_pos() {
				assert_eq!(
					eval("42"),
					eval("42 abs!")
				)
			}
			#[test]
			fn int_neg() {
				assert_eq!(
					eval("42"),
					eval("-42 abs!")
				)
			}
			#[test]
			fn vi() {
				assert_eq!(
					eval("1,2,3"),
					eval("-1,2,-3 abs!")
				)
			}
		}
		mod add {
			use super::*;
			#[test]
			fn _10__1() {
				assert_eq!(
					eval("10 1 11"),
					eval("10 1 add")
				)
			}
			#[test]
			fn _10_20_30__1() {
				assert_eq!(
					eval("10,20,30 1 11,21,31"),
					eval("10,20,30 1 add")
				)
			}
			#[test]
			fn _10_20_30__1_2_3() {
				assert_eq!(
					eval("10,20,30 1,2,3 11,22,33"),
					eval("10,20,30 1,2,3 add")
				)
			}
		}
		mod add_x {
			use super::*;
			#[test]
			fn _10__1() {
				assert_eq!(
					eval("11"),
					eval("10 1 add!")
				)
			}
			#[test]
			fn _10_20_30__1() {
				assert_eq!(
					eval("11,21,31"),
					eval("10,20,30 1 add!")
				)
			}
			#[test]
			fn _10_20_30__1_2_3() {
				assert_eq!(
					eval("11,22,33"),
					eval("10,20,30 1,2,3 add!")
				)
			}
		}
		mod at_index {
			use super::*;
			#[test]
			fn _10_20_30__0() {
				assert_eq!(
					eval("10,20,30 0 10"),
					eval("10,20,30 0 at")
				)
			}
			#[test]
			fn _10_20_30__1() {
				assert_eq!(
					eval("10,20,30 1 20"),
					eval("10,20,30 1 at")
				)
			}
			#[test]
			fn _10_20_30__2() {
				assert_eq!(
					eval("10,20,30 2 30"),
					eval("10,20,30 2 at")
				)
			}
			#[test]
			fn _10_20_30_40_50__1_2_4() {
				assert_eq!(
					eval("10,20,30,40,50 1,2,4 20,30,50"),
					eval("10,20,30,40,50 1,2,4 at")
				)
			}
		}
		mod at_index_x {
			use super::*;
			#[test]
			fn _10_20_30__0() {
				assert_eq!(
					eval("10"),
					eval("10,20,30 0 at!")
				)
			}
			#[test]
			fn _10_20_30__1() {
				assert_eq!(
					eval("20"),
					eval("10,20,30 1 at!")
				)
			}
			#[test]
			fn _10_20_30__2() {
				assert_eq!(
					eval("30"),
					eval("10,20,30 2 at!")
				)
			}
			#[test]
			fn _10_20_30_40_50__1_2_4() {
				assert_eq!(
					eval("20,30,50"),
					eval("10,20,30,40,50 1,2,4 at!")
				)
			}
		}
		mod bottom {
			use super::*;
			#[test]
			fn _1__2__3() {
				assert_eq!(
					eval("1"),
					eval("1 2 3 bottom")
				)
			}
		}
		mod decrease {
			use super::*;
			#[test]
			fn _42() {
				assert_eq!(
					eval("42 41"),
					eval("42 dec")
				)
			}
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("1,2,3 0,1,2"),
					eval("1,2,3 dec")
				)
			}
		}
		mod decrease_x {
			use super::*;
			#[test]
			fn _42() {
				assert_eq!(
					eval("41"),
					eval("42 dec!")
				)
			}
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("0,1,2"),
					eval("1,2,3 dec!")
				)
			}
		}
		mod digits {
			use super::*;
			#[test]
			fn _31415() {
				assert_eq!(
					eval("31415 3,1,4,1,5"),
					eval("31415 digits")
				)
			}
		}
		mod digits_x {
			use super::*;
			#[test]
			fn _31415() {
				assert_eq!(
					eval("3,1,4,1,5"),
					eval("31415 digits!")
				)
			}
		}
		mod divide_int {
			use super::*;
			#[test]
			fn _42__10() {
				assert_eq!(
					eval("42 10 4"),
					eval("42 10 divint")
				)
			}
			#[test]
			fn _47__10() {
				assert_eq!(
					eval("47 10 4"),
					eval("47 10 divint")
				)
			}
			#[test]
			fn _10_20_30__2() {
				assert_eq!(
					eval("10,20,30 2 5,10,15"),
					eval("10,20,30 2 divint")
				)
			}
			#[test]
			fn _10_20_30__2_4_5() {
				assert_eq!(
					eval("10,20,30 2,4,5 5,5,6"),
					eval("10,20,30 2,4,5 divint")
				)
			}
		}
		mod divide_int_x {
			use super::*;
			#[test]
			fn _42__10() {
				assert_eq!(
					eval("4"),
					eval("42 10 divint!")
				)
			}
			#[test]
			fn _47__10() {
				assert_eq!(
					eval("4"),
					eval("47 10 divint!")
				)
			}
			#[test]
			fn _10_20_30__2() {
				assert_eq!(
					eval("5,10,15"),
					eval("10,20,30 2 divint!")
				)
			}
			#[test]
			fn _10_20_30__2_4_5() {
				assert_eq!(
					eval("5,5,6"),
					eval("10,20,30 2,4,5 divint!")
				)
			}
		}
		mod duplicate {
			use super::*;
			mod from {
				use super::*;
				#[test]
				fn _0__1__2__3__4__5() {
					assert_eq!(
						eval("0 1 2 3 4 5 2"),
						eval("0 1 2 3 4 5  2 dupfrom")
					)
				}
			}
			mod to {
				use super::*;
				#[test]
				fn _0__1__2__3__4__5() {
					assert_eq!(
						eval("0 1 5 2 3 4 5"),
						eval("0 1 2 3 4 5  2 dupto")
					)
				}
			}
			mod to_bottom {
				use super::*;
				#[test]
				fn _0__1__2__3__4__5() {
					assert_eq!(
						eval("5 0 1 2 3 4 5"),
						eval("0 1 2 3 4 5 duptobottom")
					)
				}
			}
			mod top {
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
			mod under {
				use super::*;
				#[test]
				fn _0__1__2__3__4__5() {
					assert_eq!(
						eval("0 1 2 3 4 4 5"),
						eval("0 1 2 3 4 5 dupunder")
					)
				}
			}
		}
		mod first {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("1,2,3 1"),
					eval("1,2,3 first")
				)
			}
		}
		mod first_x {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("1"),
					eval("1,2,3 first!")
				)
			}
		}
		mod increase {
			use super::*;
			#[test]
			fn _42() {
				assert_eq!(
					eval("42 43"),
					eval("42 inc")
				)
			}
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("1,2,3 2,3,4"),
					eval("1,2,3 inc")
				)
			}
		}
		mod increase_x {
			use super::*;
			#[test]
			fn _42() {
				assert_eq!(
					eval("43"),
					eval("42 inc!")
				)
			}
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("2,3,4"),
					eval("1,2,3 inc!")
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
							eval("5,9,1,3,4,0,8,7,2,6 1"),
							eval("5,9,1,3,4,0,8,7,2,6 imaxf")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("5,9,1,0,3,4,0,8,9,7,2,6 1"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 imaxf")
						)
					}
				}
				mod last {
					use super::*;
					#[test]
					fn _5_9_1_3_4_0_8_7_2_6() {
						assert_eq!(
							eval("5,9,1,3,4,0,8,7,2,6 1"),
							eval("5,9,1,3,4,0,8,7,2,6 imaxl")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("5,9,1,0,3,4,0,8,9,7,2,6 8"),
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
							eval("5,9,1,3,4,0,8,7,2,6 5"),
							eval("5,9,1,3,4,0,8,7,2,6 iminf")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("5,9,1,0,3,4,0,8,9,7,2,6 3"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 iminf")
						)
					}
				}
				mod last {
					use super::*;
					#[test]
					fn _5_9_1_3_4_0_8_7_2_6() {
						assert_eq!(
							eval("5,9,1,3,4,0,8,7,2,6 5"),
							eval("5,9,1,3,4,0,8,7,2,6 iminl")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("5,9,1,0,3,4,0,8,9,7,2,6 6"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 iminl")
						)
					}
				}
			}
		}
		mod index_of_x {
			use super::*;
			mod max {
				use super::*;
				mod first {
					use super::*;
					#[test]
					fn _5_9_1_3_4_0_8_7_2_6() {
						assert_eq!(
							eval("1"),
							eval("5,9,1,3,4,0,8,7,2,6 imaxf!")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("1"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 imaxf!")
						)
					}
				}
				mod last {
					use super::*;
					#[test]
					fn _5_9_1_3_4_0_8_7_2_6() {
						assert_eq!(
							eval("1"),
							eval("5,9,1,3,4,0,8,7,2,6 imaxl!")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("8"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 imaxl!")
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
							eval("5,9,1,3,4,0,8,7,2,6 iminf!")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("3"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 iminf!")
						)
					}
				}
				mod last {
					use super::*;
					#[test]
					fn _5_9_1_3_4_0_8_7_2_6() {
						assert_eq!(
							eval("5"),
							eval("5,9,1,3,4,0,8,7,2,6 iminl!")
						)
					}
					#[test]
					fn _5_9_1_0_3_4_0_8_9_7_2_6() {
						assert_eq!(
							eval("6"),
							eval("5,9,1,0,3,4,0,8,9,7,2,6 iminl!")
						)
					}
				}
			}
		}
		mod is {
			use super::*;
			mod all {
				use super::*;
				mod equal {
					use super::*;
					#[test]
					fn _1_2_3() {
						assert_eq!(
							eval("1,2,3 0"),
							eval("1,2,3 alleq"),
						)
					}
					#[test]
					fn _2_2_2() {
						assert_eq!(
							eval("2,2,2 1"),
							eval("2,2,2 alleq"),
						)
					}
				}
				// mod not_equal {
				// 	use super::*;
				// 	#[test]
				// 	fn _1_2_3() {
				// 		assert_eq!(
				// 			eval("1,2,3 1"),
				// 			eval("1,2,3 allne"),
				// 		)
				// 	}
				// 	#[test]
				// 	fn _2_2_2() {
				// 		assert_eq!(
				// 			eval("2,2,2 0"),
				// 			eval("2,2,2 allne"),
				// 		)
				// 	}
				// }
			}
			mod two {
				use super::*;
				mod equal {
					use super::*;
					#[test]
					fn _1__2() {
						assert_eq!(
							eval("1 2 0"),
							eval("1 2 eq"),
						)
					}
					#[test]
					fn _2__2() {
						assert_eq!(
							eval("2 2 1"),
							eval("2 2 eq"),
						)
					}
					#[test]
					fn _1_2__1_2() {
						assert_eq!(
							eval("1,2 1,2 1"),
							eval("1,2 1,2 eq")
						)
					}
					#[test]
					fn _1_2__3_4() {
						assert_eq!(
							eval("1,2 3,4 0"),
							eval("1,2 3,4 eq")
						)
					}
				}
				mod not_equal {
					use super::*;
					#[test]
					fn _1__2() {
						assert_eq!(
							eval("1 2 1"),
							eval("1 2 ne"),
						)
					}
					#[test]
					fn _2__2() {
						assert_eq!(
							eval("2 2 0"),
							eval("2 2 ne"),
						)
					}
					#[test]
					fn _1_2__1_2() {
						assert_eq!(
							eval("1,2 1,2 0"),
							eval("1,2 1,2 ne")
						)
					}
					#[test]
					fn _1_2__3_4() {
						assert_eq!(
							eval("1,2 3,4 1"),
							eval("1,2 3,4 ne")
						)
					}
				}
			}
		}
		mod is_x {
			use super::*;
			mod all {
				use super::*;
				mod equal {
					use super::*;
					#[test]
					fn _1_2_3() {
						assert_eq!(
							eval("0"),
							eval("1,2,3 alleq!"),
						)
					}
					#[test]
					fn _2_2_2() {
						assert_eq!(
							eval("1"),
							eval("2,2,2 alleq!"),
						)
					}
				}
				// mod not_equal {
				// 	use super::*;
				// 	#[test]
				// 	fn _1_2_3() {
				// 		assert_eq!(
				// 			eval("1"),
				// 			eval("1,2,3 allne!"),
				// 		)
				// 	}
				// 	#[test]
				// 	fn _2_2_2() {
				// 		assert_eq!(
				// 			eval("0"),
				// 			eval("2,2,2 allne!"),
				// 		)
				// 	}
				// }
			}
			mod two {
				use super::*;
				mod equal {
					use super::*;
					#[test]
					fn _1__2() {
						assert_eq!(
							eval("0"),
							eval("1 2 eq!"),
						)
					}
					#[test]
					fn _2__2() {
						assert_eq!(
							eval("1"),
							eval("2 2 eq!"),
						)
					}
					#[test]
					fn _1_2__1_2() {
						assert_eq!(
							eval("1"),
							eval("1,2 1,2 eq!")
						)
					}
					#[test]
					fn _1_2__3_4() {
						assert_eq!(
							eval("0"),
							eval("1,2 3,4 eq!")
						)
					}
				}
				mod not_equal {
					use super::*;
					#[test]
					fn _1__2() {
						assert_eq!(
							eval("1"),
							eval("1 2 ne!"),
						)
					}
					#[test]
					fn _2__2() {
						assert_eq!(
							eval("0"),
							eval("2 2 ne!"),
						)
					}
					#[test]
					fn _1_2__1_2() {
						assert_eq!(
							eval("0"),
							eval("1,2 1,2 ne!")
						)
					}
					#[test]
					fn _1_2__3_4() {
						assert_eq!(
							eval("1"),
							eval("1,2 3,4 ne!")
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
					eval("1 2 1,2"),
					eval("1 2 join")
				)
			}
			#[test]
			fn vi_vi() {
				assert_eq!(
					eval("1,2 3,4 1,2,3,4"),
					eval("1,2 3,4 join")
				)
			}
			#[test]
			fn vi_int() {
				assert_eq!(
					eval("1,2 3 1,2,3"),
					eval("1,2 3 join")
				)
			}
			#[test]
			fn int_vi() {
				assert_eq!(
					eval("1 2,3 1,2,3"),
					eval("1 2,3 join")
				)
			}
		}
		mod join_x {
			use super::*;
			#[test]
			fn int_int() {
				assert_eq!(
					eval("1,2"),
					eval("1 2 join!")
				)
			}
			#[test]
			fn vi_vi() {
				assert_eq!(
					eval("1,2,3,4"),
					eval("1,2 3,4 join!")
				)
			}
			#[test]
			fn vi_int() {
				assert_eq!(
					eval("1,2,3"),
					eval("1,2 3 join!")
				)
			}
			#[test]
			fn int_vi() {
				assert_eq!(
					eval("1,2,3"),
					eval("1 2,3 join!")
				)
			}
		}
		mod join_digits {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("1,2,3 123"),
					eval("1,2,3 joindigits")
				)
			}
		}
		mod join_digits_x {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("123"),
					eval("1,2,3 joindigits!")
				)
			}
		}
		mod head {
			use super::*;
			#[test]
			fn _4_5_6_7() {
				assert_eq!(
					eval("4,5,6,7 4,5,6"),
					eval("4,5,6,7 head")
				)
			}
		}
		mod head_x {
			use super::*;
			#[test]
			fn _4_5_6_7() {
				assert_eq!(
					eval("4,5,6"),
					eval("4,5,6,7 head!")
				)
			}
		}
		mod last {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("1,2,3 3"),
					eval("1,2,3 last")
				)
			}
		}
		mod last_x {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("3"),
					eval("1,2,3 last!")
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
					eval("1,2,3 3"),
					eval("1,2,3 max")
				)
			}
		}
		mod max_x {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("3"),
					eval("1,2,3 max!")
				)
			}
		}
		mod min {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("1,2,3 1"),
					eval("1,2,3 min")
				)
			}
		}
		mod min_x {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("1"),
					eval("1,2,3 min!")
				)
			}
		}
		mod modulo {
			use super::*;
			mod fake {
				use super::*;
				#[test]
				fn _42__5() {
					assert_eq!(
						eval("42 5 2"),
						eval("42 5 modf")
					)
				}
				#[test]
				fn _m42__5() {
					assert_eq!(
						eval("-42 5 -2"),
						eval("-42 5 modf")
					)
				}
				#[test]
				fn _10_20_30__7() {
					assert_eq!(
						eval("10,20,30 7 3,6,2"),
						eval("10,20,30 7 modf")
					)
				}
				#[test]
				fn _m10_m20_m30__7() {
					assert_eq!(
						eval("-10,-20,-30 7 -3,-6,-2"),
						eval("-10,-20,-30 7 modf")
					)
				}
				#[test]
				fn _10_20_30__6_7_8() {
					assert_eq!(
						eval("10,20,30 6,7,8 4,6,6"),
						eval("10,20,30 6,7,8 modf")
					)
				}
				#[test]
				fn _m10_m20_m30__6_7_8() {
					assert_eq!(
						eval("-10,-20,-30 6,7,8 -4,-6,-6"),
						eval("-10,-20,-30 6,7,8 modf")
					)
				}
			}
			mod rem_euclid {
				use super::*;
				#[test]
				fn _42__5() {
					assert_eq!(
						eval("42 5 2"),
						eval("42 5 mod")
					)
				}
				#[test]
				fn _m42__5() {
					assert_eq!(
						eval("-42 5 3"),
						eval("-42 5 mod")
					)
				}
				#[test]
				fn _10_20_30__7() {
					assert_eq!(
						eval("10,20,30 7 3,6,2"),
						eval("10,20,30 7 mod")
					)
				}
				#[test]
				fn _m10_m20_m30__7() {
					assert_eq!(
						eval("-10,-20,-30 7 4,1,5"),
						eval("-10,-20,-30 7 mod")
					)
				}
				#[test]
				fn _10_20_30__6_7_8() {
					assert_eq!(
						eval("10,20,30 6,7,8 4,6,6"),
						eval("10,20,30 6,7,8 mod")
					)
				}
				#[test]
				fn _m10_m20_m30__6_7_8() {
					assert_eq!(
						eval("-10,-20,-30 6,7,8 2,1,2"),
						eval("-10,-20,-30 6,7,8 mod")
					)
				}
			}
		}
		mod modulo_x {
			use super::*;
			mod fake {
				use super::*;
				#[test]
				fn _42__5() {
					assert_eq!(
						eval("2"),
						eval("42 5 modf!")
					)
				}
				#[test]
				fn _m42__5() {
					assert_eq!(
						eval("-2"),
						eval("-42 5 modf!")
					)
				}
				#[test]
				fn _10_20_30__7() {
					assert_eq!(
						eval("3,6,2"),
						eval("10,20,30 7 modf!")
					)
				}
				#[test]
				fn _m10_m20_m30__7() {
					assert_eq!(
						eval("-3,-6,-2"),
						eval("-10,-20,-30 7 modf!")
					)
				}
				#[test]
				fn _10_20_30__6_7_8() {
					assert_eq!(
						eval("4,6,6"),
						eval("10,20,30 6,7,8 modf!")
					)
				}
				#[test]
				fn _m10_m20_m30__6_7_8() {
					assert_eq!(
						eval("-4,-6,-6"),
						eval("-10,-20,-30 6,7,8 modf!")
					)
				}
			}
			mod rem_euclid {
				use super::*;
				#[test]
				fn _42__5() {
					assert_eq!(
						eval("2"),
						eval("42 5 mod!")
					)
				}
				#[test]
				fn _m42__5() {
					assert_eq!(
						eval("3"),
						eval("-42 5 mod!")
					)
				}
				#[test]
				fn _10_20_30__7() {
					assert_eq!(
						eval("3,6,2"),
						eval("10,20,30 7 mod!")
					)
				}
				#[test]
				fn _m10_m20_m30__7() {
					assert_eq!(
						eval("4,1,5"),
						eval("-10,-20,-30 7 mod!")
					)
				}
				#[test]
				fn _10_20_30__6_7_8() {
					assert_eq!(
						eval("4,6,6"),
						eval("10,20,30 6,7,8 mod!")
					)
				}
				#[test]
				fn _m10_m20_m30__6_7_8() {
					assert_eq!(
						eval("2,1,2"),
						eval("-10,-20,-30 6,7,8 mod!")
					)
				}
			}
		}
		mod move_ {
			use super::*;
			mod from {
				use super::*;
				#[test]
				fn _0__1__2__3__4__5() {
					assert_eq!(
						eval("0 1 3 4 5 2"),
						eval("0 1 2 3 4 5  2 movefrom")
					)
				}
			}
			mod to {
				use super::*;
				#[test]
				fn _0__1__2__3__4__5() {
					assert_eq!(
						eval("0 1 5 2 3 4"),
						eval("0 1 2 3 4 5  2 moveto")
					)
				}
			}
			mod to_bottom {
				use super::*;
				#[test]
				fn _1__2__3() {
					assert_eq!(
						eval("3 1 2"),
						eval("1 2 3 movetobottom")
					)
				}
			}
		}
		mod multiply {
			use super::*;
			#[test]
			fn _3_4() {
				assert_eq!(
					eval("3 4 12"),
					eval("3 4 mul")
				)
			}
			#[test]
			fn _1_2_3__10() {
				assert_eq!(
					eval("1,2,3 10 10,20,30"),
					eval("1,2,3 10 mul")
				)
			}
			#[test]
			fn _1_2_3__4_5_6() {
				assert_eq!(
					eval("1,2,3 4,5,6 4,10,18"),
					eval("1,2,3 4,5,6 mul")
				)
			}
		}
		mod multiply_x {
			use super::*;
			#[test]
			fn _3_4() {
				assert_eq!(
					eval("12"),
					eval("3 4 mul!")
				)
			}
			#[test]
			fn _1_2_3__10() {
				assert_eq!(
					eval("10,20,30"),
					eval("1,2,3 10 mul!")
				)
			}
			#[test]
			fn _1_2_3__4_5_6() {
				assert_eq!(
					eval("4,10,18"),
					eval("1,2,3 4,5,6 mul!")
				)
			}
		}
		mod negate {
			use super::*;
			#[test]
			fn int() {
				assert_eq!(
					eval("42 -42"),
					eval("42 neg")
				)
			}
			#[test]
			fn vi() {
				assert_eq!(
					eval("1,2,3 -1,-2,-3"),
					eval("1,2,3 neg")
				)
			}
		}
		mod negate_x {
			use super::*;
			#[test]
			fn int() {
				assert_eq!(
					eval("-42"),
					eval("42 neg!")
				)
			}
			#[test]
			fn vi() {
				assert_eq!(
					eval("-1,-2,-3"),
					eval("1,2,3 neg!")
				)
			}
		}
		mod not {
			use super::*;
			#[test]
			fn _0() {
				assert_eq!(
					eval("0 1"),
					eval("0 not"),
				)
			}
			#[test]
			fn _1() {
				assert_eq!(
					eval("1 0"),
					eval("1 not"),
				)
			}
			#[test]
			fn _1_0_1() {
				assert_eq!(
					eval("1,0,1 0,1,0"),
					eval("1,0,1 not")
				)
			}
		}
		mod not_x {
			use super::*;
			#[test]
			fn _0() {
				assert_eq!(
					eval("1"),
					eval("0 not!"),
				)
			}
			#[test]
			fn _1() {
				assert_eq!(
					eval("0"),
					eval("1 not!"),
				)
			}
			#[test]
			fn _1_0_1() {
				assert_eq!(
					eval("0,1,0"),
					eval("1,0,1 not!")
				)
			}
		}
		mod pop {
			use super::*;
			#[test]
			fn _1__2__3() {
				assert_eq!(
					eval("1 2"),
					eval("1 2 3 pop")
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
							eval("5 0,1,2,3,4"),
							eval("5 range0excl")
						)
					}
				}
				mod including {
					use super::*;
					#[test]
					fn _5() {
						assert_eq!(
							eval("5 0,1,2,3,4,5"),
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
							eval("5 1,2,3,4"),
							eval("5 range1excl")
						)
					}
				}
				mod including {
					use super::*;
					#[test]
					fn _5() {
						assert_eq!(
							eval("5 1,2,3,4,5"),
							eval("5 range1incl")
						)
					}
				}
			}
		}
		mod range_x {
			use super::*;
			mod _0 {
				use super::*;
				mod excluding {
					use super::*;
					#[test]
					fn _5() {
						assert_eq!(
							eval("0,1,2,3,4"),
							eval("5 range0excl!")
						)
					}
				}
				mod including {
					use super::*;
					#[test]
					fn _5() {
						assert_eq!(
							eval("0,1,2,3,4,5"),
							eval("5 range0incl!")
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
							eval("5 range1excl!")
						)
					}
				}
				mod including {
					use super::*;
					#[test]
					fn _5() {
						assert_eq!(
							eval("1,2,3,4,5"),
							eval("5 range1incl!")
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
					eval("1,2,3 3,2,1"),
					eval("1,2,3 rev")
				)
			}
		}
		mod reverse_x {
			use super::*;
			#[test]
			fn _1_2_3() {
				assert_eq!(
					eval("3,2,1"),
					eval("1,2,3 rev!")
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
							eval("0,1,2,3,4,5,6,7,8,9 3 4,5,6,7,8,9"),
							eval("0,1,2,3,4,5,6,7,8,9 3 sliceexclfrom")
						)
					}
				}
				mod to {
					use super::*;
					#[test]
					fn _0_1_2_3_4_5_6_7_8_9() {
						assert_eq!(
							eval("0,1,2,3,4,5,6,7,8,9 3 0,1,2"),
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
								eval("0,1,2,3,4,5,6,7,8,9 3 6 4,5"),
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceexclexcl")
							)
						}
					}
					mod including {
						use super::*;
						#[test]
						fn _0_1_2_3_4_5_6_7_8_9() {
							assert_eq!(
								eval("0,1,2,3,4,5,6,7,8,9 3 6 4,5,6"),
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
							eval("0,1,2,3,4,5,6,7,8,9 3 3,4,5,6,7,8,9"),
							eval("0,1,2,3,4,5,6,7,8,9 3 sliceinclfrom")
						)
					}
				}
				mod to {
					use super::*;
					#[test]
					fn _0_1_2_3_4_5_6_7_8_9() {
						assert_eq!(
							eval("0,1,2,3,4,5,6,7,8,9 3 0,1,2,3"),
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
								eval("0,1,2,3,4,5,6,7,8,9 3 6 3,4,5"),
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceinclexcl")
							)
						}
					}
					mod including {
						use super::*;
						#[test]
						fn _0_1_2_3_4_5_6_7_8_9() {
							assert_eq!(
								eval("0,1,2,3,4,5,6,7,8,9 3 6 3,4,5,6"),
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceinclincl")
							)
						}
					}
				}
			}
		}
		mod slice_x {
			use super::*;
			mod excluding {
				use super::*;
				mod from {
					use super::*;
					#[test]
					fn _0_1_2_3_4_5_6_7_8_9() {
						assert_eq!(
							eval("4,5,6,7,8,9"),
							eval("0,1,2,3,4,5,6,7,8,9 3 sliceexclfrom!")
						)
					}
				}
				mod to {
					use super::*;
					#[test]
					fn _0_1_2_3_4_5_6_7_8_9() {
						assert_eq!(
							eval("0,1,2"),
							eval("0,1,2,3,4,5,6,7,8,9 3 sliceexclto!")
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
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceexclexcl!")
							)
						}
					}
					mod including {
						use super::*;
						#[test]
						fn _0_1_2_3_4_5_6_7_8_9() {
							assert_eq!(
								eval("4,5,6"),
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceexclincl!")
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
							eval("0,1,2,3,4,5,6,7,8,9 3 sliceinclfrom!")
						)
					}
				}
				mod to {
					use super::*;
					#[test]
					fn _0_1_2_3_4_5_6_7_8_9() {
						assert_eq!(
							eval("0,1,2,3"),
							eval("0,1,2,3,4,5,6,7,8,9 3 sliceinclto!")
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
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceinclexcl!")
							)
						}
					}
					mod including {
						use super::*;
						#[test]
						fn _0_1_2_3_4_5_6_7_8_9() {
							assert_eq!(
								eval("3,4,5,6"),
								eval("0,1,2,3,4,5,6,7,8,9 3 6 sliceinclincl!")
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
					eval("5,9,1,3,4,0,8,7,2,6 0,1,2,3,4,5,6,7,8,9"),
					eval("5,9,1,3,4,0,8,7,2,6 sort")
				)
			}
		}
		mod sort_x {
			use super::*;
			#[test]
			fn _0_1_2_3_4_5_6_7_8_9() {
				assert_eq!(
					eval("0,1,2,3,4,5,6,7,8,9"),
					eval("5,9,1,3,4,0,8,7,2,6 sort!")
				)
			}
		}
		mod subtract {
			use super::*;
			#[test]
			fn _10__1() {
				assert_eq!(
					eval("10 1 9"),
					eval("10 1 sub")
				)
			}
			#[test]
			fn _10_20_30__1() {
				assert_eq!(
					eval("10,20,30 1 9,19,29"),
					eval("10,20,30 1 sub")
				)
			}
			#[test]
			fn _10_20_30__1_2_3() {
				assert_eq!(
					eval("10,20,30 1,2,3 9,18,27"),
					eval("10,20,30 1,2,3 sub")
				)
			}
		}
		mod subtract_x {
			use super::*;
			#[test]
			fn _10__1() {
				assert_eq!(
					eval("9"),
					eval("10 1 sub!")
				)
			}
			#[test]
			fn _10_20_30__1() {
				assert_eq!(
					eval("9,19,29"),
					eval("10,20,30 1 sub!")
				)
			}
			#[test]
			fn _10_20_30__1_2_3() {
				assert_eq!(
					eval("9,18,27"),
					eval("10,20,30 1,2,3 sub!")
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
		mod swap_under {
			use super::*;
			#[test]
			fn _1__2__3() {
				assert_eq!(
					eval("2 1 3"),
					eval("1 2 3 swapunder")
				)
			}
		}
		mod swap_with_index {
			use super::*;
			#[test]
			fn _0__1__2__3__4__5() {
				assert_eq!(
					eval("0 1 5 3 4 2"),
					eval("0 1 2 3 4 5  2 swapwith")
				)
			}
		}
		mod tail {
			use super::*;
			#[test]
			fn _4_5_6_7() {
				assert_eq!(
					eval("4,5,6,7 5,6,7"),
					eval("4,5,6,7 tail")
				)
			}
		}
		mod tail_x {
			use super::*;
			#[test]
			fn _4_5_6_7() {
				assert_eq!(
					eval("5,6,7"),
					eval("4,5,6,7 tail!")
				)
			}
		}
		mod top {
			use super::*;
			#[test]
			fn _1__2__3() {
				assert_eq!(
					eval("3"),
					eval("1 2 3 top")
				)
			}
		}
	}
}

