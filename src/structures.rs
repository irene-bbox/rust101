#[allow(dead_code)]
#[allow(unused_variables)]

// import libraries
use std::mem;

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


// ARRAYS
pub fn array()
// Before we even start, remember that arrays are object of FIXED size.
// If you want something of variable size you must choose another data structure
{   // VECTORS, that is a 1D arrray
    // Fill in an array by specifying an initialized
    let mut a:[i32;5] = [1,2,3,4,5];  //an array is a data structure where the number of elements included is explicitly declared
    println!("Array {:?} has {} elements, first is {}", a, a.len(), a[0]);
    a[0] = 10;  //overwrite first element
    println!("New first element is {}", a[0]);

    if a != [1,2,3,4,5] {println!("match")} else if a == [10,2,3,4,5] {println!("changed")} else {println!()};


    // Bulk fill the array with the same value
    let b = [1;10]; //b.len() == 10
    // Below are 2 ways to print the array
    println!("B is {:?}", b);
    for i in 0..b.len()
    {
        println!("{}", b[i])
    }

    // Take a moment to ponder over memory optimization
    let b = [1;10];
    println!("b takes up {} bytes", mem::size_of_val(&b)); //undeclared element types
    let c:[u16;10] = [1;10];
    println!("b takes up {} bytes", mem::size_of_val(&c));  //array of u16-type elements
    let d:[i8;10] = [1;10];
    println!("b takes up {} bytes", mem::size_of_val(&d)); //array of i8-type elements


    // MATRICES, that is 2D+ arrays
    // Build matrices very easily as arrays of arrays
    let e:[[i8;3];3] =
    [
        [-1, 0, 0],
        [0, -1, 0],
        [0, 0, 1]
    ];
    println!("Matrix e is {:?}", e);

    // Now print the diagonal values in 2 distinct ways
    // 1st way
    for i in 0..e.len()
    {
        println!("{}", e[i][i])
    }


    // 2nd way
    for i in 0..e.len()
        {for j in 0..e[i].len()
            {if i==j //we are along the diagonal
                {println!("e[{}][{}] = {}", i, j, e[i][j])};
            }
        }
}



// SLICES
pub fn use_slice(slice: &mut[i32])
{
    println!("slice = {:?}, first elem = {}, len = {}", slice, slice[0], slice.len())
}


pub fn slices()
{ // Slices are parts of an array whose size is unknown at declaration time.
  // Mathematically, you can think of a slice as a mutable partition of an array. In other words, slices are the mutable counterpart to arrays.

    let mut slicy = [1,2,3,4,5];
    slicy[3] = 100;
    use_slice(&mut slicy[1..4]); //partition the array into a slice
    use_slice(&mut slicy); //convert the entire array into a slice
}


// TUPLES
// The types of elements in a tuple can mismatch, whereas in an array all elements are of the same type
pub fn tuples()
{
    let x:f32 = 6.0;
    let y:f32 = 4.0;
    let my_tuple = (x+y, x*y);
    println!("{0}+{1}={2}, and {0}*{1}={3}", x,y,my_tuple.0, my_tuple.1);

    // destructuring of a function returning a tuple
    fn power_and_division(x:f32, y:f32) -> (f32, f32)
    { (f32::powf(x,y), x/y) }
    let pd = power_and_division(x,y);
    let (a,b) = pd;
    println!("{0}^{1}={2}, and {0}/{1}={3}", x,y,a,b);

    // destructuring a tuple of tuples
    let pd2 = power_and_division(9.0,3.0);
    let combo_tuple = (pd, pd2);
    println!("{:?}", combo_tuple);
    let ((c,d),(e,f)) =  combo_tuple;

    // Tuples can contain mixed type elements
    let mixed_tuple = (true, 7.5, false, 'r', -1i8);
    println!("{:?}", mixed_tuple);

    // How do you make a single element tuple? It's simple, use a comma inside round brackets!
    let meaning = (42,); //if you omit the comma you obtain an integer instead of a tuple
    println!("{:?}", meaning);
}



struct point<T> {x:T, y:T}
struct Pointy<T,V> {x:T, y:V}
struct Liny<T> {start: point<T>, end:point<T>}

pub fn generics()
{
    let a:Pointy<i8,u32> = Pointy{x:9, y:11};
    let b:point<f32> = point{x:1.0, y:3.0};
    let c:point<f32> = point{x:6f32, y:7f32};
    println!("a = ({},{}), b = ({},{})", a.x, a.y, b.x, b.y);

    let myline:Liny<f32> = Liny{start:b, end:c};
}
