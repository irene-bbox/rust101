#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_must_use)]

// Import libraries

// A simple printing function
fn printer(num: i32) {
    println!("{}", num);
}

// A simple doubling function
fn double(int: &mut i32) {
    *int *= 2;
    println!("{}", int);
}

// A simple product function
fn product(x: i32, y: i32) -> i32 {
    x * y
}

pub fn funk() {
    let i: i32 = -2817;
    printer(i);

    let mut j = 9;
    double(&mut j);

    let p = product(6, 7);
    println!("{}", p);
}

// Remember:
//     '&' REFERENCES a variable
//     '*' DEREFERENCES a variable
//      both of them are placed in front of the variable

//      -> single arrow means 'return' inside a function
//      => double arrow means 'match' inside a match statement

//      a function with a return -> statement does NOT use ; at the end of the statement or line of code
//      all other functions must have ; at the end of each line of code

// METHODS
// A Method is an implementation of a function for a struct
// Use Methods to add behaviour to structures by providing methods inside an 'impl' block
// For example, create a len() function to calculate the length of a line (a line in this case is a struct)

// Define structures
struct Point {
    x: f32,
    y: f32,
}
struct Line {
    start: Point,
    end: Point,
}

// Implement method
impl Line // here we are 'impl'=implementing a function eval_length() for the struct Line
{
    fn eval_len(&self) -> f32 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub fn methods() {
    let p1 = Point { x: 2.0, y: 1.0 };
    let p2 = Point { x: 8.0, y: 7.0 };
    let line = Line { start: p1, end: p2 };
    println!("{}", line.start.x); // print the x coordinate of the inception point for the line
    println!("Line length = {}", line.eval_len());
}

// CLOSURES
// A closure is a function defined in line. A closure follows this structure:
// LET variable_name = | parameter:param_type | -> return_type { body_of_the_function }
// Store functions in variables. This happens quite commonly for high-level functions. A simple Baby Yoda example follow below
fn yoda_greetings() {
    println!("Greetings from baby Yoda!")
}
pub fn closures() {
    let yg = yoda_greetings; // store a function as a variable
    yg();

    // (1)
    let plus_one = |x: i32| -> i32 { x + 1 }; // this is a closure
                                              // Stop&Ponder! plus_one is a function that exists only inside he closures().
                                              // On the other hand, yoda_greetings() is a function that exists everywhere, inside and outside of scopes.
    let a: i32 = 10;
    println!("{} + 1 = {}", a, plus_one(a));

    // (2)
    let plus_three = |x| x + 3; // this is a more concise closure
    println!("{} + 3 = {}", a, plus_three(a));

    // (3)
    let plus_four = |x| // this is a verbose and long-winded closure
    {
        let mut z = x;
        z += 4;
        z
    };
    println!("{} + 4 = {}", a, plus_four(a));

    // (4)
    let mut add: i32 = 5;
    {
        let plus_add = |x| x + add;
        println!("{} + {} = {}", a, add, plus_add(a));
    }
    // When the closure borrows a variable, it takes entire possession of it so that the variable
    // is no longer callable nor usable outside the closure... until that variable is -so to say- released from the closure.
    // Solution? Use a scope to destroy the closure before calling back the desired variable.
    let borrow_add = add;
    println!("{} = {}", borrow_add, add);

    // (5) pass the argument as type-T: by value
    let plusone = |mut x: i32| x += 1;
    let mut c = 100;
    plusone(c);
    println!("b = {}", c); // here, we are feeding into the closure a copy of the value, so outside of the closure 'c'=100 is always equal to 100.
                           // In other words, we fed a copy of a variable into a lambda function, but we didn't change its original value

    // (6) pass the argument as mutable reference: &mut &
    let plusone = |x: &mut i32| *x += 1;
    let mut b = 100;
    plusone(&mut b);
    println!("b = {}", b); // here we are feeding into the closure a value by reference, so the function overwrites the variable it's being fed
}

// HIGHER ORDER FUNCTIONS are:

// 1. functions that take functions
// f(g) {g(x)}

// 2. functions that return functions (named 'Generators')
// f() -> g()

// Example of a function that takes a function: Sum of all even squares < 500
fn is_even(x: i32) -> bool {
    x % 2 == 0
}

pub fn high_order_fn() {
    let limit: i32 = 500;
    let mut sum = 0;

    for i in 1.. {
        let sq = i * i;
        if sq >= limit {
            break;
        } else if is_even(sq) {
            sum += sq;
        }
    }
    println!("Sum of even squares < 500 = {}", sum);

    // Example of a function that returns a function
    fn greater_than(limit: i32) -> impl Fn(i32) -> bool {
        move |y| y > limit
    }

    // let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);

    let mut sum2 = 0;
    for i in 1.. {
        let sq = i * i;
        if above_limit(sq) {
            break;
        } else if is_even(sq) {
            sum2 += sq;
        }
    }
    println!("Sum2 of even squares < 500 = {}", sum2);

    // Example of a higher order function
    let sum3: i32 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("Sum3 of even squares < 500 = {}", sum3);
}
