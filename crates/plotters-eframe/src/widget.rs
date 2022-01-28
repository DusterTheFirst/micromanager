use eframe::{
    egui::{Response, Sense, Ui, Vec2, Widget},
    epi::Frame,
};

use crate::backend::EguiBackend;

pub struct PlottersWidget<'f> {
    frame: &'f Frame,
    plot: Box<dyn FnOnce(EguiBackend)>,
    size: Vec2,
}

impl<'f> PlottersWidget<'f> {
    pub fn new(frame: &'f Frame, plot: impl FnOnce(EguiBackend) + 'static, size: Vec2) -> Self {
        Self {
            frame,
            plot: Box::new(plot), // TODO: unbox?
            size,
        }
    }
}

impl<'f> Widget for PlottersWidget<'f> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (response, painter) = ui.allocate_painter(self.size, Sense::focusable_noninteractive());

        let backend = EguiBackend::new(self.frame, painter, response.rect);

        (self.plot)(backend);

        response
    }
}
