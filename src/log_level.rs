use log::LogLevel;
use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub enum NephritLogLevel {
  Off,
  Error,
  Warn,
  Info,
  Debug,
  Trace,
}

impl NephritLogLevel {
  pub fn to_logger_log_level(level: Option<Self>) -> LogLevel {
    match level {
      Some(NephritLogLevel::Off) => LogLevel::Off,
      Some(NephritLogLevel::Error) => LogLevel::Error,
      Some(NephritLogLevel::Warn) => LogLevel::Warn,
      Some(NephritLogLevel::Info) => LogLevel::Info,
      Some(NephritLogLevel::Debug) => LogLevel::Debug,
      Some(NephritLogLevel::Trace) => LogLevel::Trace,
      None => LogLevel::Info,
    }
  }
}
