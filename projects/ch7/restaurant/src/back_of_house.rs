// Situation where a chef fixes an incorrect order and personally brings it out
// to the table
fn fix_incorrect_order() {
    cook_order();
    // `fix_incorrect_order()` is in the `back_of_house` module so we can use
    // `super::` to get to `back_of_house`'s parent module, which in this case is
    // `crate`, the root crate.
    super::deliver_order();
}

fn cook_order() {}

pub struct Breakfast {
    // customers can choose what toast they want
    pub toast: String,
    // only chef decides which fruit to give based on what's available
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}
