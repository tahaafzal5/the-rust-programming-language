pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(num: i32) -> Guess {
        if num < 0 || num > 100 {
            panic!("Guess should be between 0 and 100 (inclusive), got {}", num);
        }

        Guess { value: num }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
