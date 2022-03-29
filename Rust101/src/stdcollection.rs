#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_must_use)]
// Import libraries
use std::collections::HashMap;
use std::collections::HashSet;
use std::thread;
use std::time;

// VECTORS

// Use vectors whenever you want to create array-like objects which grow dynamically.
// Arrays have a pre-defined length and therefore cannot grow. Vectors can! A vector is pretty much like a stack of elements
pub fn vectors() {
    let mut my_vec = Vec::new(); // create a mutable vector
    my_vec.push(11.0); // add values to the vector
    my_vec.push(12.0);
    my_vec.push(13.0);

    // let indx:i32 = 0; // An index MUST be a u-size, that is unsigned and of standard size. If I'm running a 64bit machine it makes no sense for the usize to be 32.
    // So, if we ran the line of code above, the program would crash because it doesn't recognize i32 as a legitimate index type. However, the line of code below works fine:
    let indx: usize = 0;

    println!(
        "my_vec = {:?}, first elem = {}, {}",
        my_vec, my_vec[0], my_vec[indx]
    ); // {:?} is called the 'debug output'
       // Index elements in a vector in the same way how you index elements in an array

    // Now try and fetch the 6th element from a vector of len()=4
    match my_vec.get(5) {
        Some(x) => println!("6th elem ={}", x),
        None => println!(
            "There is no such element. my_vec has length {}",
            my_vec.len()
        ),
    }; // Lesson learned: use an Option<T> Some(), None to write safe code whenever you want to try to access an element in a vector

    // Add elements to a vector with the push() function
    my_vec.push(14.0);

    // Iterate a vector
    for i in &my_vec {
        println!("{}", i);
    }

    // Remove elements from a vector with the pop() function
    let last_elem = my_vec.pop();
    println!("my_vec = {:?}, Last elem = {:?}", my_vec, last_elem); // You can only print out last_element if you use the 'debug output' because what you're printing is not a number, but a Some()
                                                                    // println!("Last elem = {}", my_vec[-1]);

    // Remove all elements from a vectors in reverse order (last to first)
    while let Some(x) = my_vec.pop()
    // while the results of my_vec.pop() yields something, run print statement
    {
        println!("{}", x);
    }

    // NOTICE that both the get() and the pop() function both return an Option<T>
}

// HASHMAPS
// Hashmaps are containers for pairs of values. Hashmaps in Rust = Dictionary in Python
pub fn hashmap() {
    // initiate a HashMap with 2 pairs
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    for (key, value) in &shapes {
        println!("{}:{}", key, value);
    }

    // overwrite the second pair
    shapes.insert("square".into(), 7);
    println!("{:?}", shapes);

    // insert and modify pairs
    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 27;
        println!("{:?}", shapes);
    }
}

// HASHSETS
// A Hashset is a data type that represents a mathematical set, i.e. a container of unique, unordered elements. Hashsets in Rust = Sets in Python
// Use a hashset when you want a container with unique elements and no duplicates
pub fn hashset() {
    let mut greek = HashSet::new();
    greek.insert("alpha");
    greek.insert("beta");
    greek.insert("gamma");
    greek.insert("delta");
    print!("A handful of greek letters: {:?}", greek); // the order with which the elements are acquired from the container is NOT the same order as you put the elements in

    greek.insert("alpha");
    print!("Same handful of greek letters: {:?}", greek); // if you double insert the same element into the set, nothing happens. The hashset only sources unique elements!

    // How can I check whether your insertion succeeded? Use a boolean
    let add_letter = greek.insert("omega");
    if add_letter {
        print!("Insertion succeeded")
    };

    // How can I check whether an element is contained in a set? Use the function containt(), which returns a boolean
    if !greek.contains("epsilon") {
        println!("Epsilon is not in this set")
    } else {
        println!("Epsilon is in the set")
    };
    print!("greek = {:?}", greek);

    // Remove an element from a set and check whether removal succeeded using a boolean
    let removed = greek.remove("delta");
    if removed == true {
        println!("Removal succeeded")
    };
    print!("greek = {:?}", greek);

    // Subsets
    let OneToTen: HashSet<_> = (1..=10).collect();
    let OneToFive: HashSet<_> = (1..=5).collect();
    let FourToNine: HashSet<_> = (4..=9).collect();
    let subset = OneToFive.is_subset(&OneToTen);
    if subset == true {
        println!("Yes, it's a subset")
    };
    println!("{:?} is a subset of {:?}? {}", OneToFive, OneToTen, subset);

    // Disjoint =  no common elements
    let disjoint = OneToFive.is_disjoint(&FourToNine);
    println!(
        "{:?} is disjoint from {:?}? {}",
        OneToFive, FourToNine, disjoint
    ); // always notice that the data output is entirely out of order

    // Union
    let union = OneToFive.union(&FourToNine);
    println!(
        "The union of {:?} and {:?} is = {:?}",
        OneToFive, FourToNine, union
    );

    // Intersection
    let intersection = OneToFive.intersection(&FourToNine);
    println!(
        "The intersection of {:?} and {:?} is = {:?}",
        OneToFive, FourToNine, intersection
    );

    // Difference = A-B =  items in the first set but not in the second
    let difference = OneToFive.difference(&FourToNine);
    println!("Set {:?} - {:?} = {:?}", OneToFive, FourToNine, difference);

    // Symmetric difference = A∪B - A∩B =  items that are in the union but not in the intersection = union-intersections
    let symm_difference = OneToFive.symmetric_difference(&FourToNine);
    println!(
        "Set {:?} - {:?} = {:?}",
        OneToFive, FourToNine, symm_difference
    );
}

// ITERATORS
pub fn iterators() {
    let mut vec = vec![1, 2, 3]; // initialize a vector

    // Here are some alternative ways to iterate through a vector:
    // (1) for loop
    // for i in vec { println!("{}", i)};
    for i in &vec {
        println!("{}", i)
    } // iterate through the vector using a for loop
    for i in &vec {
        println!("{}", *i)
    }
    // What is the difference among the 3 lines of code above? Their syntax is different, they all return the same output, but their logic is different.
    // The '&' is  pointer allowing me to copy and object and iterate through it, instead of 'moving' the object and iterate through it.
    // Whenever you iterate through something, remember to use '&' to create a copy of the original object, otherwise, you won't be able to call the object again later in the code because it got moved.

    // (2) iter()
    // iter() returns immutable references for every element in that vector
    for i in vec.iter() {
        println!("{}", *i)
    }

    // (3) iter_mut()
    // iter_mut() returns mutable references for every element in that vector
    for i in vec.iter_mut() {
        *i += 5;
        println!("{}", *i)
    }
    println!("vec = {:?}", vec);
    for j in vec.iter().rev() {
        println!("reversed vec = {}", j)
    } // rev() for reverse iteration

    // (4) into_iter()
    // into_iter() is a move operation that transforms the collection into a bivalue iterator
    let mut vec1 = vec![1, 2, 3, 4];
    let mut vec2 = vec![5, 6, 7, 8];
    vec1.extend(vec2); // what happens behind the scenes is that the extend() function calls the into_iter() function to extract all values from a vector and iterates them,
                       // making the original vector unusable! The collection gets destroyed but the elements are preserved.
    println!("append = {:?}", vec1);
}
