use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    // This code will print "Hello, Macro! My name is Pancakes! "
    Pancakes::hello_macro();
}
