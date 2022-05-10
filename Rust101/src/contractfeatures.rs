use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::{env, BorshStorageKey};
use std::any::type_name;
#[warn(unused_imports)] // disregard unused imports
use subtle::ConstantTimeEq;

// checks object type
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn compare_eq() {
    // Declare two slices
    let s1: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let s2: [u8; 8] = [0, 1, 2, 3, 0, 1, 2, 3];

    // use the ConstantTimeEq function ct_eq() to check for identity between 2 slices
    pub fn compare(s1: &[u8], s2: &[u8]) -> bool {
        bool::from(s1.ct_eq(s2))
    }

    compare(&s1, &s2);
}

pub fn bytecode() {
    // 2 equivalent ways to convert a string into a slice of bytecode
    let x = b"hello!";
    let y = "hello!".as_bytes();

    println!("'Hello!' in bytecode is = {:?} = {:?}", x, y);
}

pub fn understand_semicolon() {
    // To return a value from a function you have two options in Rust:

    // Use a return <some value>; expression
    fn returns_one() -> u8 {
        1
    } // returns the value '1' of type 'u8' without using a 'return'

    // End the function with an expression or a value without a semicolon
    let is_u8: u8 = { returns_one() };

    // When neither of these options is used i.e. the function block ends with a semicolon
    // like the functions below, then the value that is returned is () aka the unit type.
    // This value is also returned from functions that do not declare a return type like this one:
    fn returns_unit() {
        1i64;
    }
    returns_unit(); // will not return anything b/c it's a unit type

    let is_unit = {
        returns_one(); // will not return anything b/c it's a unit type, too
    };

    // Also: there's no need for semicolon at the end of a function (i.e., after declarign a function),
    // but you must use a semicolon after declaring a variable. Think of it as a linebreak.
    // Functions have already a scope that gets closed and acts as a linebreak, whereas
    // variable declarations need to come to an end via a semicolon!

    println!("is_u8 = {}", is_u8);
}

pub fn hashing() {
    //now hash a random sequence of bytes using sha256
    let b1 = "Yoda".as_bytes();
    assert_eq!(b"Yoda", b1, "Objects do not match");
    println!("The hash for 'Yoda' is = {:?}", env::sha256(b1));

    //hash an accountId and return it
    let mut hash: [i32; 6] = [0; 6];
    let o = env::sha256("Yo".as_bytes());
    println!("Obj is = {:?}, with length of {}", o, o.len());

    let accountId = [1, 2, 3, 4];
    hash[2..].copy_from_slice(&accountId);
    println!("Our newly created hash is {:?}", hash);
}

pub fn hexa() {
    // encrypt and decrypt into hexadecimal representation using the 'hex' crate
    let my_sentence = "happy birthday mayllon from bboxteam";
    let sentence_hash_bytes = env::sha256(my_sentence.as_bytes());
    let sentence_hash_string = hex::encode(sentence_hash_bytes);
    println!("my_sentence is: {:?}", my_sentence);
    println!(
        "its hexadecimal representation is: {:?}, with length {}",
        sentence_hash_string,
        sentence_hash_string.len()
    );
    println!(
        "Decrypting {:?} from hexadecimal into string representation yields {:?}",
        sentence_hash_string, my_sentence
    );

    // Return object type
    let my_var = env::log_str("Try again");
    println!("The object type is: {}", type_of(my_var));
}

// pub fn testing() {
//     pub struct Contract {
//         token_by_id: LookupMap<String, String>,
//     };

//     impl Contract {
//         pub fn nft_mint(&mut self, tokenid: Option<TokenId>) {
//             self.token_by_id.insert(&tokenid, "Token1");
//             assert!(
//                 self.token_by_id.insert(&tokenid, "Token2").is_none(),
//                 "Token already exists"
//             );
//         }
//     };

//     test = nft_mint("001");
//     test;
// }

pub fn storage() {
    let my_var: u16 = 144;

    let used_storage = env::storage_usage();
    println!("The storage used thus far is {}", used_storage);
}

// pub fn lookupsets() {
//     let a = b"s";
//     let b = a.to_vec();
//     // print object type
//     println!("A = {:?}, and B ={:?} of type = {}", a, b, type_of(a));

//     struct Contract {
//         scores: LookupSet<String>,
//     }

//     pub fn my_contract() {
//         let test = Contract {
//             scores: LookupSet::new(String::from("Baby Yoda")),
//         };
//         println!("A sample contract is this: {:?}", test.scores);
//     }
// }

pub fn query_state() {
    // Define a 'State' struct
    struct State {
        pub max_size: u64,
        pub user_count: u64,
        pub score_count: u64,
    };

    let x = State {
        max_size: 300000,
        user_count: 1,
        score_count: 1,
    };

    println!("A sample 'State' is = {:?}", x.max_size);
}
