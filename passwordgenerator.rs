#![allow(unstable)] // allow unstable libraries


// programm for password generation


extern crate rand;
use rand::Rng;
// use std::__rand::thread_rng;

// main function
fn main() { 

	// variable for length of password
	let mut pass_length: i32 = 8;
	let mut password = String::new(); // assign password as empty string
	let mut rng = rand::thread_rng(); // instance of thread; rng = random number generator (object)
	
	// variable for strength of password
	
	// length, special character, different characters, non redundance
	
	
	// assign array with (ascii) signs
	let mut  signs : Vec<char> = vec![];
	
	// fill vector
	for i in 33u8..126u8 {
		signs.push(i as char);
	}
	
	// looping -> creation of password
	for _ in 0..pass_length {
		//password.push(rng.choose(&signs).unwrap()); // get only password
		password.push(*(rng.choose(&signs).unwrap())); // * for derefernces for processing
	}

	// we have a password
	println!("password is: {}", password);

	// create random number between (range array elements)
	
	// from number of array element -> password
	
	
	// create password
//	let s = rand::thread_rng()
//		.gen_ascii_chars() // generate ASCII signs
//		.take(pass_length) // take signs for password
//		.collect::<String>();

	// print out password
//	println!("random string: {}", s);

	// assign variable for password
//	let mut str = String::new();
	
	// create password
//	for _ in () {
//		str.push(rand::random::<u8>() as char); // push character to string str
//	}

	// assing counter 
//	let mut i: i32 = 0;

//	for (i = 0, i < pass_length, i++) {
//		str.push(rand::random::<u8>() as char); 
//	}

//	let choices = [33..126];
//	let mut rstr = String::new();
//	let mut rng = rand::thread_rng();

//	for _ in 0..8 {
//		  rstr.push((rng.choose(&choices).unwrap() as u8) as char); // Return a random element from values
//	}

}
