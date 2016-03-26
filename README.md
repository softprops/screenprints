# screenprints

[![Build Status](https://travis-ci.org/softprops/screenprints.svg?branch=master)](https://travis-ci.org/softprops/screenprints) [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE) ![crates.io](http://meritbadge.herokuapp.com/screenprints)

> reprints for your terminal screen


Screenprints acts as a buffer for terminal display continuously printing output at a configured interval.

## api docs

Find them [here](https://softprops.github.io/screenprints)

## usage

Screensprints defines a `Printer` which implements [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) which means anywhere you would normally write output to, you could substitude in an instance of a `Printer`.

```rust
extern crate screenprints;

use screenprints::Printer;
use std::io::{stdout, Write};
use std::time::Duration;
use std::thread;

fn main() {
    let mut printer = Printer::new(stdout(), Duration::from_millis(10));
    for f in vec!["foo.txt", "bar.txt", "baz.txt"] {
        for i in 0..51 {
            let _ = write!(printer, "Downloading {}.. ({}/{}) GB\n", f, i, 50);
            thread::sleep(Duration::from_millis(50));
        }
    }
}
```

The result should look something like the following

[![asciicast](https://asciinema.org/a/9auhm32umebr14bulaifhynni.png)](https://asciinema.org/a/9auhm32umebr14bulaifhynni)


Doug Tangren (softprops) 2016
