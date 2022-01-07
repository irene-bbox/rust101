#![allow(dead_code)]
use std::mem;

struct Point
{
    x: f64,
    y:f64
}


fn origin() -> Point
{
    Point{x:0.0, y:0.0}
}


pub fn stack_and_heap()
{
    let p1 = origin(); //this is a stack allocation of p1
    let p2 = Box::new(origin()); //this is a heap allocation of p2
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2; //where the * is the pointer to the Box where p2 is saved;
    println!("p3.x = {}", p3.x);
}