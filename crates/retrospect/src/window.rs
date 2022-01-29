use std::{f64::consts, sync::Arc, thread};

use eframe::epi::{self, App, Frame};
use egui::{
    plot::{Legend, Plot, Points, Value, Values},
    Color32, CtxRef, InnerResponse, Sense, Ui,
};
use plotters::prelude::IntoDrawingArea;
use plotters_bitmap::BitMapBackend;
use plotters_eframe::PlottersWidget;

use crate::{export::export, plot::PlotProjection, point::Point};

pub struct Window {
    points: Arc<Vec<Point>>,

    projection: PlotProjection,

    bitmap_backend: bool,
    native_plotters: bool,
}

impl Window {
    pub fn new(points: Vec<Point>) -> Self {
        Self {
            points: Arc::from(points),

            projection: PlotProjection {
                pitch: consts::FRAC_PI_6,
                scale: 0.75,
                yaw: consts::FRAC_PI_3,
            },

            bitmap_backend: false,
            native_plotters: true,
        }
    }

    pub fn add_point(&mut self, point: Point) {
        Arc::make_mut(&mut self.points).push(point);
    }
}

impl App for Window {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        ctx.set_debug_on_hover(true);

        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: size selection
            if ui.button("💾").clicked() {
                let points = self.points.clone();
                let projection = self.projection;

                // TODO: error dialog
                thread::spawn(move || export(projection, &points, (1080, 1080)).unwrap());
            }

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
                    let plot = |ui: &mut Ui, map: fn((f64, f64, f64)) -> Value| {
                        Plot::new(&map)
                            .view_aspect(1.0)
                            .data_aspect(1.0)
                            .include_x(2.0)
                            .include_x(-2.0)
                            .include_y(2.0)
                            .include_y(-2.0)
                            .legend(Legend::default())
                            .width(ui.available_width().min(300.0))
                            .show(ui, {
                                let points = self.points.clone();

                                move |ui| {
                                    ui.points(
                                        Points::new(Values::from_values_iter(
                                            points.iter().copied().map(map),
                                        ))
                                        .name("Magnetometer Calibration data"),
                                    )
                                }
                            });
                    };

                    plot(&mut ui[0], |(x, y, _z)| Value::new(x, y));
                    plot(&mut ui[1], |(_x, y, z)| Value::new(y, z));
                    plot(&mut ui[2], |(x, _y, z)| Value::new(x, z));
                });
            }

            ui.checkbox(&mut self.bitmap_backend, "Bitmap Backend?");

            ui.columns(self.bitmap_backend.then(|| 2).unwrap_or(1), |ui| {
                egui::trace!(ui[0], "PlottersWidget");

                let InnerResponse {
                    response: plotter, ..
                } = PlottersWidget::new(frame)
                    .sense(Sense::click_and_drag())
                    .show(&mut ui[0], {
                        let points = self.points.clone();
                        let projection = self.projection;

                        move |backend| {
                            let drawing_area = backend.into_drawing_area();

                            crate::plot::draw_plot(drawing_area, projection, points.as_ref())
                        }
                    });

                let drag = plotter.drag_delta();

                self.projection.yaw -= (drag.x / plotter.rect.width()) as f64;
                self.projection.pitch += (drag.y / plotter.rect.height()) as f64;

                // Bitmap
                if self.bitmap_backend {
                    egui::trace!(ui[1], "BitmapBackend");

                    let available_size = ui[1].available_size().round();

                    let x = available_size.x as usize;
                    let y = available_size.y as usize;

                    let mut buffer = vec![0; x * y * 3];
                    let backend = BitMapBackend::with_buffer(&mut buffer, (x as u32, y as u32));

                    let drawing_area = backend.into_drawing_area();

                    crate::plot::draw_plot(drawing_area, self.projection, self.points.as_ref());

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
