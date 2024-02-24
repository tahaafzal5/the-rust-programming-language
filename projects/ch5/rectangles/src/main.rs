fn main() {
    let width = 30;
    let height = 50;

    let area = area_simple(width, height);
    println!("The area from area_simple is {area}");

    println!("-------------------------------------------------");
    println!("Refactored with tuples");
    let rect1 = (30, 50);
    let area = area_tuples(rect1);
    println!("The area from area_tuples is {area}");

    println!("-------------------------------------------------");
    println!("Refactored with Structs");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let area = area_struct(&rect1);
    println!("The area from area_struct for {rect1:?} is {area}");
    println!("{rect1:#?}");
    dbg!({ &rect1 });

    println!("-------------------------------------------------");
    println!("Methods");
    let area = rect1.area_method();
    println!("The area from area_method() is {area}");

    println!("-------------------------------------------------");
    println!("Methods with more parameters");
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("-------------------------------------------------");
    println!("Associated functions");
    let square = Rectangle::square(9);
    println!(
        "The area from area_method for {square:?} is {}",
        square.area_method()
    );
}

fn area_simple(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

impl Rectangle {
    fn area_method(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
