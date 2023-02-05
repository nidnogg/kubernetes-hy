// use axum_server::Handle;
use std::thread::sleep;
use std::time::Duration;
use time;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// This app prints out a randomly generated
/// `String` accompanied by a timestamp every 5 seconds
fn main() {
    main_cycle()
}

fn main_cycle() {
    let random_string = random_string(10);
    loop {
        let timestamp = time::OffsetDateTime::now_utc();
        // Sleep every 5 seconds
        sleep(Duration::new(5, 0));
        println!("{}: {}", timestamp, random_string);
    }
}

fn random_string(n: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect();
    rand_string
}
