use std::io::{Result, Write};
use std::thread;

pub struct Canvas<W> {
    inner: W,
    buffer: Vec<u8>,
    lines: usize
}

impl <W: Write> Canvas<W> {
    pub fn new(inner: W) -> Canvas<W> {
        Canvas {
            inner: inner,
            buffer: vec![],
            lines: 0
        }
    }

    pub fn start(&self) {

    }

    pub fn stop(&self) {

    }

    fn wait(&self) {

    }
}

impl <W: Write> Write for Canvas<W> {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.inner.write(data)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

#[test]
fn it_works() {
}
