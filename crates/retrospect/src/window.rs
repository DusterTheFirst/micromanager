use std::{f64::consts, rc::Rc};

use eframe::epi::{self, App, Frame};
use egui::{
    plot::{Plot, Points, Value, Values},
    Color32, CtxRef,
};
use plotters::prelude::IntoDrawingArea;
use plotters_bitmap::BitMapBackend;
use plotters_eframe::PlottersWidget;

use crate::{plot::PlotProjection, point::Point};

pub struct Window {
    points: Rc<Vec<Point>>,

    bitmap_backend: bool,
    native_plotters: bool,
}

impl Window {
    pub fn new(points: Vec<Point>) -> Self {
        Self {
            points: Rc::from(points),

            bitmap_backend: false,
            native_plotters: true,
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
            ui.label(format!(
                "Last frame: {} ms",
                frame.info().cpu_usage.unwrap_or(0.0) * 1000.0
            ));
            ui.label(format!(
                "Pixels per point: {}",
                frame.info().native_pixels_per_point.unwrap_or(f32::NAN)
            ));

            ui.checkbox(&mut self.native_plotters, "Native Plotters?");

            if self.native_plotters {
                ui.columns(3, |ui| {
                    Plot::new("xy")
                        .view_aspect(1.0)
                        .data_aspect(1.0)
                        .width(ui[0].available_width().min(300.0))
                        .show(&mut ui[0], {
                            let points = self.points.clone();

                            move |ui| {
                                ui.points(Points::new(Values::from_values_iter(
                                    points.iter().copied().map(|(x, y, _)| Value::new(x, y)),
                                )))
                            }
                        });

                    Plot::new("yz")
                        .view_aspect(1.0)
                        .data_aspect(1.0)
                        .width(ui[1].available_width().min(300.0))
                        .show(&mut ui[1], {
                            let points = self.points.clone();

                            move |ui| {
                                ui.points(Points::new(Values::from_values_iter(
                                    points.iter().copied().map(|(_, y, z)| Value::new(y, z)),
                                )))
                            }
                        });

                    Plot::new("xz")
                        .view_aspect(1.0)
                        .data_aspect(1.0)
                        .width(ui[2].available_width().min(300.0))
                        .show(&mut ui[2], {
                            let points = self.points.clone();

                            move |ui| {
                                ui.points(Points::new(Values::from_values_iter(
                                    points.iter().copied().map(|(x, _, z)| Value::new(x, z)),
                                )))
                            }
                        });
                });
            }

            ui.checkbox(&mut self.bitmap_backend, "Bitmap Backend?");

            ui.columns(2, |ui| {
                egui::trace!(ui[0], "PlottersWidget");

                ui[0].add(PlottersWidget::new(
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
                    ui[0].available_size(),
                ));

                // Bitmap
                if self.bitmap_backend {
                    egui::trace!(ui[1], "BitmapBackend");

                    let available_size = ui[1].available_size().round();

                    let x = available_size.x as usize;
                    let y = available_size.y as usize;

                    let mut buffer = vec![0; x * y * 3];
                    let backend = BitMapBackend::with_buffer(&mut buffer, (x as u32, y as u32));

                    let drawing_area = backend.into_drawing_area();

                    crate::plot::draw_plot(
                        drawing_area,
                        &PlotProjection {
                            pitch: consts::FRAC_PI_6,
                            scale: 0.75,
                            yaw: consts::FRAC_PI_3,
                        },
                        self.points.as_ref(),
                    );

                    let pixels = buffer
                        .chunks_exact(3)
                        .map(|pixels| {
                            Color32::from_rgba_unmultiplied(
                                pixels[0],
                                pixels[1],
                                pixels[2],
                                u8::MAX,
                            )
                        })
                        .collect();

                    let texture_id = frame.alloc_texture(epi::Image {
                        size: [x, y],
                        pixels,
                    });

                    ui[1].image(texture_id, available_size);

                    frame.free_texture(texture_id);
                }
            });
        });
    }

    fn name(&self) -> &str {
        "hello there!"
    }
}
