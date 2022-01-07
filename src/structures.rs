#[allow(dead_code)]
#[allow(unused_variables)]

// GEOMETRICAL STRUCTURES
// Let's create a point in a 3D space using 'struct'
struct Point {x:f64, y:f64, z:f64}

// Let's create a line object using 'struct'
struct Line{start:Point, end:Point}

// Let's write a function that uses the point
pub fn structures()
{
    let p1 = Point {x:1.0, y:3.0, z:3.0};
    println!("Point p1 is located at ({}, {}, {})", p1.x, p1.y, p1.z);
    let p2 = Point {x:8.0, y:0.0, z:10.0};
    let my_line = Line {start:p1, end:p2};
}



// ENUMERATIONS
// create an enumeration of colors
// enum objects can comprise of ordinary members, tuple-like members, or struct members
enum Color {
    Red,
    Green,
    Blue, //ordinary
    rgbColor(u8,u8,u8), //tuple
    cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8} //struct
}

// use a match statement to find our the color and print it
// The function below does patter matching against the elements of an enum
pub fn enums()
{
    let c = Color::rgbColor(1,2,3);
    match c
    {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::rgbColor(0,0,0)
            | Color::cmyk{cyan:_,magenta:_,yellow:_,black:255}  => println!("black"),
        Color::rgbColor(r,g,b) => println!("rgbColor({},{},{})",r,g,b),
        Color::cmyk{cyan,magenta,yellow,black} => println!("cmyk({},{},{},{})",cyan,magenta,yellow,black),
        _ => () //catch all that does absolutely nothing, but makes the match statement exhaustive
    }
}




// UNIONS
// Unions are data structures typical of C+ and C++, which are used to allocate a piece of memory to something w/o specifying what that something is.

// Imagine you want to assign 32 bits of memory to store either an integer or a floating point
union IntOrFloat {i: i32, f: f32}  // example of a 32bits union

pub fn unions()
{
    let mut iof = IntOrFloat {i:365};
    iof.i = 218;
    let value = unsafe {iof.i};  // the difficult aspect about unions is retrieving the values from it, since we don't know what type they are. That's why we must declare the scope to be 'unsafe' for the program to run
    println!("iof = {}", value);
}



// OPTION <T>, a built-in type to handle Nones
pub fn options()
{
    let x = 14.0;
    let y = 7.0;
     //Option -> Some(value) | None
    let result = if y != 0.0 {Some(x/y)} else {None};

    match result {
       Some(z) => println!("{}/{}={}", x,y,z),
        None => println!("Can't divide by zero!")
    }


    // Rust has special keywords to check whether something contains None. Two of these keywords are 'if let' and 'while let'
    if let Some(z) = result {println!("result is {}", z)}
    // in the case above, if the result it equal to none, then the if statement doesn't get executed and vice versa

}