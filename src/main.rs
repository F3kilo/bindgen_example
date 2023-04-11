use std::{
    thread,
    time::{Duration, Instant},
};

mod time;

fn main() {
    println!("Thread sleeps:");
    for _ in 0..5 {
        thread::sleep(Duration::from_secs(1));
        println!("Timestamp: {}s;", cpu_time().as_secs_f32());
    }

    println!("Thread works:");
    for _ in 0..5 {
        let instant = Instant::now();
        while Instant::now() < instant + Duration::from_secs(1) {}
        println!("Timestamp: {}s;", cpu_time().as_secs_f32());
    }
}

static mut INIT_CLOCK: i64 = 0;

fn cpu_time() -> Duration {
    unsafe {
        let clock = time::clock();
        if INIT_CLOCK == 0 {
            INIT_CLOCK = clock;
        }
        Duration::from_secs_f32((clock - INIT_CLOCK) as f32 / 1_000_000_f32)
    }
}
