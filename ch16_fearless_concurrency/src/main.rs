use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    example_interleaved_threads();
    closures_with_threads();
    message_passing();
    using_mutexes();
}

fn example_interleaved_threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn closures_with_threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Heres a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("sending {val}");
        // once we send val it's owned by the other thread
        tx.send(val).unwrap();
    });

    let recieved = rx.recv().unwrap();
    println!("got {recieved}");

    thread::spawn(move || {
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

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("tx2"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("got: {recieved}");
    }
}

fn using_mutexes() {
    mutex_api();
    sharing_a_mutex();
}

fn mutex_api() {
    let m = Mutex::new(5);

    println!("m = {:?}", m);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn sharing_a_mutex() {
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

    println!("result: {}", *counter.lock().unwrap());
}
