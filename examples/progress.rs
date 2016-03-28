extern crate screenprints;

use screenprints::Printer;
use std::io::{stdout, Write};
use std::time::Duration;
use std::thread;

fn main() {
    let mut p = Printer::new(stdout(), Duration::from_millis(10));
    for f in &["foo.txt", "bar.txt", "baz.txt"] {
        for i in 0..51 {
            let _ = write!(p, "Downloading {}.. ({}/{}) GB\n", f, i, 50);
            thread::sleep(Duration::from_millis(50));
        }
    //    p.clear();
    }
}
