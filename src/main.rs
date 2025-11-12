use time::OffsetDateTime;

fn main() {
    let mclaren_load_time = get_load_time("https://www.mclaren.com");
    println!("McLaren Load Time: {}", mclaren_load_time);
}

fn get_load_time(website_url: &str) -> f64 {
    let start = OffsetDateTime::now_utc();
    let _ = reqwest::blocking::get(website_url);
    let end = OffsetDateTime::now_utc();
    let duration = end - start;
    return duration.as_seconds_f64();
}
