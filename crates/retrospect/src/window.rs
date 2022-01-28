use std::{f64::consts, rc::Rc};

use eframe::epi::{App, Frame};
use egui::CtxRef;
use plotters::prelude::IntoDrawingArea;
use plotters_eframe::PlottersWidget;

use crate::{plot::PlotProjection, point::Point};

pub struct Window {
    points: Rc<Vec<Point>>,
}

impl Window {
    pub fn new(points: Vec<Point>) -> Self {
        Self {
            points: Rc::from(points),
        }
    }

    pub fn add_point(&mut self, point: Point) {
        Rc::make_mut(&mut self.points).push(point);
    }
}

impl App for Window {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        ctx.set_debug_on_hover(true);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Rendering {} points", self.points.len()));

            egui::trace!(ui, "PlottersWidget");

            ui.add(PlottersWidget::new(
                frame,
                {
                    let points = self.points.clone();

                    move |backend| {
                        let drawing_area = backend.into_drawing_area();

                        crate::plot::draw_plot(
                            drawing_area,
                            &PlotProjection {
                                pitch: consts::FRAC_PI_6,
                                scale: 0.75,
                                yaw: consts::FRAC_PI_3,
                            },
                            points.as_ref(),
                        )
                    }
                },
                ui.available_size(),
            ));
        });
    }

    fn name(&self) -> &str {
        "hello there!"
    }
}
