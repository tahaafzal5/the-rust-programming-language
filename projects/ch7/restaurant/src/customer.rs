use crate::back_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // Because we're using `use`
    hosting::add_to_waitlist();

    // Order a breakfat in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what break we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    // this won't compile since `seasonal_fruit` is private
    // meal.seasonal_fruit = String::from("mangoes");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
