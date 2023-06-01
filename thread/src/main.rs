use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    println!("---- simple_case ----");
    simple_case();
    println!("\n\n");

    println!("---- channel_single_producer ----");
    channel_single_producer();
    println!("\n\n");

    println!("---- channel_multiple_producers ----");
    channel_multiple_producers();
    println!("\n\n");

    println!("---- simple_mutex ----");
    simple_mutex();
    println!("\n\n");

    println!("---- shared_mutex_between_multiple_threads ----");
    shared_mutex_between_multiple_threads();
    println!("\n\n");
}

fn simple_case() {
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

fn channel_single_producer() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for r in rx {
        println!("Got {}", r);
    }

    handle.join().unwrap();
}

fn channel_multiple_producers() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let handle1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for r in rx {
        println!("Got {}", r);
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn simple_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn shared_mutex_between_multiple_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
