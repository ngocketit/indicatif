use console::{style, Emoji};
use indicatif::{HumanDuration, ProgressBar};
use std::thread;
use std::time::{Duration, Instant};

static SUCCESS: Emoji<'_, '_> = Emoji("✨ ", ":-)");

fn main() {
    let count = 200;
    let timer = Instant::now();
    let pb = ProgressBar::new(count);
    pb.set_prefix(style("[1/1]").bold().dim().to_string());
    for _ in 0..count {
        pb.inc(1);
        thread::sleep(Duration::from_millis(10));
    }
    pb.finish_and_clear();
    println!("{} Done in {}", SUCCESS, HumanDuration(timer.elapsed()));
}
