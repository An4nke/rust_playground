// don't rust -> restart

#[macro_use] extern crate text_io;

// rust only uses few things by default ('prelude') -> add your needs!
use std::io; // use io library from the standard library

fn main() {
	//let n: i32 = read!();
	//println!("Read in: {}", n);
	//fib (n);
	let n: i32 = 1;
	fib (n);
}

// Write a function fib which takes a single i32 argument n < 10 and returns n's Fibonacci number (also i32).

fn fib (n:i32) {
	println!("{} \n", n);
}
