use std::sync::Arc;

use flexi_logger::{Age, Criterion, FileSpec, Logger, LoggerHandle, Naming};

pub struct FileLogger {
    pub logger: Arc<LoggerHandle>,
}

impl FileLogger {
    pub fn new(logger: Arc<LoggerHandle>) -> Self {
        FileLogger { logger: logger }
    }

    pub fn create_handle() -> LoggerHandle {
        Logger::try_with_str("info")
            .unwrap() // Write all error, warn, and info messages
            .log_to_file(FileSpec::default()
            
        )
            .rotate(
                // If the program runs long enough,
                Criterion::Age(Age::Day), // - create a new file every day
                Naming::Timestamps,
                flexi_logger::Cleanup::Never,
            )
            .start()
            .unwrap()
    }
}
