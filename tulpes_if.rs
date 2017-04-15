// 'primitive' types
// what a sunny easter day - best for programming!


// main function
fn main() { 
    // assigning tulpes
    let mut x = (1, 2); // x: (i32, i32)
    let y = (2, 3); // y: (i32, i32)
    x = y; // assign tulpe into another
    
    let x1 = x.0; // access first field of tulpe x
    let x2 = x.1; // access second field of tulpe x
    println!("the first element of x is: {}", x1);
    
    let (a, b, c) = (1, 2, 3,); // access tulpe by destructuring
    println!("the value of a is: {}", a);
    
    let d =(0,); // single element tulpe
    
    
    let z = 7; 
    
    // some looping
    if z == 5 {
        println!("z is five!");
    } else if z == 3 {
        println!("z is three!");
    }
    
    else {
        println!("z is not five and not three! :(");
    }
    
    // more looping, written differently
    let z = 9;
    
    let y = if z == 5 {
        10
    } else {
        15
    }; // y: i32
    println!("y is defined as: {}", y);
}

