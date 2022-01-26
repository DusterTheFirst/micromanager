use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use cairo::Context;
use cast::u32;
use gtk::{
    glib::{self, clone},
    prelude::*,
    DrawingArea, Orientation,
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
    (azimuth, elevation): (f64, f64),
    points: Arc<Vec<Point>>,
) {
    let width = u32(width).unwrap();
    let height = u32(height).unwrap();

    let backend = CairoBackend::new(cr, (width, height));
    let area = backend.into_drawing_area();

    area.fill(&WHITE).unwrap();

    let x_axis = (-3.0..3.0).step(0.1);
    let z_axis = (-3.0..3.0).step(0.1);

    let mut chart = ChartBuilder::on(&area)
        .caption("3D Plot Test".to_string(), ("sans", 20))
        .build_cartesian_3d(x_axis, -3.0..3.0, z_axis)
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

    let mouse_position = Arc::new((
        AtomicU64::new(0.0f64.to_bits()),
        AtomicU64::new(0.0f64.to_bits()),
    ));

    let text = gtk::Label::new(None);

    drawing_area.set_draw_func(clone!(@strong mouse_position => move |drawing_area, context, width, height| {
        let mouse_position = (f64::from_bits(mouse_position.0.load(Ordering::SeqCst)), f64::from_bits(mouse_position.1.load(Ordering::SeqCst)));

        draw(drawing_area, context, width, height, mouse_position, points.clone())
    }));

    drawing_area.add_controller(&{
        let gesture = gtk::GestureDrag::new();

        gesture.connect_drag_update(
            clone!(@weak drawing_area, @weak mouse_position, @weak text => move |gesture, x, y| {
                gesture.set_state(gtk::EventSequenceState::Claimed);

                drawing_area.queue_draw();

                let x = x / cast::f64(drawing_area.width()) * 360.0;
                let y = y / cast::f64(drawing_area.height()) * 360.0;

                mouse_position.0.store(x.to_radians().to_bits(), Ordering::SeqCst);
                mouse_position.1.store(y.to_radians().to_bits(), Ordering::SeqCst);

                text.set_label(&format!("x: {}, y: {}", x, y));
            }),
        );

        gesture
    });

    let column = gtk::Box::new(Orientation::Vertical, 0);
    column.append(&drawing_area);
    column.append(&text);

    window.set_child(Some(&column));
    window.set_default_size(500, 500);

    window.show();
}
