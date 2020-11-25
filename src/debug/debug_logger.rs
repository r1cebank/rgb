use crate::debug::message::DebugMessage;
use flume::{Sender, TrySendError};
use log::{LevelFilter, Log, Metadata, Record};
use simplelog::{Config, SharedLogger};
use std::borrow::Cow;

// A logger that implements sharedlogger from simplelogger, used to direct log to screen
pub struct DebugLogger {
    level: LevelFilter,
    config: Config,
    filter_allow: Cow<'static, [Cow<'static, str>]>,
    log_sender: Sender<DebugMessage>,
}

impl DebugLogger {
    pub fn new(
        level_filter: LevelFilter,
        config: Config,
        log_sender: Sender<DebugMessage>,
    ) -> Box<DebugLogger> {
        Box::new(Self {
            filter_allow: Cow::Borrowed(&[]),
            level: level_filter,
            config,
            log_sender,
        })
    }
    pub fn add_filter_allow(&mut self, time_format: String) -> &mut DebugLogger {
        let mut list = Vec::from(&*self.filter_allow);
        list.push(Cow::Owned(time_format));
        self.filter_allow = Cow::Owned(list);
        self
    }
    pub fn should_skip(&self, config: &Config, record: &Record<'_>) -> bool {
        // If a module path and allowed list are available
        match (record.target(), &*self.filter_allow) {
            (path, allowed) if allowed.len() > 0 => {
                // Check that the module path matches at least one allow filter
                if let None = allowed.iter().find(|v| path.starts_with(&***v)) {
                    // If not, skip any further writing
                    return true;
                }
            }
            _ => {}
        }
        return false;
    }
}

impl SharedLogger for DebugLogger {
    fn level(&self) -> LevelFilter {
        self.level
    }

    fn config(&self) -> Option<&Config> {
        Some(&self.config)
    }

    fn as_log(self: Box<Self>) -> Box<dyn Log> {
        Box::new(*self)
    }
}

impl Log for DebugLogger {
    fn enabled<'a>(&self, metadata: &Metadata<'a>) -> bool {
        metadata.level() <= self.level
    }

    fn log<'a>(&self, record: &Record<'a>) {
        if self.enabled(record.metadata()) {
            if self.should_skip(&self.config, record) {
                // NOP
            } else {
                match self
                    .log_sender
                    .try_send(DebugMessage::LogUpdate(format!("{}", record.args())))
                {
                    Ok(_) => {}
                    Err(TrySendError::Full(_)) => {}
                    Err(TrySendError::Disconnected(_)) => {}
                }
            }
        }
    }

    fn flush(&self) {}
}
