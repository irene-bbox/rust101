// Run code w/o returning useless warnings
#[allow(dead_code)]
#[allow(unused_variables)]

// Import libraries
use std::mem;


// The program executes all that is contained in the main() function, so include your code inside the function pls!
fn main()
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

