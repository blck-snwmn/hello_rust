use std::{thread, time};

fn main() {
    let child = thread::spawn(move || {
        println!("child start");
        thread::sleep(time::Duration::from_secs(2));
        println!("child finish");
    });

    println!("parent start & finish");

    // Wait until child done.
    match child.join() {
        Ok(()) => println!("finish"),
        Err(e) => println!("err={:?}", e),
    }
}
