use opentelemetry_appender_tracing::layer;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::logs::SdkLoggerProvider;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    let otlp_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()
        .expect("Failed to create OTLP log exporter");

    let logger_provider = SdkLoggerProvider::builder()
        .with_simple_exporter(otlp_exporter)
        .build();

    let filter = EnvFilter::new("info")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    let otel_layer = layer::OpenTelemetryTracingBridge::new(&logger_provider).with_filter(filter);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer().json().with_filter(filter);

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(fmt_layer)
        .init();
}
