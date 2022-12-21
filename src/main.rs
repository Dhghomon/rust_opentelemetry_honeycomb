use opentelemetry::{global::shutdown_tracer_provider, sdk::trace as sdktrace, trace::TraceError};
use opentelemetry_otlp::WithExportConfig;
use std::{collections::HashMap, env::set_var};
use tracing::{error, info, info_span, warn};
use tracing_subscriber::layer::SubscriberExt;

const DATASET_NAME: &str = ""; // Choose a name like "mydataset" or "dev" and Honeycomb will create it
const API_KEY: &str = ""; // Sign up to Honeycomb and put your key in here

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_endpoint("https://api.honeycomb.io/v1/traces")
                .with_http_client(reqwest::Client::default())
                .with_headers(HashMap::from([
                    ("x-honeycomb-dataset".into(), DATASET_NAME.into()),
                    ("x-honeycomb-team".into(), API_KEY.into()),
                ]))
                .with_timeout(std::time::Duration::from_secs(2)),
        ) // Replace with runtime::Tokio if using async main
        .install_batch(opentelemetry::runtime::TokioCurrentThread)
}

#[tracing::instrument]
fn function_with_error_event() {
    info!(message = "extra_info", "info event!");
    let thing = vec![8];
    error!("Bunch of stuff: {thing:?}");
    function_with_warn_event();
}

#[tracing::instrument]
fn function_with_warn_event() {
    warn!("Some warning event");
    function_with_info_event();
}

#[tracing::instrument]
fn function_with_info_event() {
    info!("Some info event");
    function_with_all_events();
}

#[tracing::instrument]
fn function_with_all_events() {
    warn!("Warn event");
    error!("Error event");
    info!("Info event");
}

fn main() {
    // Used to bridge with async code if you want main to be non-async: https://tokio.rs/tokio/topics/bridging
    // Don't need to use rt if using async main
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    // Without this it panics with "thread 'main' panicked at 'there is no reactor running, must be called from the context of a Tokio 1.x runtime'"
    // Won't need it if you are just using a regular async main
    let _entered = rt.enter();
    set_var("RUST_LOG", "debug");
    env_logger::init();

    let tracer = init_tracer().unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    for _ in 0..2 {
        let root_span = info_span!("Root span");
        let _entered = root_span.enter();
        function_with_error_event();
    }
    shutdown_tracer_provider();
}
