use add_one;
use add_two;

fn main() {
    let mut num = 10;
    println!("After added 1 to 10: {}", add_one::add_one(num));
    println!("After added 2 to 11: {}", add_two::add_two(num));
}
