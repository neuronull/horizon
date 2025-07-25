use std::io::{self, Write};
use tokio::sync::mpsc::Sender;
use tracing_subscriber::fmt::{self, MakeWriter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct Logs {
    tx: Sender<String>,
}

impl Write for Logs {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = std::str::from_utf8(buf).unwrap_or_default();

        if !s.trim().is_empty() {
            // TODO check result
            self.tx.try_send(s.to_string());
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
    pub fn new(tx: Sender<String>) -> Self {
        Self {
            writer: Logs { tx },
        }
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
pub fn setup_logging(tx: Sender<String>) {
    let make_writer = LogWriter::new(tx);

    let env_filter = EnvFilter::new("horizon=info,lib_weather=info");

    #[cfg(not(target_arch = "wasm32"))]
    {
        let subscriber = tracing_subscriber::registry().with(env_filter).with(
            fmt::layer()
                .with_writer(make_writer)
                .with_ansi(false)
                .with_target(false),
        );
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }

    #[cfg(target_arch = "wasm32")]
    {
        use tracing_subscriber::fmt::{format::Writer, time::FormatTime};
        use web_sys::js_sys::Date;

        #[derive(Debug)]
        struct WasmTime;

        impl FormatTime for WasmTime {
            fn format_time(&self, w: &mut Writer<'_>) -> Result<(), std::fmt::Error> {
                let now = Date::new_0();
                write!(w, "[{}]", now.to_locale_time_string("en-US"))
            }
        }
        let subscriber = tracing_subscriber::registry().with(env_filter).with(
            fmt::layer()
                .with_writer(make_writer)
                .with_ansi(false)
                .with_target(false)
                .with_timer(WasmTime),
        );

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }
}
