use aggregate::{notify, NewsArticle, Summary, Suspense, Tweet};

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

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
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
    println!("1 new tweet: {}", tweet.summarize());

    println!("---------------------------------------------");
    println!("Default Implementations");
    println!("Suspense with default implementation: {}", tweet.suspense());

    println!("---------------------------------------------");
    println!("Traits as Parameters");
    let news = NewsArticle {
        headline: String::from("This just in"),
        location: String::from("Sioux Falls, SD"),
        author: String::from("tahaafzal5"),
        content: String::from("He is learning Rust!"),
    };
    notify(&news);

    println!("---------------------------------------------");
    println!("Preventing Dangling References with Lifetimes");
    // won't compile because the value r is referring to (x),
    // goes out of scope before we try to use it
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {r}");

    println!("---------------------------------------------");
    println!("Generic Lifetimes in Functions");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // this example works b/c the values adhere to the constraints of the lifetimes
    let string1 = String::from("dkaofjhf");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // this example won't compile cause string2 doesn't live as long as string1
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    println!("---------------------------------------------");
    println!("Lifetime Annotations in Struct Definitions");
    let novel = String::from("Call me Taha. Once upon a time...");
    let first_sentence = novel.split('.').next().expect("Could not find '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i.part);

    println!("---------------------------------------------");
    println!("Lifetime Elision");
    
}
