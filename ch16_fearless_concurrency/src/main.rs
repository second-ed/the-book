use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    example_interleaved_threads();
    closures_with_threads();
    message_passing();
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
