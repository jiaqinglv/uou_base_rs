use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, Resource};
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
    /// new_tracing_opentelemetry
    pub fn new_tracing_opentelemetry(tracer_name: String) -> Result<(), Box<dyn Error>> {
        let exporter = opentelemetry_stdout::LogExporterBuilder::default().build();

        let logger_provider = opentelemetry_sdk::logs::LoggerProvider::builder()
            // .with_config(opentelemetry_sdk::logs::Config::default())
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

    /// new_jaeger_opentelemetry
    pub fn new_jaeger_opentelemetry_log(
        tracer_name: String,
        endpoint: String,
    ) -> Result<(), Box<dyn Error>> {
        let tracer_provider = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(endpoint),
            )
            .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
                Resource::new(vec![KeyValue::new(
                    opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                    tracer_name.clone(),
                )]),
            ))
            .install_batch(runtime::Tokio)
            .unwrap();

        opentelemetry::global::set_tracer_provider(tracer_provider.clone());

        let tracer_name_static = format!("{}", tracer_name);
        opentelemetry::global::tracer(tracer_name_static);

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
