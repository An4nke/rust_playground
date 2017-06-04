// struct
// snowy eastern oO


// main function
fn main() { 
    // creating complex data type
    let origin_x = 0;
    let origin_y = 0;
    
    let origin = Point { x: 0, y: 0 }; // origin: Point
    
    println!("The origin is at ({}, {})", origin.x, origin.y);
    
    let mut point = Point { x: 0, y: 0 };
    point.x = 5; // excess field by name

    println!("The point is at ({}, {})", point.x, point.y);
    
    {
        let r = PointRef { x: &mut point.x, y: &mut point.y };

        *r.x = 5;
        *r.y = 6;
    }
    
    assert_eq!(5, point.x); // 'assert_eq' -> assert two expression are equal
    assert_eq!(6, point.y);

    let mut point3d = Point3d { x: 0, y: 0, z: 0 };
    // point3d = { x: 1, .. point3d }; // gives point a new y, but keeps x and z // doesn't work anymore
    
    
    // making new point
    // let point_new = Point3d { z: 1, x: 2, .. origin };

    // assigning color black (tuple struct)
    let black = Color(0, 0, 0); // not the same type as 'let origin = Point(0, 0, 0)'
    let origin = Point2(0, 0, 0);
    
    // tuple struct with only one element -> 'newtype'
    let length = Inches(10);
    
    let Inches(inter_length) = length; // extract inner integer typ
    let inter_length2 = length.0; // does the same as aboves
    
    println!("length is {} inches", inter_length);
}

// combine two single data types into one
struct Point { // struct for creatiting 2D point
    x: i32, // could use 'let' here
    y: i32,
}

// structure with reference pointer
struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

// structure for 3D points
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

// tuple struct
//struct Color(i32, i32, i32);
struct Point2(i32, i32, i32);
struct Inches(i32);

// struct -> clearer
struct Color {
    red: i32, // actually names instead positions
    blue: i32,
    green: i32,
}
