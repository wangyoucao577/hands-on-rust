use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![0; 5];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from spwaned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        println!("vec: {:?}", v);
    });

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
