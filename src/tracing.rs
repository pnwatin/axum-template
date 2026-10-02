use tracing_subscriber::{
    EnvFilter, filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt,
};

pub fn init() {
    init_with_default_level(LevelFilter::INFO);
}

pub fn init_with_default_level(level: LevelFilter) {
    let env_filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env_lossy();

    let format_layer = tracing_subscriber::fmt::layer().pretty();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(format_layer)
        .init();
}
