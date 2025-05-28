use std::collections::VecDeque;
use std::sync::{Mutex};
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

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

pub trait LogAsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Log + Any> LogAsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn init() {
    log::set_boxed_logger(Box::new(UiLogger::new())).unwrap();
    log::set_max_level(LevelFilter::Trace);
}
