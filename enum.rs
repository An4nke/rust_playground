// enum
// 


// main function
fn main() { 
    // use enums
    let x: Message = Message::Move { x: 3, y: 4 };
    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

    // enum working like function
    let m = Message::Write("Hello World".to_string());
    let x = foo("Hello World".to_string()); // same as above, using function 'foo()'

    // convert vector of strings into vector of Message::Write
    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
}

// enmus
enum Message {
    Quit,
    ChangeColor(i32, i32, i32), // tulpe
    Move { x: i32, y: i32 }, // struct
    Write(String),
}
enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}

//functions
fn foo(x: String) -> Message {
    Message::Write(x)
}
