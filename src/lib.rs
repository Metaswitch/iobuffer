//! IoBuffer library
//!
//! Copyright 2017 Metaswitch Networks
//!
//! Memory-based buffer which implements Write and Read traits.

use std::default::Default;
use std::result::Result::Ok;
use std::sync::{Arc, Mutex};
use std::io;
use std::cmp;

/// Simple object which implements both `std::io::Write` and `std::io::Read`.
/// Uses an internal buffer. Thread-safe and cloneable.
#[derive(Clone)]
pub struct IoBuffer {
    /// The actual shared contents of the buffer.
    inner: Arc<Mutex<IoBufferInner>>,
}

/// A simple read/write buffer.
struct IoBufferInner {
    /// All content that has been written so far.
    buf: Vec<u8>,

    /// The current reading cursor.
    pos: usize,
}

impl IoBuffer {
    /// Create a new empty buffer.
    pub fn new() -> Self {
        IoBuffer {
            inner: Arc::new(Mutex::new(IoBufferInner {
                buf: vec![],
                pos: 0,
            })), // LCOV_EXCL_LINE kcov bug?
        }
    }
}

impl Default for IoBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl io::Write for IoBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Trivial implementation - add all the data onto the end.
        let mut lock = self.inner.lock().expect("lock poisoned");
        lock.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // Nothing to do.
        Ok(())
    }
}

impl io::Read for IoBuffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Trivial implementation - read all the available data that can fit, and
        // advance the cursor.
        let mut lock = self.inner.lock().expect("lock poisoned");
        let len = cmp::min(lock.buf.len() - lock.pos, buf.len());
        let pos = lock.pos;
        buf[0..len].copy_from_slice(&lock.buf[pos..pos + len]);
        lock.pos += len;
        Ok(len)
    }
}

mod tests {
    #![allow(unused_imports)] // these are required, but there's a warning for some reason
    use super::IoBuffer;
    use std::io::{Read, Write};

    #[test]
    fn test_simple_buffer_usage() {
        let s1 = "This is some unexciting test data";
        let s2 = "This is some more unexciting test data";

        let mut buf = IoBuffer::new();

        let mut dest = Vec::new();
        let rc = buf.read_to_end(&mut dest).unwrap();
        assert!(rc == 0, "{}", rc);

        write!(buf, "{}", s1).unwrap();
        write!(buf, "{}", s2).unwrap();
        buf.flush().unwrap();

        let rc = buf.read_to_end(&mut dest).unwrap();
        assert!(
            rc == s1.len() + s2.len(),
            "{} != {}",
            rc,
            s1.len() + s2.len()
        );
        let s_out = String::from_utf8(dest).unwrap();
        assert!(s_out == (s1.to_string() + s2), "{}", s_out);
    }
}
