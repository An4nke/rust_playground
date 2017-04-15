// here plays the music!
// Syntax & Semantic


// main function
fn main() { 
    let x: i32 = 5; // set x unmutable, integer 5
    
    let mut y = 7; // set y mutable 5 (here an integer)
    println!("The value of y is: {}", y);
    y = 4; // change y
    println!("The value of y is: {}", y);
    
    {
            println!("The value of x is: {}", x);
            let x: i32 = 7; // changing x, only inside loop
            println!("The value of x is: {}", x);
    }
    let y = "I'm a text now!"; // change y again (here as text)
    println!("The value of y is: {} ", y);
    
    println!("The value of x is: {}", x);

    print_number(88); // call function
    sum(3, 5);
    let a = add_one(x);
    
    let z = (x + 2);
    println!("New values a: {}, z: {}", a, z);

    // variable bindings with point to function
    let f: fn(i32) -> i32 = add_one; // without type inference
    //let f = add_one; // with type inference
    let six = f(5); // call function
    println!("called function says: {}", six);
}


// declaring function

// function for printing numbers
fn print_number(x: i32) {
    println!("x is: {}", x);
}

// function for adding two numbers
fn sum(x: i32, y: i32) {
    println!("sum is: {}", x + y); // print sum of numbers
}

// adding 1 to number
fn add_one(i: i32) -> i32 { // -> i32 declairs return of function
    i + 1 // here no ';' -> Rust = expression based language 
    // x + 1; doesn't retourn value
}

// never returning function
fn diverges() -> ! {
    panic!("This function never returns!"); // macro -> causes the current thread of execution to crash
}
