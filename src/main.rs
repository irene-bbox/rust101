// Run code w/o returning useless warnings
#[allow(dead_code)]
#[allow(unused_variables)]

// Import libraries
mod variables;
mod stackheap;
mod controlflow;
mod structures;
mod pmatch;
use std::mem;


// This program executes all that is contained in the main() function, so include your code inside the function pls!
fn main()
{
  //variables::variables();
  //variables::operators();
  //variables::scope_and_shadowing();
  //variables::constants();
  //stackheap::stack_and_heap();
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
  structures::generics();
  //pmatch::pattern_matching();

}
