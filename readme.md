# Using Opentelemetry with Rust on Honeycomb

I spent a great deal of time getting opentelemetry tracing on Honeycomb to work and thought I'd share a setup for others to at least start with something that works and modify it from there. Some tough parts were:

* The Opentelemetry-otlp crate looks for a copy of [protoc](https://grpc.io/docs/protoc-installation/) if you use the most recent one (0.11). Fortunately 0.10 doesn't
* Downgrading Opentelemetry-otlp means it will look for an older version of the tracer in the Opentelemetry crate and on top of that the tracing_opentelemetry crate is looking for a certain version of the tracer as well. Turns out that 0.10 and 0.17 
* The Honeycomb endpoint is https://api.honeycomb.io, but the one for traces is https://api.honeycomb.io/v1/traces and does not feature prominently on their site

Note: if you only care about Opentelemetry and not tracing_opentelemetry as well (tracing_opentelemetry is nicer as it gives access to the #[instrument] macro and the info!() etc. macros) then the setup is a bit easier and you can just get a [BoxedTracer](https://docs.rs/opentelemetry/latest/opentelemetry/global/struct.BoxedTracer.html) to do the tracing. Those examples are pretty common online, just make sure the endpoint is the right one if using Honeycomb.