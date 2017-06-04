#![allow(unstable)] // allow unstable libraries


// programm for password generation


// To-Do:
// parameters for password options
// Strength of password (number from parameter)
// not to complicated character  (| l I ö 0) 


// crates
extern crate rand;
extern crate getopts;


use rand::Rng;
// use std::__rand::thread_rng;
use getopts::Options;
use std::string::String;
use std::str::FromStr;
use std::env; // use command line arguments


// main function
fn main() { 
	// get parameters
	let args: Vec<String> = env::args().collect();
	// let program = args[0].clone(); // name of program inside parameters

	let mut opts = Options::new(); // create new options objetct
    opts.optopt("l", "", "set length of password", "LENGTH");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m } // match
        Err(f) => { panic!(f.to_string()) }
    };


	// variable for length of password
	let mut pass_length: i32 = 8;
	let mut password = String::new(); // assign password as empty string
	let mut rng = rand::thread_rng(); // instance of thread; rng = random number generator (object)
	
	// variable for strength of password
	let mut score: i32 = 0; // 0 -> no password/very bad password
	
	// assign array with (ascii) signs
	let mut  signs : Vec<char> = vec![];
	
	// fill vector with number of ASCII signs allowed for password
	for i in 33u8..126u8 {
		signs.push(i as char);
	}
	
	// looping -> creation of password
	for _ in 0..pass_length {
		// choose random element from vector signs -> get ASCII signs for number
		password.push(*(rng.choose(&signs).unwrap())); // * for dereference for processing
	}

	// length, special character, different characters, non redundance
	let length = password.len(); // get length of string
	// println!("length: {}", password.len());

	// we have a password
	println!("password is: {}", password);
}


// function
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
