use std::{
    f64::consts,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use cast::u32;
use gtk::{
    glib::{self, clone},
    prelude::*,
    DrawingArea, Orientation, cairo::Context,
};
use plotters::{
    prelude::*,
    style::{Color, BLACK, WHITE},
};
use plotters_cairo::CairoBackend;
use tracing::info_span;

use crate::point::Point;

fn draw(
    _: &DrawingArea,
    cr: &Context,
    width: i32,
    height: i32,
    azimuth: f64,
    elevation: f64,
    points: Arc<Vec<Point>>,
) {
    let width = u32(width).unwrap();
    let height = u32(height).unwrap();

    let backend = CairoBackend::new(cr, (width, height));
    let area = backend.into_drawing_area();

    area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&area)
        .caption(
            "3D Scatter Plot of Magnetometer Data".to_string(),
            ("sans", 20),
        )
        .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)
        .unwrap();

    chart.with_projection(|mut pb| {
        pb.yaw = azimuth;
        pb.pitch = elevation;

        pb.scale = 0.75;

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

pub fn main(points: Arc<Vec<Point>>) {
    let _gtk_span = info_span!("gtk_main");

    let application = gtk::Application::new(
        Some("com.dusterthefirst.micromanager-vis"),
        Default::default(),
    );

    application.connect_activate(move |application| build_ui(application, points.clone()));

    application.run();
}

pub fn build_ui(application: &gtk::Application, points: Arc<Vec<Point>>) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Vis")
        .build();

    let drawing_area = DrawingArea::builder().hexpand(true).vexpand(true).build();

    let azimuth = Arc::new(AtomicU64::new(consts::FRAC_PI_3.to_bits()));
    let elevation = Arc::new(AtomicU64::new(consts::FRAC_PI_6.to_bits()));

    let text = gtk::Label::new(None);

    drawing_area.set_draw_func(
        clone!(@strong azimuth, @strong elevation => move |drawing_area, context, width, height| {
            let azimuth = f64::from_bits(azimuth.load(Ordering::SeqCst));
            let elevation = f64::from_bits(elevation.load(Ordering::SeqCst));

            draw(drawing_area, context, width, height, azimuth, elevation, points.clone())
        }),
    );

    drawing_area.add_controller(&{
        let azimuth_save = Arc::new(AtomicU64::new(0.0f64.to_bits()));
        let elevation_save = Arc::new(AtomicU64::new(0.0f64.to_bits()));

        let gesture = gtk::GestureDrag::new();

        gesture.connect_drag_begin(clone!(@strong azimuth_save, @strong elevation_save,@strong azimuth, @strong elevation=> move |_gesture, _x, _y| {
            azimuth_save.store(azimuth.load(Ordering::SeqCst), Ordering::SeqCst);
            elevation_save.store(elevation.load(Ordering::SeqCst), Ordering::SeqCst);
        }));

        gesture.connect_drag_update(
            clone!(@weak drawing_area, @strong azimuth, @strong elevation, @weak text => move |gesture, x, y| {
                gesture.set_state(gtk::EventSequenceState::Claimed);

                let azimuth_delta = -x / cast::f64(drawing_area.width()) * consts::TAU;
                let elevation_delta = y / cast::f64(drawing_area.height()) * consts::TAU;

                let azimuth_save = f64::from_bits(azimuth_save.load(Ordering::SeqCst));
                let elevation_save = f64::from_bits(elevation_save.load(Ordering::SeqCst));

                let new_azimuth = (azimuth_save + azimuth_delta) % consts::TAU;
                let new_elevation = (elevation_save + elevation_delta)% consts::TAU;

                text.set_label(&format!("azimuth: {:.1}°, elevation: {:.1}°",new_azimuth.to_degrees(), new_elevation.to_degrees()));

                azimuth.store(new_azimuth.to_bits(), Ordering::SeqCst);
                elevation.store(new_elevation.to_bits(), Ordering::SeqCst);

                drawing_area.queue_draw();
            }),
        );

        gesture
    });

    let column = gtk::Box::new(Orientation::Vertical, 5);
    column.append(&drawing_area);
    column.append(&text);

    window.set_child(Some(&column));
    window.set_default_size(500, 500);

    window.show();
}
