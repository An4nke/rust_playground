// another rust programm
// let's get started ^.^

extern crate rand; // actually using rand (declared in dependencies of 'Cargo.toml')

// rust only uses few things by default ('prelude') -> add your needs!
use std::io; // use io library from the standard library
use std::cmp::Ordering; // use ordering type
use rand::Rng; // using rands function rand::thread_rng()


// main function
fn main() { 
    println!("guess a number!"); // macro, prints string
    
    let secret_number = rand::thread_rng() // get copy of random number generator
        .gen_range(1, 101); // methods takes 2 arguments -> generate random number between (inclusive on the lower bound, but exclusive on the upper bound -> 1 - 100)

    // println!("The secret number is: {}", secret_number); // print out secret number // just for testing -> runs the game!
    
    
    loop { // building infinity loop
    
        println!("please input your guess!."); // macro, prints string

        let mut guess = String::new(); // let -> create 'variable bindings'; String -> string type (UTF-8 encoded bit of text); '::' -> associate function; 'new()' -> created empty
        let foo = 5; // 'foo' is immutable
        let mut bar = 5; //'bar' is mutable
        
        io::stdin().read_line(&mut guess) // calling associated function // without std::io -> written 'std::io::stdin()' //returns handle to standard input
            // read_line() -> calling method (like associate function, only on parcticular instant of type) an handle
            // read_line():&mut guess -> passing argument to mutable reference of guess
            .expect("failed to read line"); // io::Result encodes error handling information // not successful -> panic! with message "failed to read line"
        // alternative written: 'io::stdin().read_line(&mut guess).expect("failed to read line");'
        
        // let guess: u32 = guess.trim().parse() // 'shadow' guess with new one -> convert string into u32 // 'trim()' eliminates whitespace // 'prase()' -> parses string into number
            // .expect("Please type a number!"); // stop programm if not a number
        
        let guess: u32 = match guess.trim().parse() { // 'handling error' -> 'match' instead 'expect()' // result returned by prase()
            Ok(num) => num, // success -> set name num on unwrapped Ok value (integer)
            Err(_)  => continue, // failure -> don't care of kind of error -> catch all _(everything not ok) // 'continue' -> move to next iteration of loop (aka ignore all errors)
        };
        
        println!("you guessed: {}", guess); // print out saved string from STDIN
        println!("FOO: {} & BAR: {}", foo, bar); // print out string variables
        
        match guess.cmp(&secret_number) { // cmp can called on anything comparable, takes references to thing to compare // returns odering type // match -> determine exactly what typ of ordering (ordering = enum (enumeration)) // guess + secret_number have to be same type for comparision!
                Ordering::Less      => println!("To small!"), // ordering enum -> 3 possible variants (less/greater/equal)
                Ordering::Greater   => println!("To big!"),
                Ordering::Equal     => {
                    println!("You win!");
                    break;
                }
        }
    }
}
