#![allow(unstable)] // allow unstable libraries

// programm for password generation

// cargo run -- -l 10 -n 2 -s 50


// To-Do
	// variable for strength of password
	// points for length of pw
	// malus for same character, more often -> higher malus
	// no special character -> malus
	// same case -> malus
	// length, special character, different characters, non redundance
	// savety check of password
	// password strenght by user input



extern crate rand;
extern crate getopts;
use rand::Rng;
use getopts::Options;
use std::env; // use parameters


// functions

// print help menu
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

// create vector containing signs for password
fn filler () -> Vec<char> {
	let mut signs : Vec<char> = vec![]; // assign empty vector
	// fill vector
	for i in 33u8..126u8 {
		match i as char { // filtering I, l
			'l'		=> continue,
			'I'		=> continue,
			'O'		=> continue,
			'°'		=> continue,
			'²'		=> continue,
			'³'		=> continue,
			'€'		=> continue,
			_	=> signs.push(i as char)
		}
	}
	return signs
}

// creation of passward with certain length
fn pw (pass_length: &i32, signs: &Vec<char>) -> String {
	let mut password = String::new(); // assign password as empty string
	let mut rng = rand::thread_rng(); // instance of thread; rng = random number generator (object)
	for _ in 0..*pass_length {
		//password.push(rng.choose(&signs).unwrap()); // get only password
		let w = (*(rng.choose(&signs).unwrap()));
		//password.push(*(rng.choose(&signs).unwrap())); // * for derefernces for processing
		password.push(w);
	}
	return password
}


// main function
fn main() {
	// variables
	let mut pass_length: i32 = 8; // length of password
	let mut number: i32 = 1; // set number of created passwords
	let mut limit = 20; // set score threshold for password
	let mut score = 0; // score for password quality
	
	// parameters
	//let args: Vec<String> = std::env::args().skip(1).collect(); // skip() -> skip programm call
	let args: Vec<String> = std::env::args().collect();
	let program = args[0].clone();
	
	// create new option object
	let mut opts = Options::new();
	opts.optopt("l", "length", "set length of password [default: 8]", "LENGTH");
	opts.optopt("n", "number", "set number of created passwords [default: 1]", "NUMBER");
	opts.optopt("s", "score", "set score of password", "SCORE");
	opts.optflag("h", "help", "print this help menu");
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m }
		Err(f) => { panic!(f.to_string()) }
	};
	
	// print help menu??
	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}
	pass_length = matches.opt_str("l").unwrap().parse::<i32>().unwrap(); // unwrap Options<String> from opt_str -> convert into int // unwrap_or(String::from())
	number = matches.opt_str("n").unwrap().parse::<i32>().unwrap(); 
	score = matches.opt_str("s").unwrap().parse::<i32>().unwrap();
	println!("Password length: {}, Number of passwords: {}, Score of Password: {}", pass_length, number, score);		
	score += pass_length;
	
	// assign array with (ascii) signs
	let mut  signs : Vec<char> = filler();

	// create n passwords
	for x in 0..number {
		// create password
		let password = pw(&pass_length, &signs);
		
		//let mut control = String::new();
		let mut control: char = password.chars().next().unwrap(); // get first element of string
		println!("first {}", control);

		for c in password.chars() {
			
			// c already used?
			match c {
				control => println!("{} is already in use -> {}", c, control),
				_ => println!("{} is new, not {}", c, control),
			}
			
			// same character more times in a row?
			
			// different cases
			
			println!("{} vs {}", control, c);
			//control.push(c.clone());
			control = c;
		}
		
		// we have a password
		println!("password is: {}", password);
	}
}
