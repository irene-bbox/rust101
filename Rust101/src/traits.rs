#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_must_use)]

// Import libraries
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Neg}; // add 'addition' from the operation module for Rust

// TRAITS
// A trait is a group of methods that can be applied to a structure


// define a trait
trait Animal
{
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) { println!("{} can do math", self.name()); }
}

// define a struct
struct Human { name: &'static str }
struct Cat { name: &'static str }

// define an impl for your struct
impl Animal for Human { fn create(name: &'static str) -> Human { Human{name:name} }
                        fn name(&self) -> &'static str { self.name }  }
impl Animal for Cat { fn create(name: &'static str) -> Cat { Cat{name:name} }
                        fn name(&self) -> &'static str { self.name } }

// a summable for a vector
trait Summable<T> { fn sum(&self) -> T; }
impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut sum:i32 = 0;
        for i in self { sum += *i; }
        return sum;
    }
}


// declare function
pub fn RustTrait()
{
    //let man = Human{name:"Gottfried Wilhelm Leibniz"};
    let man = Human::create("Leonard Euler");
    let man:Human = Animal::create("Johannes Kepler"); // this line of code is amazing. Ponder over it...
    // and notice it's equivalent to the line of code above. Yet, it calls on 'Aminal' instead of 'Human'
    man.talk();
    //let cat = Cat{name:"Chubby Howard"};
    let cat = Cat::create("Squirrel");
    cat.talk();


    // create a trait to calculate the sum of all elements in a vector
    let vec = vec![1,2,3,4,5,6];
    println!("sum of vec = {}", vec.sum());
}


// Write functions that take traits as parameters
#[derive(Debug)]
struct Circle{radius:f64, }
#[derive(Debug)]
struct Square{side:f64, }

trait Shape { fn area(&self) -> f64; }

impl Shape for Square { fn area(&self) -> f64 { self.side*self.side } }
impl Shape for Circle { fn area(&self) -> f64 { self.radius*self.radius*std::f64::consts::PI } }

//  ∃ distinct ways to specify that the parameter of this function is a trait

//1st way 'arg: impl NameOfTrait'
fn print_info(shape: impl Shape)
{ println!("The area is = {}", shape.area()) }
fn print_info0(shape: impl Shape + Debug)
{ println!("{:?}", shape); println!("The area is = {}", shape.area()); }

//2nd way: the trait bound syntax. Super useful when you want to implement multiple traits on multiples parameters
//fn print_info2<T: Shape + Debug>(shape1:T, shape2:T) { println!("{:?}", shape); println!("The area is = {}", shape.area()); }

//3nd way: specify the traits to be used after the 'where' statement
fn print_info3<T>(shape: T)
    where T: Shape + Debug
{ println!("{:?}", shape); println!("The area is = {}", shape.area()); }


pub fn TraitParameter()
{
    let c = Circle{radius:2.0};
    print_info(c);
    let s = Square{side:3.0};
    print_info0(s);
    let q = Circle{radius:4.0};
    print_info3(q);
}


// INTO
// The 'into' trait allows automatic conversion where possible
struct Person{name:String}

impl Person
{
    // (1)
    // fn new1(name: &str) -> Person
    // {
    //     Person {name: name.to_string() }
    // }

    // (2)
    // define the new() function as taking in an S-type parameter that is convertible into a String
    fn new2<S: Into<String>>(name: S) -> Person  // 'S' is the genetic type, and Into<String> is the trait that the generic has to implement
    { Person { name: name.into() } }

    // (3)
    fn new3<S>(name: S) -> Person
        where S: Into<String>
    {
        Person { name: name.into() }
    }
}

pub fn into()
{
    let mathematician:String = "Imre Lakatos".to_string();
    let p =  Person::new3(mathematician);
}





// DROP
// 'Drop' is a trait that destroys variables. It's also known as 'destructor' in other programming languages
struct Creature{name:String}

impl Creature
{
    fn new(name: &str) -> Creature
    {
        println!("{} enters the game", name);
        Creature { name: name.into()}
    }
}

impl Drop for Creature
{
    fn drop(&mut self)
    {
        println!("{} is dead", self.name);
    }
}

pub fn drop()
{
    let mut dragon = Creature::new("Gigi");
    println!("The fire dragon show game proceeds");
    // here, i.e. at the end of scope is where the drop function typically gets called to destroy the created variables.
    // If you don't to wait for the  drop() function to be implicitly at the end of scope, then call it deterministically.
    // Call drop() as a global function to impose deterministic finalization of the variable
    //drop(dragon); // once you've dropped a variable you can't call on it anymore!
}



#[derive(Debug)]
struct Complex<T> { re: T, im: T }

impl<T> Complex<T>
{
    fn new(re:T, im:T) -> Complex<T>
    {
        Complex::<T> {re, im} // the '::' here serves to instruct the compiler not to treas '<' as a 'less than' operator
    }
}


// a+b
impl<T> Add for Complex<T>
    where T: Add<Output = T> // this line of code says: 'the generic type T supports addition (i.e., the 'Add' trait) and its output is also of type T
{
    type Output = Complex<T>; // this is an associated type, i.e. a type I need to specify for a specific trait to work. In our case, this is the result type of the addition operation
    // a+b
    fn add(self, rhs: Self) -> Self::Output {
        Complex
        {
            re: self.re+rhs.re,
            im: self.im+rhs.im
        }
    }
}


// a+=b
impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    // a+=b
    fn add_assign(&mut self, rhs: Self)
    {
        self.re+=rhs.re;
        self.im+=rhs.im
    }

}

pub fn operatoroverload()
{
    let mut a = Complex::new(1.0,2.0);
    let mut b = Complex::new(4.0,5.0);
    // Now, let's perform an addition of 2 complex numbers using 'Add' which is a trait
    // println!("a+b = {:?}", a+b);
    a += b;
    println!("{:?}", a);
}

