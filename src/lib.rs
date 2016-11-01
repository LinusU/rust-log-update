extern crate ansi_escapes;

use std::io::Error;
use std::io::Write;

/// Main struct that holds the state for one Write stream
pub struct LogUpdate<W: Write> {
    stream: W,
    previous_line_count: u16,
    cursor_is_hidden: bool,
}

impl<W: Write> LogUpdate<W> {
    /// Create a new LogUpdate instance.
    pub fn new(mut stream: W) -> Result<Self, Error> {
        try!(write!(stream, "{}", ansi_escapes::CursorHide));
        try!(stream.flush());

        Ok(LogUpdate { stream: stream, previous_line_count: 0, cursor_is_hidden: true })
    }

    /// Update the log to the provided text.
    pub fn render(&mut self, text: &str) -> Result<(), Error> {
        try!(write!(self.stream, "{}{}\n", ansi_escapes::EraseLines(self.previous_line_count), text));
        try!(self.stream.flush());

        self.previous_line_count = text.chars().filter(|x| *x == '\n').count() as u16 + 2;

        Ok(())
    }

    /// Clear the logged output.
    pub fn clear(&mut self) -> Result<(), Error> {
        try!(write!(self.stream, "{}", ansi_escapes::EraseLines(self.previous_line_count)));
        try!(self.stream.flush());

        self.previous_line_count = 0;

        Ok(())
    }

    /// Persist the logged output.
    /// Useful if you want to start a new log session below the current one.
    pub fn done(&mut self) -> Result<(), Error> {
        if self.cursor_is_hidden {
            try!(write!(self.stream, "{}", ansi_escapes::CursorShow));
            try!(self.stream.flush());
        }

        self.previous_line_count = 0;
        self.cursor_is_hidden = false;

        Ok(())
    }
}

impl<W: Write> Drop for LogUpdate<W> {
    fn drop(&mut self) {
        if self.cursor_is_hidden {
            write!(self.stream, "{}", ansi_escapes::CursorShow).unwrap();
            self.stream.flush().unwrap();
        }
    }
}

#[cfg(test)]
extern crate tempfile;

#[cfg(test)]
mod tests {
    use tempfile::tempfile;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};

    use super::LogUpdate;

    #[test]
    fn it_handles_most_common_use_case() {
        let mut file: File = tempfile().unwrap();
        let mut log_update = LogUpdate::new(file.try_clone().unwrap()).unwrap();

        log_update.render("test1").unwrap();
        log_update.render("test2").unwrap();

        log_update.done().unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        assert_eq!(concat!("\x1B[?25l", "test1\n", "\x1B[1000D\x1B[K\x1B[1A\x1B[1000D\x1B[K", "test2\n", "\x1B[?25h"), buf);
    }

    #[test]
    fn it_restores_cursor_when_dropped() {
        let mut file: File = tempfile().unwrap();
        let mut log_update = LogUpdate::new(file.try_clone().unwrap()).unwrap();

        log_update.render("test").unwrap();

        drop(log_update);

        file.seek(SeekFrom::Start(0)).unwrap();

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        assert_eq!(concat!("\x1B[?25l", "test\n", "\x1B[?25h"), buf);
    }
}
