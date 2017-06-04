// patterns
// f*up life..


// main function
fn main() { 
    // assign tuple
    let tuple: (u32, String) = (5, String::from("five"));
    
    // move string -> move tiple
    let (x, _s) = tuple;
    
    // gives error because tuple is moved
    // println!("Tuple is: {:?}", tuple);
    
    let tuple = (5, String::from("five"));
    
    // tuple is not moved -> u32 is a copy
    let (x, _) = tuple;
    
    // works
    println!("Tuple is: {:?}", tuple);
    
    let x = 1;
    
    match x { // bind values to names
        e @ 1 ... 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
    
    #[derive(Debug)] // compiler -> basic implementations for some traits via the #[derive] attribute, traits can manually implemented
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) };
    
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }

}
