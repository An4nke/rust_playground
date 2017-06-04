// match
// 


// main function
fn main() { 
    let x = 2;
    let a = 1;
    
    match a {
        b => println!("a: {} b: {}", a, b), // catches '_' 'any case'
    }
    
    match x { // more powerful than if/else
        1 => println!("one"), // 
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("something else"), // ‘exhaustiveness checking’ -> error if not used
    }

    let number = match x { // use match for variable bindings
        1 => "one", // integer converted into string
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };
    
    println!("number is: {}", number);

    struct Point {
        x: i32,
        y: i32,
    }
    
    let origin = Point { x: 0, y: 0 }

    match origin { // deconstruct compound data type
        Point { x, y } => println!("({},{})", x, y),
    }

}

// enmus
enum Message {
    Quit,
    ChangeColor(i32, i32, i32), // tulpe
    Move { x: i32, y: i32 }, // struct
    Write(String),
}

// funtions
fn quit() { /* ... */ }
fn change_color(r: i32, g: i32, b: i32) { /* ... */ }
fn move_cursor(x: i32, y: i32) { /* ... */ }

// process possible variants of enum with match
fn process_message(msg: Message) {
    match msg {
        Message::Quit => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move { x, y: new_name_for_y } => move_cursor(x, new_name_for_y),
        Message::Write(s) => println!("{}", s),
    };
}
