use tracing_subscriber::{fmt, layer::{SubscriberExt, Layer}, util::SubscriberInitExt};
use tracing_subscriber::EnvFilter;
use tracing_appender::rolling;

use tracing::Level;

pub fn init_logging() {
    // FILE APPENDERS 
    let access_file = rolling::daily("logs", "access.log");
    let app_file = rolling::daily("logs", "application.log");

    let (access_writer, _access_guard) = tracing_appender::non_blocking(access_file);
    let (app_writer, _app_guard) = tracing_appender::non_blocking(app_file);

    // IMPORTANT: guards must live forever
    std::mem::forget(_access_guard);
    std::mem::forget(_app_guard);

    // ACCESS LOG LAYER 
    let access_layer = fmt::layer()
        .with_writer(access_writer)
        .with_ansi(false)
        .with_target(false)
        .with_level(true)
        .with_filter(
            EnvFilter::new("tower_http=info")
        );

    // APPLICATION LOG LAYER 
    let app_layer = fmt::layer()
        .with_writer(app_writer)
        .with_ansi(false)
        .with_target(true)
        .with_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        );

    tracing_subscriber::registry()
        .with(access_layer)
        .with(app_layer)
        .init();
}
