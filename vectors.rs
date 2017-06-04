// vectors
// jesus respawn is near!


// main function
fn main() { 
    // assigning vectors
    let mut v = vec![1, 2, 3, 4, 5]; // v: vec<i32>
    let w = vec![10; 0]; // vector of 10 zeroes
    
    // print particular element of v 
    println!("the 3. element of v is:{}", v[2]); // counting elements beginning at 0
    
    // index
    let i: usize = 0;
    // let j: i32 = 0; // doesn't work!
    println!("ite element: {}", v[i]);

    // handling out of bound errors
    match w.get(8) { // use 'get()' or 'get_mut()'
        Some(x) => println!("item 7 is: {}", x),
        None => println!("Sorry, this vectori is to short!")
    }
    
    // iteration of vectors
    for i in &v { // using unmutable references
        println!("A reference to {}", i);
    }
    for i in &mut v {
        println!("A mutable reference to {}", i);
    }
    for i in v { // note: you cannot use vector again!
        println!("Take ownership of the vector and its elements {}", i);
    }

    let a = 5; // a: i32
    let b = true; // b: bool
    let a2 = double(a); 
    let b2 = change_truth(b);
    println!("{}", a); // works, because i32 has no pointer -> copy trait implemented
    println!("{}", b); // works -> bool has copy trait
}


// function
fn double(x: i32) -> i32 {
    x * 2
}
fn change_truth(x: bool) -> bool {
    !x
}
