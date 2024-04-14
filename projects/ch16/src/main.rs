use std::thread;
use std::time::Duration;

fn main() {
    println!("---------------------------------------");
    println!("Creating a New Thread with spawn");
    // let thread1 = thread::spawn(|| {
    //     for i in 1..100 {
    //         println!("{} from spawned area", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // thread1.join().unwrap();

    // for i in 1..10 {
    //     println!("{} from main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    println!("---------------------------------------");
    println!("Using move Closures with Threads");
    // let nums = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("The vector is: {:?}", nums);
    // });

    // handle.join().unwrap();

    println!("---------------------------------------");
    println!("Using Message Passing to Transfer Data Between Threads");
    println!("Sending Multiple Values and Seeing the Receiver Waiting");
    println!("Creating Multiple Producers by Cloning the Transmitter");
    // use std::sync::mpsc;

    // let (tx, rx) = mpsc::channel();

    // // Clone the transmitter so we can implement mpsc to have 2 threads that send to the
    // // same receiver.
    // let tx1 = tx.clone();
    // // Move the transmitting end into a spawned thread and have it send
    // // messages so that the spawned thread is communicating with the main thread
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("1st"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_millis(500));
    //     }
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("second"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_millis(500));
    //     }
    // });

    // for received in rx {
    //     println!("Received: {}", received);
    // }

    println!("---------------------------------------");
    println!("The API of Mutex<T>");
    // use std::sync::Mutex;

    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m: {:?}", m);

    println!("---------------------------------------");
    println!("Sharing a Mutex<T> Between Multiple Threads");
    println!("Multiple Ownership with Multiple Threads");
    println!("Atomic Reference Counting with Arc<T>");
    // use std::sync::{Arc, Mutex};

    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Counter: {}", *counter.lock().unwrap());
}
