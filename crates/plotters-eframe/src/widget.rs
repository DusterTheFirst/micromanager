use eframe::{
    egui::{InnerResponse, NumExt, Sense, Ui, Vec2},
    epi::Frame,
};

use crate::backend::EguiBackend;

pub struct PlottersWidget {
    frame: Frame,

    min_size: Vec2,

    width: Option<f32>,
    height: Option<f32>,

    sense: Sense,
}

impl PlottersWidget {
    pub fn new(frame: &Frame) -> Self {
        Self {
            frame: frame.clone(),
            min_size: Vec2::splat(64.0),
            height: None,
            width: None,
            sense: Sense::focusable_noninteractive(),
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.min_size.x = width;
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.min_size.y = height;
        self.height = Some(height);
        self
    }

    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, plot_fn: impl FnOnce(EguiBackend) -> R) -> InnerResponse<R> {
        let size = {
            let width = self
                .width
                .unwrap_or_else(|| ui.available_size_before_wrap().x)
                .at_least(self.min_size.x);

            let height = self
                .height
                .unwrap_or_else(|| ui.available_size_before_wrap().y)
                .at_least(self.min_size.y);

            Vec2::new(width, height)
        };

        let (response, painter) = ui.allocate_painter(size, self.sense);

        let backend = EguiBackend::new(self.frame, painter, response.rect);

        let inner = (plot_fn)(backend);

        InnerResponse::new(inner, response)
    }
}
