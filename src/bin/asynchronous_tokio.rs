use std::time::{Duration, Instant};
use tokio::task;
use tokio::time::delay_for;

async fn do_task(name: &str, delay_secs: u64) {
    println!("{} execute", name);
    delay_for(Duration::from_secs(delay_secs)).await;
    println!("{} done", name);
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let start = Instant::now();

    let task1 = task::spawn(async {
        do_task("task1", 2).await;
    });
    let task2 = task::spawn(async {
        do_task("task2", 10).await;
    });
    let task3 = task::spawn(async {
        do_task("task3", 5).await;
    });

    let _ = tokio::join!(task1, task2, task3);

    println!("see you");

    let end = Instant::now();
    println!("elapsed: {:?}", end - start);
}
