use std::collections::VecDeque;
use std::sync::{Arc, Mutex, LazyLock};
use log::{Level, LevelFilter, Metadata, Record, Log};
use std::any::Any;

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub level: Level,
    pub message: String
}

pub struct UiLogger {
    pub entries: Mutex<VecDeque<LogEntry>>,
    capacity: Mutex<usize>,
}

pub static GLOB_LOGGER : LazyLock<Arc<UiLogger>> = LazyLock::new(|| Arc::new(UiLogger::new()));

impl log::Log for UiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!(
                "[{:<5}] {}",
                record.level(),
                record.args()
            );

            while self.entries.lock().unwrap().len() >= *self.capacity.lock().unwrap() {
                let _ = self.entries.lock().unwrap().pop_front();
            }

            self.entries.lock().unwrap().push_back(LogEntry{
                level: record.level(),
                message: msg
            });
        }
    }

    fn flush(&self) {}
}

impl UiLogger {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(VecDeque::with_capacity(1000)),
            capacity: Mutex::new(1000)
        }
    }

    pub fn set_capacity(&mut self, new_cap: usize) {
        while self.entries.lock().unwrap().len() > new_cap {
            let _ = self.entries.lock().unwrap().pop_front();
        }
        *(self.capacity.lock().unwrap()) = new_cap;
    }

    pub fn init() -> Arc<UiLogger> {
        log::set_logger(&**GLOB_LOGGER).unwrap();
        log::set_max_level(LevelFilter::Trace);
        Arc::clone(&GLOB_LOGGER)
    }
}
