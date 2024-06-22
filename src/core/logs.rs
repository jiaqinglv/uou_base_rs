use std::error::Error;
use log::Level;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::Level as TraceLevel;

use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;
use opentelemetry::{
    KeyValue, global,
};
use opentelemetry_appender_log::{self, OpenTelemetryLogBridge};


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
            Ok(_) => Ok(DefaultLogger{
                level
            }),
            Err(err) => panic!("{}", err),
        }
    }
}

impl DefaultLogger {
    
    /// opentelemetry
    pub fn new_tracing_opentelemetry(level: log::Level, tracer_name: String) -> Result<Self, Box<dyn Error>>  {
        let exporter = opentelemetry_stdout::LogExporterBuilder::default().build();

        let logger_provider = opentelemetry_sdk::logs::LoggerProvider::builder()
            .with_config(
                opentelemetry_sdk::logs::Config::default().with_resource(opentelemetry_sdk::Resource::new(vec![KeyValue::new(
                    "service.name",
                    tracer_name,
                )])),
            )
            .with_simple_exporter(exporter)
            .build();
        
        let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
        log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
        log::set_max_level(level.to_level_filter());
        
        Ok(DefaultLogger{
            level
        })
    }

    /// opentelemetry
    pub fn new_tracing_opentelemetry_jaeger(level: log::Level, tracer_name: String) -> Result<Self, Box<dyn Error>>  {
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name(tracer_name)
            .install_simple()?;

        // Create a tracing layer with the configured tracer
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let subscriber = Registry::default().with(telemetry);        
        log::set_max_level(level.to_level_filter());
        // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
        global::set_text_map_propagator(TraceContextPropagator::new());

        subscriber.init();

        Ok(DefaultLogger{
            level
        })
    }

}



/// 初始化默认日志
pub fn new_default_log(level: log::Level) -> Result<(), Box<dyn Error>> {
    tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(match level {
            Level::Error => TraceLevel::ERROR,
            Level::Warn  => TraceLevel::WARN,
            Level::Info  => TraceLevel::INFO,
            Level::Debug => TraceLevel::DEBUG,
            Level::Trace => TraceLevel::TRACE
        })
        // builds the subscriber.
        .finish()
        .init();
    Ok(())
}
