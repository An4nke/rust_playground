// ownership
// Happy Jesus Respawn! ;D


// main function
fn main() { 
    // assign vectors
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    // use functions
    let (v1, v2, answer) = foo(v1, v2);
    let answer2 = boo(&v1, &v2);
    println!("Answer is: {}", answer);
    println!("Second answer is: {}", answer2);

    // playing with mutable references
    let mut x = 5;
    { // needed for error free compiling
        let y = &mut x; // y borrows x here
        *y += 1; // '*' needed to access references
    }
    println!("{}", x); // tries to borrow x here -> only 1 &mut x allowed!
    
    let y = &5; // the same as 'let _y = 5; let y = &_y'
    let f = Foo {x: y };
    println!("{}", f.x);
}


// functions
fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // return answer // give back ownership of vectors
   (v1, v2, 42)
}

//better version of 'foo' using advantage of borrowing
fn boo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // give back answer
    42
}




// struct -> way for creating more complex data types
struct Foo<'a> { // declaring lifetime 'a
    x: &'a i32, // using lifetime 'a
}
