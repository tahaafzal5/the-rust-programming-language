enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Only gives us immutable access to data,
// but data can have multiple owners
use core::str;
use std::cell::RefCell;
use std::rc::Rc;
enum ListRC {
    Cons(i32, Rc<ListRC>),
    Nil,
}

#[derive(Debug)]
enum ListRCRefCell {
    Cons(Rc<RefCell<i32>>, Rc<ListRCRefCell>),
    Nil,
}

#[derive(Debug)]
enum ListReferenceCycle {
    Cons(i32, RefCell<Rc<ListReferenceCycle>>),
    Nil,
}

impl ListReferenceCycle {
    fn tail(&self) -> Option<&RefCell<Rc<ListReferenceCycle>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use crate::List::{Cons, Nil};
use crate::ListRC::{Cons as Cons2, Nil as Nil2};
use crate::ListRCRefCell::{Cons as Cons3, Nil as Nil3};
use crate::ListReferenceCycle::{Cons as Cons4, Nil as Nil4};

struct MyBox<T>(T);

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("ERROR: You are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("URGENT: You are at 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("WARNING: You are at 75% of your quota");
        }
    }
}

use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    println!("\nBox<T>");
    println!("---------------------------------------------");
    let b = Box::new(5);
    println!("b: {}", b);

    println!("\nUsing `Box<T>` to Get a Recursive Type with a Known Size");
    println!("---------------------------------------------");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("\nDefining Our Own Smart Pointer");
    println!("Implementing the Deref Trait");
    println!("---------------------------------------------");
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("\nRunning Code on Cleanup with the Drop Trait");
    println!("---------------------------------------------");
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    println!("\nUsing Rc<T> to Share Data");
    println!("---------------------------------------------");
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("Reference count after creating a: {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("Reference count after creating b: {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("Reference count after creating c: {}", Rc::strong_count(&a));
    }
    println!(
        "Reference count after c goes out of scope: {}",
        Rc::strong_count(&a)
    );

    println!("\nA Use Case for Interior Mutability: Mock Objects");
    println!("---------------------------------------------");
    println!("See Messenger, LimitTracker, and MockMessenger");

    println!("\nAllowing Multiple Owners of Mutable Data with Rc<T> and RefCell<T>");
    println!("---------------------------------------------");
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));
    let b = Cons3(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    println!("\nCreating a Reference Cycle");
    println!("---------------------------------------------");
    let a = Rc::new(Cons4(5, RefCell::new(Rc::new(Nil4))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons4(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());

    println!("\nCreating a Tree Data Structure: A Node with Child Nodes");
    println!("---------------------------------------------");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
