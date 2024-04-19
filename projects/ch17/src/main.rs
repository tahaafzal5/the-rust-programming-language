use std::{fmt::Pointer, vec};

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);

        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        self.average = self.sum() as f64 / self.list.len() as f64
    }

    fn sum(&self) -> i32 {
        self.list.iter().sum()
    }
}

// -------------------------------------------

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for compnent in self.components.iter() {
            compnent.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    length: u32,
    width: u32,
    label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("Drawing {:?}", self);
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing {:?}", self);
    }
}

// --------------------------------------------------------------------------------------

trait State {
    // Box<Self> means the method is only valid when called
    // on a Box holding the type.
    // This syntax takes ownership of Box<Self>, invalidating the old state so
    // the state value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn add_text(&self, post: &mut Post, text: &str) {}
}

struct Draft {}
struct PendingReview {}
struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, post: &mut Post, text: &str) {
        post.content.push_str(text)
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::from(""),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(state) = self.state.take() {
            state.add_text(self, text);

            self.state = Some(state);
        }
    }

    // A Post can delegate to a content method defined on its state
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve())
        }
    }
}

// --------------------------------------------------------------------------------------

pub struct Post2 {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post2 {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post2 {
        Post2 {
            content: self.content,
        }
    }
}

fn main() {
    println!("-------------------------------------------");
    println!("Implementing the Trait");

    let screen = Screen {
        components: vec![
            Box::new(Button {
                length: 30,
                width: 10,
                label: String::from("Click Me!"),
            }),
            Box::new(SelectBox {
                width: 30,
                height: 10,
                options: vec![String::from("1"), String::from("2")],
            }),
        ],
    };

    screen.run();

    println!("-------------------------------------------");
    println!("Implementing an Object-Oriented Design Pattern");

    let mut post = Post::new();

    post.add_text("I ate a salad today");
    assert_eq!("", post.content());
    println!("Post created in draft");

    post.request_review();
    assert_eq!("", post.content());
    println!("Requested review on Post");

    // Post only is "published" after it is approved
    post.approve();
    assert_eq!("I ate a salad today", post.content());
    println!("Post approved");

    println!("-------------------------------------------");
    println!("Encoding States and Behavior as Types");
    let mut post2 = Post2::new();

    post2.add_text("I ate a salad today");
    println!("Post2 created in draft");

    let post2 = post2.request_review();
    println!("Requested review on Post2");

    let post2 = post2.approve();
    assert_eq!("I ate a salad today", post2.content());
    println!("Post2 approved");
}
