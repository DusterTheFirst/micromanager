use std::ops::Add;

use plotters::{
    coord::Shift,
    prelude::*,
    style::{Color, BLACK, WHITE},
};
use plotters_cairo::CairoBackend;

use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PlotProjection {
    pub yaw: f64,
    pub pitch: f64,
    pub scale: f64,
}

impl Add for PlotProjection {
    type Output = PlotProjection;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            pitch: self.pitch + rhs.pitch,
            scale: self.scale + rhs.scale,
            yaw: self.yaw + rhs.yaw,
        }
    }
}

pub fn draw_plot(
    drawing_area: DrawingArea<CairoBackend, Shift>,
    projection: &PlotProjection,
    points: &[Point],
) {
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(
            "3D Scatter Plot of Magnetometer Data".to_string(),
            ("sans", 20),
        )
        .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)
        .unwrap();

    chart.with_projection(|mut pb| {
        pb.yaw = projection.yaw;
        pb.pitch = projection.pitch;
        pb.scale = projection.scale;

        pb.into_matrix()
    });

    chart.configure_axes().draw().unwrap();

    chart
        .draw_series(PointSeries::of_element(
            points.iter().copied(),
            2u32,
            RED.filled(),
            &|(x, y, z), size, style| Circle::new((x, y, z), size, style),
        ))
        .unwrap()
        .label("Test")
        .legend(|(x, y)| Circle::new((x, y), 2u32, RED.filled()));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.75))
        .draw()
        .unwrap();
}
