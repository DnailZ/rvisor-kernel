use log::{Level, Log, Metadata, Record, SetLoggerError};

struct KernelLogger {
    level: Level,
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_string = {
                #[cfg(feature = "colored")]
                {
                    match record.level() {
                        Level::Error => record.level().to_string().red(),
                        Level::Warn => record.level().to_string().yellow(),
                        Level::Info => record.level().to_string().cyan(),
                        Level::Debug => record.level().to_string().purple(),
                        Level::Trace => record.level().to_string().normal(),
                    }
                }
                #[cfg(not(feature = "colored"))]
                {
                    record.level().to_string()
                }
            };
            let target = if record.target().len() > 0 {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };
            #[cfg(feature = "chrono")]
            {
                println!(
                    "{} {:<5} [{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
                    level_string,
                    target,
                    record.args()
                );

            #[cfg(not(feature = "chrono"))]
            {
                println!("{:<5} [{}] {}", level_string, target, record.args());
            }
        }
    }

    fn flush(&self) {}
}
