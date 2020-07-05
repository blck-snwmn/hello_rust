use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender1, reciever1) = mpsc::channel::<i64>();
    let (sender2, reciever2) = mpsc::channel::<i64>();

    let handle1 = thread::spawn(move || {
        for i in 1..10 {
            match sender1.send(i) {
                Err(e) => println!("[thread1]failed to send {}({})", e, e.0),
                Ok(_) => {}
            }
            thread::sleep(Duration::from_secs(1));
            match reciever2.recv() {
                Err(e) => println!("[thread1]failed to recieve! {}", e),
                Ok(v) => println!("[thread1]recieved! {}", v),
            }
            thread::sleep(Duration::from_secs(1));
        }

        thread::sleep(Duration::from_secs(5));
    });
    let handle2 = thread::spawn(move || {
        for i in 11..20 {
            match sender2.send(i) {
                Err(e) => println!("[thread2]failed to send {}({})", e, e.0),
                Ok(_) => {}
            }
            thread::sleep(Duration::from_secs(1));
            match reciever1.recv() {
                Err(e) => println!("[thread2]failed to recieve! {}", e),
                Ok(v) => {
                    println!("[thread2]recieved! {}", v);
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
        thread::sleep(Duration::from_secs(5));
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
