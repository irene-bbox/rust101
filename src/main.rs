// Run code w/o returning useless warnings
#[allow(dead_code)]
#[allow(unused_variables)]

// Import libraries
mod stackheap;
use std::mem;

// In Rust there exist such a thing as GLOBAL variables, which are declared outside of the main function
// Global variables can be mathematical constants or can be user-defined
// Global variables can be mutable. If so, declare the scope to be 'unsafe'. If you run a mutable global variable outside of an 'unsafe' scope, Rust returns an error
// GLOBAL VARIABLES
const MEANING_OF_LIFE:u8 = 42;
static MY_CONSTANT:i8 = -42;
static mut UNSAFE:i8 = 17;




// This program executes all that is contained in the main() function, so include your code inside the function pls!
fn main()
{
  //variables();
  //operators();
  //scope_and_shadowing();
  //constants();
  stackheap::stack_and_heap();
}


// VARIABLES
fn variables()
{
  // VARIABLES TYPES: u8, u16, u32, u64, u128, i8, i16, ...
  let a: u8 = 100;   // This is a variable declaration for a, which is now stored in memory as an u=unsigned, 8 bits, 0-255 number
  println!("a = {}", a);   // a in now an immutable
  // Beware all variables which are declared in Rust are IMMUTABLE variables.
  // Mutable variables must be otherwise specified to be mutable at the time of variable declaration


  // u = unsigned, with range 0 ... 2^N-1, e.g. 0 to 2^8-1=256-1=255, that is [0,255]
  // i = signed, with halved range -(2^N-1)/2 ... +(2^N-1)/2, e.g. -128 to 127, that is [-128,127]
  let mut b: i8 = 0;
  println!("Initiallly, b = {}", b);
  b = 42;
  println!("But after, b = {}", b);

  // Rust can do type inference, i.e. it can infer the type once you declare a variable
  let mut c = 1234567; // i32 = 32 bits = 4 bytes
  println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
  c = -1;
  println!("New variable c = {}", c);

 // VARIABLE TYPES: usize and isize
  let d: isize = 123;
  let size_of_d = mem::size_of_val(&d);
  println!("d = {}, takes up {} bytes, {}-bit OS", d, size_of_d, size_of_d*8);


  // VARIABLE TYPES: single characters
  let e: char = 'x';
  // because we haven't specified mutability, this variable is immutable by default
  // to define a single character, use single quotes
  // a character or 'char' type variables isn't just a single letter, but it could also e punctuation, etc.
  // you will soon find out in the print statement below that a 'char' type variable takes up 32 bits or memory
  println!("{} is a character, size = {} bytes", e, mem::size_of_val(&e));


  // VARIABLE TYPES: f32, f64, IEEE754 floating point numbers, which are non-whole numbers
  // There is no signed/unsigned distinction for floating point numbers !
  // The default size for floating point numbers is f64
  let mut f: f32 = -2.5;
  println!("f = {}, takes up {} bytes", f, mem::size_of_val(&f));
  f = 7.78;
  println!("New variable f = {}", f);

  // VARIABLE TYPES: Booleans
  let g: bool = false; // true or false
  println!("f = {}, takes up {} bytes", g, mem::size_of_val(&g));
  // Although 1 bit is sufficient for the representation of a Boolean, it still takes up 1 bytes, using 1 bit and wasting the 7 others.
  // Too bad, but this is the standard memory storage setting for Rust
}



fn operators()
{ //ARITHMETIC operators
  let mut a = (72/2-16)*5;
  println!("a = {}", a);
  a = a+1;
  a -= 90; // a = a-90
  // Rust operators are -, +, *, /, % but also -=, +=, *=, /=, %=
  // the % operator gives you the remainder of division
  println!("Remainder of {}/{} = {}", a, 3, (a%3));

  let a_cubed = i32::pow(a, 3);
  println!("{} cubed is {}", a, a_cubed);

  let b = 2.5;
  let b_cubed = f64::powi(b, 3);
  let b_to_pi = f64::powf(b, std::f64::consts::PI); // Constants are always written in capital in Rust
  println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

  // BITWISE operators
  // which in Rust are only available for integers. Here is a list of operators
  // | means OR
  // & means AND
  // ^ means XOR =  exclusive or
  // ! means NOR = NEGATION operator
  // << means shift to the left, by padding with zeros
  // >> means shift to the right,     "     "      "
  let c = 1|2;  // 10 OR 01 = 11 == 3
  println!("1|2 = {}", c);

  let two_to_10 = 1 << 10;  // Here we are saying that 2^10 in binary is 100000000000, b/c we have that in binary code 2=10, 2^2=4=100, 2^3=8=1000, and so on
  println!("2^10 = {}", two_to_10);

  // LOGICAL operators
  // List of logical operators: > >= < <= ==
  let pi_less_4 = (std::f64::consts::PI) < 4.0;
  println!("Is pi < 4? {}", pi_less_4);
  let pi_is_314 = (std::f64::consts::PI) == 3.14;
  println!("Is pi = 3.14? {}", pi_is_314);
  let x = 5;
  let x_is_5 = x == 5;
  println!("{}", x_is_5);
}


// SCOPE AND SHADOWING
// Notice that every variable has a scope and it only exists within the scope of the function where the variable is defined
// i.e., if you define variable a inside function variables{} and then you only execute the function main(), the compiler returns an error
// A scope in Rust is defined by opening and closing curly braces {    }

fn scope_and_shadowing()
{ let a = 1;

  {
    let b= 2;
    println!("inside scope, b={}", b);
    let a = 7;
    println!("inside scope, a={}", a);
  }

  println!("a = {}", a);
  //println!("outside scope, b={}", b); This line does NOT run, b/c it's out of scope
}


fn constants()
{
  println!("const = {}, static = {}", MEANING_OF_LIFE, MY_CONSTANT);

  unsafe
  {
    println!("Mutable global variable = {}", UNSAFE);
    UNSAFE += 2;
    println!("Mutable global variable after mutation = {}", UNSAFE);
  }
}