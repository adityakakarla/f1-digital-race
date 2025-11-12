use std::{
    sync::{Arc, Barrier, Mutex, atomic::AtomicI8},
    thread::{self, JoinHandle},
};
use time::OffsetDateTime;

fn main() {
    let urls = vec![
        "https://www.landonorris.com",
        "https://www.oscarpiastri.com",
        "https://www.verstappen.com",
        "https://www.georgerussell63.com",
        "https://charlesleclerc.com/en/",
        "https://www.lewishamilton.com",
        "https://www.mercedesamgf1.com/drivers/driver/andrea-kimi-antonelli",
        "https://www.alexalbon.com",
        "https://www.nicohulkenberg.net",
        "https://www.isackhadjar.com",
        "https://www.olliebearman.com",
        "https://www.fernandoalonso.com",
        "https://www.carlossainz.es/en/",
        "https://www.liamlawson30.com",
        "https://www.lancestroll.com",
        "https://www.esteban-ocon.com",
        "https://www.yukitsunoda.com",
        "https://www.pierregasly.com/en/",
        "https://www.gabrielbortoleto.com.br/en/",
        "https://www.francolapinto.com",
    ];
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let barrier = Arc::new(Barrier::new(urls.len()));
    let counter = Arc::new(AtomicI8::new(1));

    for url in urls {
        let barrier = barrier.clone();
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            barrier.wait();
            let time = get_load_time(url);
            let rank = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            println!("P{:?}: {} ({})", rank, url, time);
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
