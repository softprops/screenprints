//! # Screenprints
//!
//! Screensprints is a terminal interface tool.
use std::io::{Result, Write};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

mod os;

enum Op {
    Write(Vec<u8>),
    Clear,
    Flush,
    Close,
}

/// A Printer buffers writes and flushes at
/// a specified interval, clearing the display of any
/// lines of text previously written
pub struct Printer {
    writes: Arc<Mutex<Sender<Op>>>,
}

impl Printer {
    /// Creates a new Printer instance that delegates writes to the provided
    /// Write instance delayed at the interval provided
    pub fn new<W>(mut underlying: W, interval: Duration) -> Printer
        where W: Write + Send + 'static
    {
        // write op signals
        let (writer, writes) = channel();
        let writers = Arc::new(Mutex::new(writer));
        let sleeper = writers.clone();
        let forwards = writers.clone();
        let printer = Printer { writes: forwards };

        // inter thread close signals
        let (closer, closing) = channel();

        thread::spawn(move || {
            loop {
                if let Ok(_) = closing.try_recv() {
                    return;
                }
                thread::sleep(interval);
                if let Ok(s) = sleeper.lock() {
                    let _ = s.send(Op::Flush);
                }
            }
        });

        thread::spawn(move || {
            let mut buffer = vec![];
            let mut lines = 0;
            loop {
                if let Ok(op) = writes.recv() {
                    match op {
                        Op::Clear => {
                            buffer.clear();
                            lines = 0
                        }
                        Op::Close => {
                            let _ = closer.send(());
                            return;
                        }
                        Op::Flush => {
                            if buffer.is_empty() {
                                continue;
                            }
                            // clear lines
                            for _ in 0..lines {
                                os::clear_line_move_one_up(&mut underlying);
                            }
                            lines = buffer.iter().filter(|&b| *b == b'\n').count();

                            let _ = underlying.write(&buffer);
                            let _ = underlying.flush();
                            buffer.clear();
                        }
                        Op::Write(data) => buffer.extend(data),
                    }
                }
            }
        });
        printer
    }

    /// clear the current buffer and reset linecount
    pub fn clear(&self) {
        let _ = self.writes.lock().unwrap().send(Op::Clear);
    }

    /// close the printer, afterwhich all writes will be discarded
    pub fn close(&self) {
        let _ = self.writes.lock().unwrap().send(Op::Close);
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

impl Drop for Printer {
    fn drop(&mut self) {
        self.close();
    }
}

#[test]
fn it_works() {}
