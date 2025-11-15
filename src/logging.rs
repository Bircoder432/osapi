#[cfg(feature = "logging")]
pub fn init() {
    use tracing_subscriber::prelude::*;
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
