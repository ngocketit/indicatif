use console::style;
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;

fn main() {
    let count = 200;
    let pb = ProgressBar::new(count);
    pb.set_prefix(style("[1/1]").bold().dim().to_string());
    for _ in 0..count {
        pb.inc(1);
        thread::sleep(Duration::from_millis(100));
    }
    pb.finish_and_clear();
}
