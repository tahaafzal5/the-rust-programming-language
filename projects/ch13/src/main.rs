use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_blue = 0;
        let mut num_red = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {
    println!("--------------------------------------------------");
    println!("Capturing the Environment with Closures");
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_preference_1 = Some(ShirtColor::Red);
    let giveaway_color_1 = store.giveaway(user_preference_1);
    println!(
        "User with preference {:?} gets {:?}",
        user_preference_1, giveaway_color_1
    );

    let user_preference_2 = None;
    let giveaway_color_2 = store.giveaway(user_preference_2);
    println!(
        "User with preference {:?} gets {:?}",
        user_preference_2, giveaway_color_2
    );

    println!("--------------------------------------------------");
    println!("Capturing References or Moving Ownership");
    println!("Immutable Reference");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    println!("Mutable Reference");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut mutably_borrows = || list.push(9);

    mutably_borrows();
    println!("After calling closure: {:?}", list);

    println!("Thread with `move`");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    println!("--------------------------------------------------");
    println!("Using Closures That Capture Their Environment");
    // look at shoes_in_size's filter
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
