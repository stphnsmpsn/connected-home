use http::HeaderMap;
use opentelemetry::{
    global,
    propagation::{Extractor, Injector},
    sdk::{
        propagation::TraceContextPropagator,
        trace,
        trace::{RandomIdGenerator, Sampler},
        Resource,
    },
    KeyValue,
};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};

pub use tracing::{instrument, level_filters::LevelFilter};

use crate::tracing::config::TracingConfig;

pub mod config;

/// initialize tracing. this function will install the otlp exporter and set the global tracer.
/// the exporter / tracer will be installed on the tokio runtime, therefore you must call this
/// from a tokio runtime.
pub fn init_tracing(config: TracingConfig) {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(format!("{}:{}", config.tempo_url, config.tempo_port)) // change the endpoint accordingly
        .with_protocol(Protocol::Grpc);
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new("service.name", config.service_name)])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("pipeline install failure");

    let layer = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(config.level);

    Registry::default()
        .with(layer)
        .try_init()
        .expect("failed to initialize tracing");
}

pub struct HeaderInjector<'a>(pub &'a mut http::HeaderMap);

impl<'a> Injector for HeaderInjector<'a> {
    /// Set a key and value in the HeaderMap.  Does nothing if the key or value are not valid inputs.
    fn set(&mut self, key: &str, value: String) {
        if let Ok(name) = http::header::HeaderName::from_bytes(key.as_bytes()) {
            if let Ok(val) = http::header::HeaderValue::from_str(&value) {
                self.0.insert(name, val);
            }
        }
    }
}

pub fn store_tracing_context(headers: &mut HeaderMap) {
    let span = tracing::Span::current();
    let context = span.context();
    let mut injector = HeaderInjector(headers);
    global::get_text_map_propagator(|propagator| propagator.inject_context(&context, &mut injector));
}

pub struct HeaderExtractor<'a>(pub &'a http::HeaderMap);

impl<'a> Extractor for HeaderExtractor<'a> {
    /// Get a value for a key from the HeaderMap.  If the value is not valid ASCII, returns None.
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    /// Collect all the keys from the HeaderMap.
    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|value| value.as_str()).collect::<Vec<_>>()
    }
}

pub fn restore_tracing_context(headers: &HeaderMap) -> opentelemetry::Context {
    global::get_text_map_propagator(|propagator| {
        let extractor = HeaderExtractor(headers);
        propagator.extract(&extractor)
    })
}
