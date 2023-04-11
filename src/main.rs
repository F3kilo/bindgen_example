use std::{
    thread,
    time::{Duration, Instant},
};

mod time;

fn main() {
    println!("Thread sleeps:");
    for _ in 0..5 {
        thread::sleep(Duration::from_secs(1));
        let ts = unsafe { time::clock() };
        println!("Timestamp: {ts};");
    }

    println!("Thread works:");
    for _ in 0..5 {
        let instant = Instant::now();
        while Instant::now() < instant + Duration::from_secs(1) {}

        let ts = unsafe { time::clock() };
        println!("Timestamp: {ts};");
    }
}
