
#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_must_use)]

// Import libraries
use rand::Rng;
use sdt::io::stdin;

// STRINGS
// There exist 2 object types in Rust for strings: 'str' and 'String'
// 'str' is a utf-8 rather static immutable object
// 'String' is a heap and more flexible object
pub fn strings()
{
    // str
    let yoda = "I am Yoda"; // &str = string slice
    let baby: &'static str = "Baby Yoda"; // &str = string slice
    // 'str' objects can't be sliced, e.g. yoda[0] or modified yoda="Yodaaa!"

    for x in baby.chars().rev() { println!("{}", x) };

    if let Some(first_letter) = baby.chars().nth(0) { println!("First letter = {}", first_letter) };


    // String
    // create a string with the letters of the alphabet
    let mut alphabet = String::new();
    let mut a= 'a' as u8;
    while a <= ('z' as u8)
    {
        alphabet.push(a as char);
        alphabet.push(',');
        a += 1;
    }
    println!("Alphabet = {}", alphabet);

    // Conversion between &str <> String
    // Make a &str from a String
    let alphy:&str = &alphabet;
    for i in alphy.chars() { println!("{}", i) };


    // Make a String from a &str
    // Here is a couple of different ways
    let mut babyyoda = String::from("Baby Yoda");
    let mut bb8 = "BB8".to_string();
    let mut friends= String::from("are friends");
    let sentence = babyyoda + " & " + &bb8 + " " + &friends;  // concatenate
    for i in sentence.chars() { println!("{}", i) };

    let mut phrase = String::from(sentence);
    println!("{}", phrase);
    phrase.push_str(" !!"); // append to a String
    phrase.remove(28); // remove characters from String by index
    phrase.replace("BB8", "R2"); // replace inside a String
    println!("{}", phrase.replace("BB8", "R2"));


    // format!()
    let surname = "Yoda";
    let intro =  format!("Hi, I'm {} and I'm a jedi.", surname);
    println!("{}", intro);

    let may = "May";
    let day = "Day";
    let mdmd = format!("{0}{1}, {0}{1}!", may, day);
    println!("{}", mdmd);

    let info = format!("My name is {last}. {first} {last}.", first = "James", last = "Bond");
    println!("{}", info);
    // you can also mix and match the format!() types
}


