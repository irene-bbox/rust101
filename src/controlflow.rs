#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

// import libraries

// declare a bunch of public functions
pub fn if_statement()
{
    let mut temp: i8 = 35;
    if temp > 30
    { println!("It's hot outside!"); }
    else if temp < 10
    { println!("It's cold outside"); }
    else
    { println!("It's neither hot nor cold outside"); }

    // Differently to some other programming languages, if statements in Rust can be expressions
    // Let's declare a string variable using an if statement
    let today = if temp > 20 {"is a warm"} else {"is a cold}"};
    println!("Today {} day", today);
    // Lesson learned: you can use if statements to return a value and assign that value to a variable


    // You can also use if statements inside the print line macro
    temp = 25;
    println!("Today is {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"moderate"});

    // Obviously, you can also have if statements inside of other if statements
    println!("Today is {}",
         if temp > 20
             {
               if temp <= 25 {"hot, bwt 20-25 C degrees"}
                 else {"hot, above 25 C degrees"}
             }
         else if temp < 10 {"cold"}
         else {"moderate"});
}


// The match statement is like the if statement which checks several cases at the same
// You can think of the match statement as a surjective function mapping the elements in the domain to all elements in the image (in a match statement, image and codomain coincide)
// Your match statement must be exhaustive: Rust is smart enough to force you to cover all possible cases, i.e. image=codomain must be the one and the some, otherwise it won't compute
pub fn match_statement()
{
    let country_code = 123;
    let country =  match country_code
    {
        44 => "UK",
        46 => "Sweden",
        50 => "Russia",
        51..=100 => "Unknown",   //[51,100] inclusive
        _ => "invalid" // _ means all other cases. It's like "else" for an if statement
    };
    println!("The country with code {} is {}", country_code, country);



    let i = true;
    let output = match i
    {
        true => "yes",
        false => "no"
    };
    println!("Does Santa Claus really exist? {}", output);
}



pub fn while_loop()
{
    let mut x: u8 = 1;
    while x < 100
    {
        x *= 2;
        if x == 32 { continue;} //the 'continue' command instructs the program not to execute the current line of code, but to just straight to the next line instead
        println!("x = {}", x);
    }

    // In Rust, there's an alternative to a while loop which is simply called 'loop'
    // 'loop' is technically equivalent to 'while true' in other programming languages and it refers to that type of loop which continues to execute unless you break out of it
    // How do you break out of it? Using conditional statements, i.e., run the loop until a condition is met. Then break!
    let mut y = 1;
    loop // while loop
    {
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 { break; } //if y is equal to 2^10=1024
    }
}


pub fn for_loop()
{
    for z in 1..11 //for x in [1,10] inclusive
    {
        if z == 3 {continue;} //if z is 3, skip and continue
        if z == 1<<3 {break;} //if z is equal to 2^3=8 which is 1000 in binary, break
        println!("z = {}", z);
    }


    // Let's practise with a another for loop printing both the index and the value of the element
    for (index, w) in (20..30).enumerate()
    {
        println!("{}:{}", index, w);
    }
}



// Simulate your attempt to open a permutation locker
enum State { locked, failed, unlocked}  //enumerate the states the locker can be in as you first interact with it

pub fn permut_locker()
{
    let combo = String::from("1234");
    let mut state = State::locked;
    let mut attempt = String::new();
    println!("The construction of a permutation lock is to be continued...");
}