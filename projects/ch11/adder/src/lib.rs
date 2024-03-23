#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than 100, got {}", value);
        }
        Guess { value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value: {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 7,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        let larger = Rectangle {
            width: 7,
            height: 10,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two_4() {
        assert_eq!(6, add_two(4));
    }

    #[test]
    fn it_adds_two_8() {
        assert_eq!(10, add_two(8));
    }

    #[test]
    #[ignore]
    fn it_adds_two_100() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn greet_carol() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name. Value was {result}"
        );
    }

    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two doesn't euqal 2 for you"))
        }
    }

    #[test]
    fn this_passes() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_fails() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(4, value);
    // }
}
