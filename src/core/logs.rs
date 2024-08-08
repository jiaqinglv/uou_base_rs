use std::error::Error;
use tracing::Level;
use tracing::Level as TraceLevel;

use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub trait Logger: Sized + Send + Sync + Send {
    // 创建默认日志记录
    fn new(level: Level) -> Result<Self, Box<dyn Error>>;
}

pub struct DefaultLogger {
    pub level: Level,
}

impl Logger for DefaultLogger {
    fn new(level: Level) -> Result<Self, Box<dyn Error>> {
        match new_default_log(level) {
            Ok(_) => Ok(DefaultLogger { level }),
            Err(err) => panic!("{}", err),
        }
    }
}

impl DefaultLogger {
    /// opentelemetry
    pub fn new_tracing_opentelemetry(tracer_name: String) -> Result<(), Box<dyn Error>> {
        let exporter = opentelemetry_stdout::LogExporterBuilder::default().build();

        let logger_provider = opentelemetry_sdk::logs::LoggerProvider::builder()
            // .with_config(opentelemetry_sdk::logs::Config::default()
            .with_resource(opentelemetry_sdk::Resource::new(vec![KeyValue::new(
                "service.name",
                tracer_name,
            )]))
            .with_simple_exporter(exporter)
            .build();

        let otel_log_appender = layer::OpenTelemetryTracingBridge::new(&logger_provider);
        tracing_subscriber::registry()
            .with(otel_log_appender)
            .init();

        Ok(())
    }
}

/// 初始化默认日志
pub fn new_default_log(level: tracing::Level) -> Result<(), Box<dyn Error>> {
    tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(match level {
            Level::ERROR => TraceLevel::ERROR,
            Level::WARN => TraceLevel::WARN,
            Level::INFO => TraceLevel::INFO,
            Level::DEBUG => TraceLevel::DEBUG,
            Level::TRACE => TraceLevel::TRACE,
        })
        // builds the subscriber.
        .finish()
        .init();
    Ok(())
}
