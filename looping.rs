// loops
// 


// main function
fn main() { 

    // infinity loop
    // loop {
       // println!("neverending looping!");
    // }
    
    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool
    
    while !done { // looping as long as done == true
        x += x - 3; // adding 2 to x
        println!("{}", x);
        if x % 5 == 0 { // stop looping if x is multiple of 5
            done = true;
        }
    }
    
    // redone of upper while loop -> better way
    loop {
        x += x - 3; // adding 2 to x
        println!("{}", x);
        if x % 5 == 0 { // stop looping if x is multiple of 5
            break; // break out loop
            // return; // will do the same as 'break'
        }
    }
    
    // for loop
    'outer: for y in 0..10 {
        'inner: for mut x in 0..10 { // from 0 to 9, 10 (upper bound is exclusive)
            x += 1 + x;
            if x % 2 == 0 { continue 'inner; } // go to next step inner loop -> only print odd numbers
            if y % 2 == 0 { continue 'outer; } // go to next step outer loop
            println!("now x is: {} and y is: {}", x, y); // x: i32
        }
    }

    for (index, value) in (5..10).enumerate() {
        println!("index = {} value = {}", index, value);
    }
    
    let line = "Hello\nWorld!".lines();
    
    for (linenumber, line) in line.enumerate() {
        println!("{}: {}", linenumber, line);
    }
}

