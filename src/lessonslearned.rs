#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_must_use)]
// Import libraries
use std::collections::HashMap;
use std::collections::HashSet;

// PRIMITIVES
// Rust has a humber of types which are built-in to the language and are therefore called 'primitives'. These include:
// Scalar Types:
// - signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
// - unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
// - floating point: f32, f64
// - char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
// - bool either 'true' or 'false'
// - and the unit type '()', whose only possible value is an empty tuple: ()
// Compound Types:
// - arrays like [1,2,3]
// - tuples like (5.0, true)
// Indexing: arrays[0] & tuples.0 & vectors[0]. Tuples use a . , arrays and vectors use a []

// SLICES
// A ‘slice’ is a reference to (or “view” into) another data structure. They are useful for allowing safe, efficient access to a portion of an array without copying.
// By nature, a slice is not created directly, but from an existing variable binding. Slices have a defined length, and can be mutable or immutable.

// VECTORS
// A ‘vector’ is a dynamic or ‘growable’ array, implemented as the standard library type Vec<T>. The T means that we can have vectors of any type (see the chapter on generics for more).
//  Vectors always allocate their data on the heap. You can create them with the vec! macro

// &mut REFERENCE
// A ‘mutable reference’: '&mut T' allows you to mutate the resource you’re borrowing.

// TYPE DETECTION
// How do I find out the type of a variable in Rust? Quick and dirty way: cause an error (e.g., by running a non-existent method on that variable) and get the compiler to pick it up

pub fn summary() {
    let a: i8 = -128; //regular annotation
    let b = 127i8; //suffix notation
    let c = 3.0; //inferred from context 'f64'
    println!("a, b, c = {}, {}, {}", a, b, c);
    let bol: bool = true;
    //bol.what_is_this();  //the method .what_is_this() is entirely made up by Irene. Run it to cause an error and find out the type of 'bol'
    let mut d: i32 = 1234;
    d += 1;
    println!("d = {}", d);
    let mut e: u64 = 10000; //A mutable variable can be changed
                            //e = true; //Error! The type of a variable can't be changed
    let e = true; //Variables can be overwritten by shadowing
    let myarray = [1, 2, 3, 4, 5]; //array:[type, length] = [....]
    let mytuple = (1, true, 5.0f64, -7i8, 'A'); //tuple:(type, type, type, ...) = (.., .., .., ..) a tuple can have any length and mixed types
    println!("An array and a tuple {:?}, {:?}", myarray, mytuple);
    println!(
        "first elem = {}, array.len() = {}",
        myarray[0],
        myarray.len()
    );
    let tuple_of_tuple = ((-2i8, 0u8, 'A'), (false, 1.0f32), 'c');
    let f = (17i8,); //To create one element tuples, the comma is required to tell them apart from a literal surrounded by parentheses
    println!("tuple_of_tuple = {:?}, f = {:?}", tuple_of_tuple, f);
    let g: char = 'g';
    let h = &myarray[0..=2];
    let i = &myarray[..];
    println!("g = {}, h= {:?}, i = {:?}", g, h, i);

    let mut x = 5; // &mut reference
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

    let mut myvec = vec![2, 4, 6, 8]; // generate a vector
    let mut myvec2: Vec<i32> = (20..=25).collect();
    myvec.push(10);
    myvec.push(12);
    myvec.push(14);
    let indi: usize = 3; // Important: you must index a vector with usize type
    println!(
        "myvec = {:?}, 4th elem = {}, len = {}",
        myvec,
        myvec[indi],
        myvec.len()
    );
    for i in myvec.iter().rev() {
        if *i == 10 {
            continue;
        } else {
            println!("{}", i);
        }
    } // iterate through a vector in reverse, skip i==10
    myvec.extend(myvec2); // combine 2 vectors
    println!("myvec after extension = {:?}", myvec);

    let mut dic = HashMap::new(); // generate a HashMap
    dic.insert('a', 1);
    dic.insert('b', 2);
    dic.insert('c', 3);
    dic.insert('c'.into(), 6); //overwrite
    dic.entry('d'.into()).or_insert(4); //insert or overwrite
    println!("{:?}", dic);

    let mut set = HashSet::new(); // generate a HashSet
    set.insert(3);
    set.insert(2);
    set.insert(1);
    let remove = set.remove(&3); // remove elements
    if set.contains(&3) {
        "3 is in the set"
    } else {
        "3 not in set"
    }; // check for elements
    let superset: HashSet<i32> = (5..=15).collect(); // generate a new set called 'superset'
    println!("set = {:?}, superset = {:?}", set, superset);

    // Functions and function pointers
    fn foo(j: i32) -> i32 {
        return j;
    }
    let k: fn(i32) -> i32 = foo; // In this case, k is a ‘function pointer’ to a function that takes an i32 and returns an i32
    foo(d);
    k(d);

    let l: char = 'l';
    let m: &str = "I am a &str";
    let n = "I am a String".to_string();
    let n_slice = &n[0..=3];
    let o: String = String::from("There's a jedi in our office");
    let p = "Test String";
    println!(
        "l = {}, m = {}, n = {} o = {}, p = {}, n_slice={}",
        l, m, n, o, p, n_slice
    );
    for i in m.chars() {
        println!("{}", i)
    }
}
