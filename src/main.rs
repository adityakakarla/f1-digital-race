mod drivers;

use std::{
    sync::{Arc, Barrier, atomic::AtomicI8},
    thread::{self, JoinHandle},
};
use time::OffsetDateTime;

use crate::drivers::get_drivers;

fn main() {
    let drivers = get_drivers();
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let barrier = Arc::new(Barrier::new(drivers.len()));
    let counter = Arc::new(AtomicI8::new(1));

    for driver in drivers {
        let barrier = barrier.clone();
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            barrier.wait();
            let time = get_load_time(&driver.website_url);
            let rank = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if rank <= 9 {
                println!("P{:?}:  {} ({:.4})", rank, driver.short_name, time);
            } else {
                println!("P{:?}: {} ({:.4})", rank, driver.short_name, time);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn get_load_time(website_url: &str) -> f64 {
    let start = OffsetDateTime::now_utc();
    let _ = reqwest::blocking::get(website_url);
    let end = OffsetDateTime::now_utc();
    let duration = end - start;
    return duration.as_seconds_f64();
}
