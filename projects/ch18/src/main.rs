fn main() {
    println!("------------------------------------------");
    println!("Matching Named Variables");
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => {
            println!("Got 50");
        }
        Some(y) => {
            println!("Matched y = {y}");
        }
        _ => {
            println!("Default case: x = {:?}", x);
        }
    }

    println!("At the end:");
    println!("x = {:?}", x);
    println!("y = {y}");

    println!("------------------------------------------");
    println!("Extra Conditionals with Match Guards");
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}
