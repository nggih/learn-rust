#[derive(Debug)]
struct Person {
    name: String, 
    age: u8,
}

// A unit struct 
struct Unit;

// a tuple struct 
struct Pair(i32, f32);

// A struct with two fields 
struct Points {
    x: f32,
    y: f32,
}

// structs can be resued as fields of another struct 
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom corners are in space. 
    top_left: Point, 
    bottom_right: Point,
}

fn main {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    //print debug struct 
    println!("{:?}", peter);

    // instantiate a 'Point'
    let point: Point = Point { x: 10.3, y: 0.4 };

    // access the fields of the point 
    println!("point coordinates: ({}, {})", point.x, point.y);

    // make a new point by using struct update syntax to use the fields of our other one
    
}