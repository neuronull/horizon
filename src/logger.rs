use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use tracing_subscriber::fmt::{self, MakeWriter};
use tracing_subscriber::layer::SubscriberExt;

#[derive(Clone, Default)]
pub struct Logs {
    logs: Arc<Mutex<Vec<String>>>,
}

impl Logs {
    /// # Panics
    ///
    /// Will panic if unable to acquire the lock.
    #[must_use]
    pub fn take_all(&self) -> Vec<String> {
        let mut logs = self.logs.lock().unwrap();
        std::mem::take(&mut *logs)
    }
}

impl Write for Logs {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = std::str::from_utf8(buf).unwrap_or_default();

        if !s.trim().is_empty() {
            let mut logs = self.logs.lock().unwrap();
            logs.push(s.to_string());
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct LogWriter {
    writer: Logs,
}

impl LogWriter {
    pub fn new() -> (Self, Logs) {
        let writer = Logs::default();
        (
            Self {
                writer: writer.clone(),
            },
            writer,
        )
    }
}

impl<'a> MakeWriter<'a> for LogWriter {
    type Writer = Logs;

    fn make_writer(&'a self) -> Self::Writer {
        self.writer.clone()
    }
}

/// # Panics
///
/// Will panic if unable to setup subscriber.
#[must_use]
pub fn setup_logging() -> Logs {
    let (make_writer, writer) = LogWriter::new();

    let subscriber = tracing_subscriber::registry().with(
        fmt::layer()
            .with_writer(make_writer)
            .with_ansi(false)
            .with_target(false),
    );

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    writer
}
