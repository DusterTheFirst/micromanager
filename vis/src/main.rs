use std::sync::Arc;

use color_eyre::{eyre::Context, Result};
use point::PointCloud;

pub mod point;
mod window;

fn main() -> Result<()> {
    color_eyre::install()?;
    install_tracing();

    let points: Vec<_> = PointCloud::iter().take(500).collect();
    let points = Arc::new(points);

    window::main(points);

    Ok(())
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
