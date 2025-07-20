use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use tracing_subscriber::fmt::{self, MakeWriter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

#[derive(Clone, Default)]
pub struct Logs {
    logs: Arc<Mutex<Vec<String>>>,
}

impl Logs {
    /// # Panics
    ///
    /// Will panic if unable to acquire the lock.
    #[must_use]
    pub fn get(&self) -> Vec<String> {
        let logs = self.logs.lock().unwrap();
        logs.clone()
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

    let env_filter = EnvFilter::new("horizon=info");

    let subscriber = tracing_subscriber::registry().with(env_filter).with(
        fmt::layer()
            .with_writer(make_writer)
            .with_ansi(false)
            .with_target(false),
    );

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    writer
}
