use std::sync::Arc;

use color_eyre::Result;
use pyo3::Python;
use tracing::{info, info_span};

use crate::point::Point;

pub fn run_python(python: Python, points: Arc<Vec<Point>>) -> Result<()> {
    let _python_span = info_span!("run_python");

    info!("GUL acquired");

    info!("Importing matplotlib...");
    let pyplot = python.import("matplotlib.pyplot")?;
    info!("Importing matplotlib...success");

    let locals = pyo3::types::PyDict::new(python);
    locals.set_item("pyplot", pyplot)?;
    locals.set_item("raw_data", points.as_ref())?;

    let code = include_str!("plot.py");

    info!("Running code...");
    python.run(code, None, Some(locals))?;

    Ok(())
}
