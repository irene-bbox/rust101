// Run code w/o returning useless warnings
mod contractfeatures;
mod controlflow;
mod evenfibonacci;
mod function;
mod lessonslearned;
mod stdcollection;
mod string;
mod structures;
mod traits;

#[allow(dead_code)]
#[allow(unused_variables)]
// Import libraries
mod variables;
use std::mem;

// This program executes all that is contained in the main() function, so include your code inside the function pls!
fn main() {
    //variables::variables();
    //variables::operators();
    //variables::scope_and_shadowing();
    //variables::constants();
    //variables::stack_and_heap();
    //controlflow::if_statement();
    //controlflow::match_statement();
    //controlflow::while_loop();
    //controlflow::for_loop();
    //controlflow::permut_locker();
    //structures::structures();
    //structures::enums();
    //structures::unions();
    //structures::options();
    //structures::array();
    //structures::slices();
    //structures::tuples();
    //structures::generics();
    //structures::pattern_matching();
    // stdcollection::vectors();
    //stdcollection::hashmap();
    //stdcollection::hashset();
    //stdcollection::iterators();
    //string::strings();
    //function::funk();
    //function::methods();
    //function::closures();
    //function::high_order_fn();
    //traits::RustTrait();
    //traits::TraitParameter();
    //traits::into();
    //traits::drop();
    //traits::operatoroverload();
    //evenfibonacci::evenfib();
    //lessonslearned::summary();
    //contractfeatures::understand_semicolon();
    //contractfeatures::compare_eq();
    // contractfeatures::bytecode();
    // contractfeatures::hashing();
    //contractfeatures::hexa();
    // contractfeatures::storage();
    contractfeatures::query_state();
}
