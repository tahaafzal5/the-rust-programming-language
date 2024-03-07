use aggregate::{Summary, Tweet};

fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for value in list {
        if value > largest {
            largest = value;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointMultipleType<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointMultipleType<X1, Y1> {
    fn mix_points<X2, Y2>(self, other: PointMultipleType<X2, Y2>) -> PointMultipleType<X1, Y2> {
        PointMultipleType {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("---------------------------------------------");
    println!("In Function Definitions");

    let nums = vec![0, -113, 33, 13];
    let largest = find_largest(&nums);
    println!("largest in the list is {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = find_largest(&char_list);
    println!("The largest char is {largest}");

    println!("---------------------------------------------");
    println!("In Struct Definitions");
    let int_point = Point { x: 3, y: 5 };
    let float_point = Point { x: 3.0, y: 5.0 };
    let multiple_type_point = PointMultipleType { x: 3, y: 5.0 };

    println!("---------------------------------------------");
    println!("In Method Definitions");
    println!("{}", int_point.x());

    let p1 = PointMultipleType { x: 5, y: 10.4 };
    let p2 = PointMultipleType { x: "Hello", y: 'c' };
    let p3 = p1.mix_points(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    println!("---------------------------------------------");
    println!("Traits: Defining Shared Behavior");

    let tweet = Tweet {
        usename: String::from("p.pon"),
        content: String::from("sup!"),
        reply: false,
        retweet: true,
    };
    println!("tweet summary: {}", tweet.summarize())
}
