#![allow(unstable)] // allow unstable libraries

extern crate rand;
extern crate getopts;
extern crate num_traits;
extern crate regex;

use rand::Rng;
use num_traits::pow;
use getopts::Options;
use std::env; // use parameters
use std::io::{self, BufRead}; 
use regex::Regex;

// programm for password generation

// cargo run -- -l 10 -n 2 -s 50
// cat testword | cargo run -- -v


// To-Do
	// variable for strength of password
	// points for length of pw
	// malus for same character, more often -> higher malus
	// no special character -> malus
	// same case -> malus
	// length, special character, different characters, non redundance
	// savety check of password
	// password strenght by user input



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

// fill vector with special character only
fn spcl () -> Vec<char> {
	let mut special: Vec<char> = vec![];
	// fill vector with special character
	for i in 33u8..47u8 {
		special.push(i as char)
	}
	for i in 58u8..64u8 {
		special.push(i as char)		
	}
	for i in 91u8..96u8 {
		special.push(i as char)		
	}
	for i in 123u8..126u8 {
		special.push(i as char)
	}
	return special
}

// creation of passward with certain length
fn pw (pass_length: &usize, signs: &Vec<char>) -> String {
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

// create n passwords
fn muPW (number: &i32, pass_length: &usize, signs: &Vec<char>) {
	println!("Printing {} passwords of lengths {}", number, pass_length);
	for x in 0..*number {
		// create password
		let password = pw(&pass_length, &signs);
				
		// we have a password
		println!("{}", password);
	}
}


// main function
fn main() {
	// variables
	let mut pass_length: usize = 8; // length of password
	let mut number: i32 = 1; // set number of created passwords
	let mut limit = 20; // set score threshold for password
	let mut score: f64 = 0.0; // score for password quality
	
	// parameters
	//let args: Vec<String> = std::env::args().skip(1).collect(); // skip() -> skip programm call
	let args: Vec<String> = std::env::args().collect();
	let program = args[0].clone();
	
	// create new option object
	let mut opts = Options::new();
	opts.optopt("l", "length", "set length of password [default: 8]", "LENGTH");
	opts.optopt("n", "number", "set number of created passwords [default: 1]", "NUMBER");
	opts.optopt("s", "score", "set score of password", "SCORE");
	opts.optflag("v", "verify", "verification"); // verification of existing password
	opts.optflag("h", "help", "print this help menu");
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m }
		Err(f) => { panic!(f.to_string()) }
	};
	
	// PARAMETERS
	// print help menu??
	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}

	// only verification of existing password from STDIN
	if matches.opt_present("v") {
		let input = io::stdin(); // read password from stdin
		let mut password = input.lock().lines().next().unwrap().unwrap();
		pass_length = password.chars().count(); // get password length
		// pass_length usize -> f64
		let tmp: f64 = pass_length.clone() as f64;
		score += tmp;
		//println!("{} with length {} this get a score of {}", password, pass_length, score);
		let mut control: char = password.chars().next().unwrap(); // get first element of string
		//let test: char = '5';
		//println!("first {}", control);

		let mut special: Vec<char> = spcl(); // fill vector with special character
		let mut lowcase = 0; // counter for lowercase
		let mut upcase = 0; // counter for upptercase
		let uprex = Regex::new(r"[A-Z]").unwrap(); // regex uppercase
		let lowex = Regex::new(r"[a-z]").unwrap();// regex lowercase
		//assert!(uprex.is_match("A")); // testing regex
		//assert!(lowex.is_match("b")); 
		/*for i in &special {
			println!("{}", i);
		}*/
		
		let mut dup: f64 = 0.0; // number of same character in row
		// verification of password
		for c in password.chars() {
			
			// same character more times in a row? 
			if c == control { // c already used?
				//println!("{} is already in use -> {}", c, control);
				//println!("OUR counter is {}", dup);
				dup += 1.0;
			} else {
				//println!("{} is new, not {}", c, control);
				score -= 2f64.powf(dup - 1.0); // power of: pow(base, exponent) -> for score
				//println!("SCORE {}", score);
				dup = 0.0;
			}

			// different cases
			let cstr = c.to_string(); // convert <char> into <String>
			if uprex.is_match(&cstr) {
				//println!("uppercase letter {}", c);
				upcase = 1;
			} else if lowex.is_match(&cstr) {
				//println!("lowercase letter {}", c);
				lowcase = 1;				
			}
			
			// special character??
			
			//println!("{} vs {}", control, c);
			control = c.clone();
		}
		
		// different cases?
		if lowcase == 1 && upcase == 1 {
			score += 4.0; // increase score
		}
		println!("password {} has a score of {}!", password, score);
	} else {
		pass_length = matches.opt_str("l").unwrap().parse::<usize>().unwrap(); // unwrap Options<String> from opt_str -> convert into int // unwrap_or(String::from())
		number = matches.opt_str("n").unwrap().parse::<i32>().unwrap(); 
		score = matches.opt_str("s").unwrap().parse::<f64>().unwrap();
		//println!("Password length: {}, Number of passwords: {}, Score of Password: {}", pass_length, number, score);	
		let mut  signs : Vec<char> = filler(); // assign array with (ascii) signs
		muPW (&number, &pass_length, &signs); // create n passwords
	}
}
