use std::io::{Result, Write};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

enum Op {
    Write(Vec<u8>),
    Clear,
    Flush,
}

/// A Printer is a buffering write which flushes at
/// a specified interval, clearing the display of any
/// lines of text previously written
pub struct Printer {
    writes: Arc<Mutex<Sender<Op>>>,
}

impl Printer {
    /// Creates a new Printer instance that delegates writes to the provided
    /// Write instance delayed at the interval provided
    pub fn new<W>(mut writer: W, interval: Duration) -> Printer
        where W: Write + Send + 'static
    {
        let (tx, rx) = channel();
        let shared = Arc::new(Mutex::new(tx));
        let sleeper = shared.clone();
        let writes = shared.clone();
        let printer = Printer { writes: writes };

        thread::spawn(move || {
            loop {
                thread::sleep(interval);
                let _ = sleeper.lock().unwrap().send(Op::Flush);
            }
        });

        thread::spawn(move || {
            let mut buffer = vec![];
            let mut lines = 0;
            loop {
                match rx.recv() {
                    Ok(op) => {
                        match op {
                            Op::Write(data) => buffer.extend(data),
                            Op::Clear => {
                                buffer.clear();
                                lines = 0
                            }
                            Op::Flush => {
                                if buffer.is_empty() {
                                    continue;
                                }
                                // clear lines
                                for _ in 0..lines {
                                    let _ = write!(writer, "\x1B[0A"); // move the cursor up
                                    let _ = write!(writer, "\x1B[2K\r");  // Clear the line
                                }
                                lines = 0;
                                for b in buffer.iter() {
                                    if *b == ('\n' as u8) {
                                        lines += 1;
                                    }
                                }

                                let _ = writer.write(&buffer);
                                let _ = writer.flush();
                                buffer.clear();
                            }
                        }
                    }
                    _ => return,
                }
            }
        });
        printer
    }

    pub fn clear(&self) {
        let _ = self.writes.lock().unwrap().send(Op::Clear);
    }
}

impl Write for Printer {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        let _ = self.writes.lock().unwrap().send(Op::Write(data.to_owned()));
        Ok(data.len())
    }

    fn flush(&mut self) -> Result<()> {
        let _ = self.writes.lock().unwrap().send(Op::Flush);
        Ok(())
    }
}

#[test]
fn it_works() {}
