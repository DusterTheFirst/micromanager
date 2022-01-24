use std::f64::consts::{PI, TAU};

use color_eyre::Result;
use pyo3::{types::IntoPyDict, Python};
use rand::Rng;
use tracing::{info, info_span};

pub fn run_python(python: Python) -> Result<()> {
    let _python_span = info_span!("run_python");

    info!("GUL acquired");

    info!("Importing matplotlib...");
    let pyplot = python.import("matplotlib.pyplot")?;
    info!("Importing matplotlib...success");

    // FIXME: don't do this here
    let points = PointCloud.take(500).collect::<Vec<_>>();

    let locals = pyo3::types::PyDict::new(python);
    locals.set_item("pyplot", pyplot)?;
    locals.set_item("raw_data", points)?;

    let code = include_str!("plot.py");

    info!("Running code...");
    python.run(code, None, Some(locals))?;

    Ok(())
}

struct PointCloud;

impl Iterator for PointCloud {
    type Item = (f64, f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();

        let mut get_point = || loop {
            let num = (rng.gen::<f64>() - 0.5) * 2.0;

            if num == 0.0 {
                continue;
            }

            break num;
        };

        let x = get_point();
        let y = get_point();
        let z = get_point();

        let mag = (x * x + y * y + z * z).sqrt();

        Some((
            x / mag,
            y / mag,
            z / mag,
        ))
    }
}
