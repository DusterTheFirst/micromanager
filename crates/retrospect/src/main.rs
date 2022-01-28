use color_eyre::Result;
use eframe::NativeOptions;
use point::PointCloud;
use window::Window;

pub mod plot;
pub mod point;
// pub mod gtk_window;
pub mod window;

fn main() -> Result<()> {
    color_eyre::install()?;
    install_tracing();

    let points: Vec<_> = PointCloud::iter().take(500).collect();

    eframe::run_native(Box::new(Window::new(points)), NativeOptions::default());
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
