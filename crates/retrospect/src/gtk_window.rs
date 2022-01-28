use std::{
    f64::consts,
    sync::{Arc, Mutex},
};

use gtk::{prelude::*, DrawingArea, Orientation};
use plotters::prelude::*;
use plotters_cairo::CairoBackend;
use tracing::info_span;

use crate::{
    plot::{draw_plot, PlotProjection},
    point::Point,
};

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
    let _build_ui_span = info_span!("build_ui");

    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Vis")
        .build();

    let drawing_area = DrawingArea::builder().hexpand(true).vexpand(true).build();

    let projection = Arc::new(Mutex::new(PlotProjection {
        pitch: consts::FRAC_PI_6,
        scale: 0.75,
        yaw: consts::FRAC_PI_3,
    }));

    let text = gtk::Label::builder().build();

    drawing_area.set_draw_func({
        let projection = projection.clone();

        move |_, context, width, height| {
            let projection = projection.lock().unwrap();

            let width = cast::u32(width).expect("provided a negative width");
            let height = cast::u32(height).expect("provided a negative height");

            let backend = CairoBackend::new(context, (width, height));
            let drawing_area = backend.into_drawing_area();

            draw_plot(drawing_area, &projection, points.as_slice())
        }
    });

    drawing_area.add_controller(&{
        let gesture = gtk::GestureDrag::new();

        let projection_save = Arc::new(Mutex::new(None));

        gesture.connect_drag_begin({
            let projection_save = projection_save.clone();
            let projection = projection.clone();

            move |gesture, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);

                let mut projection_save = projection_save.lock().unwrap();
                let projection = projection.lock().unwrap();

                projection_save.replace(*projection);
            }
        });

        gesture.connect_drag_update({
            let projection_save = projection_save.clone();
            let drawing_area = drawing_area.clone();
            let text = text.clone();

            move |gesture, x, y| {
                gesture.set_state(gtk::EventSequenceState::Claimed);

                let projection_delta = PlotProjection {
                    pitch: y / cast::f64(drawing_area.height()) * consts::TAU % consts::PI,
                    yaw: -x / cast::f64(drawing_area.width()) * consts::TAU % consts::PI,
                    scale: 0.0,
                };

                let mut projection = projection.lock().unwrap();
                let projection_save = projection_save.lock().unwrap();
                let projection_save =
                    projection_save.expect("connect_drag_update called before connect_drag_begin");

                *projection = projection_save + projection_delta;

                text.set_label(&format!(
                    "yaw: {:.1}°, pitch: {:.1}°, scale: {:.2}",
                    projection.yaw.to_degrees(),
                    projection.pitch.to_degrees(),
                    projection.scale
                ));

                drawing_area.queue_draw();
            }
        });

        gesture.connect_drag_end({
            move |gesture, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);

                projection_save.lock().unwrap().take();
            }
        });

        gesture
    });

    let action_bar = gtk::ActionBar::builder().build();
    action_bar.pack_start(&{ gtk::Button::builder().icon_name("document-save-as").build() });
    action_bar.pack_end(&text);

    let column = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    column.append(&drawing_area);
    column.append(&action_bar);

    window.set_child(Some(&column));
    window.set_default_size(500, 500);

    window.show();
}
