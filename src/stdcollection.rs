#[allow(dead_code)]
#[allow(unused_variables)]

// Import libraries
use std::collections::HashMap;


// VECTORS

// Use vectors whenever you want to create array-like objects which grow dynamically.
// Arrays have a pre-defined length and therefore cannot grow. Vectors can! A vector is pretty much like a stack of elements
pub fn vectors()
{
    let mut my_vec = Vec::new(); // create a mutable vector
    my_vec.push(11.0); // add values to the vector
    my_vec.push(12.0);
    my_vec.push(13.0);

    //let indx:i32 = 0; // An index MUST be a u-size, that is unsigned and of standard size. It I'm running a 64bit machine it makes no sense for the usize to be 32.
    // So, if we ran the line of code above, the program crashed because it doesn't recognize i32 as a legitimate index type. However, the line of code below works fine:
    let indx:usize = 0;

    println!("my_vec = {:?}, first elem = {}, {}", my_vec, my_vec[0], my_vec[indx]); // {:?} is called the 'debug output'
    // Index elements in a vector in the same way how you index elements in an array

    // Now try and fetch the 6th element from a vector of len()=4
    match my_vec.get(5)
    {
        Some(x) => println!("6th elem ={}", x),
        None => println!("There is no such element. my_vec has length {}", my_vec.len())
    }; // Lesson learned: use an Option<T> Some(), None to write safe code whenever you want to try to access an element in a vector


    // Add elements to a vector with the push() function
    my_vec.push(14.0);

    // Iterate a vector
    for i in &my_vec { println!("{}", i); };

    // Remove elements from a vector with the pop() function
    let last_elem = my_vec.pop();
    println!("my_vec = {:?}, Last elem = {:?}", my_vec, last_elem); // You can only print out last_element if you use the 'debug output' because what you're printing is not a number, but a Some()
    // println!("Last elem = {}", my_vec[-1]);

    // Remove all elements from a vectors in reverse order (last to first)
    while let Some(x) = my_vec.pop() // while the results of my_vec.pop() yields something, run print statement
    {
        println!("{}", x);
    }

    // NOTICE that both the get() and the pop() function both return an Option<T>
}



// HASHMAPS
// Hashmaps are containers for pairs of values
pub fn hashmap()
{
    // initiate a HashMap with 2 pairs
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    for (key,value) in &shapes { println!("{}:{}", key, value); };

    // overwrite the second pair
    shapes.insert("square".into(), 7);
    println!("{:?}", shapes);

    // insert and modify pairs
    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}",shapes);
    {
        let actual = shapes
            .entry("circle".into())
            .or_insert(2);
        *actual = 0;
        println!("{:?}",shapes);
    }
}