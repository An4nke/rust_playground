// another rust programm
// let's get started ^.^


use std::io; // use io library from the standard library
// rust only uses few things by default ('prelude') -> add your needs!


//main function
fn main() { 
    println!("guess a number!"); // macro, prints string
    
    println!("please input your guess!."); // macro, prints string

    let mut guess = String::new(); // let -> create 'variable bindings'; String -> string type (UTF-8 encoded bit of text); '::' -> associate function; 'new()' -> created empty
    let foo = 5; // 'foo' is immutable
    let mut bar = 5; //'bar' is mutable
    
    io::stdin().read_line(&mut guess) // calling associated function // without std::io -> written 'std::io::stdin()' //returns handle to standard input
        // read_line() -> calling method (like associate function, only on parcticular instant of type) an handle
        // read_line():&mut guess -> passing argument to mutable reference of guess
        .expect("failed to read line"); // io::Result encodes error handling information // not successful -> panic! with message "failed to read line"
        
    println!("you guessed: {}", guess); // print out saved string from STDIN
    println!("FOO: {} & BAR: {}", foo, bar) // print out string variables
}
