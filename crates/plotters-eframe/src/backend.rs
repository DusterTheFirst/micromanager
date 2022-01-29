use std::{convert::Infallible, mem};

use eframe::{
    egui::{
        color::{linear_f32_from_gamma_u8, Rgba},
        emath::{Pos2, RectTransform},
        epaint::Mesh,
        Align, Align2, Color32, Painter, Rect, Shape, Stroke, TextStyle, Vec2,
    },
    epi::{Frame, Image},
};
use plotters_backend::{
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
};

pub struct EguiBackend {
    frame: Frame,
    painter: Painter,
    size: Vec2,

    to_screen_coords: RectTransform,

    shapes: Vec<Shape>,
}

impl EguiBackend {
    // TODO: note about widget
    pub fn new(frame: Frame, painter: Painter, rect: Rect) -> Self {
        let to_screen_coords =
            RectTransform::from_to(Rect::from_min_size(Pos2::ZERO, rect.size()), rect);

        Self {
            frame,
            painter,
            size: rect.size(),

            to_screen_coords,

            shapes: Vec::new(),
        }
    }

    fn pos2(&self, (x, y): BackendCoord) -> Pos2 {
        self.painter
            .round_pos_to_pixels(self.to_screen_coords * Pos2::new(x as f32, y as f32))
    }
}

fn color32(color: BackendColor) -> Color32 {
    let (r, g, b) = color.rgb;

    let r = linear_f32_from_gamma_u8(r);
    let g = linear_f32_from_gamma_u8(g);
    let b = linear_f32_from_gamma_u8(b);
    let a = color.alpha as f32;

    Rgba::from_rgba_premultiplied(r * a, g * a, b * a, a).into()
}

fn stroke<S: BackendStyle>(style: &S) -> Stroke {
    Stroke::new(style.stroke_width() as f32, color32(style.color()))
}

impl DrawingBackend for EguiBackend {
    type ErrorType = Infallible;

    fn get_size(&self) -> (u32, u32) {
        let Vec2 { x, y } = self.painter.round_vec_to_pixels(self.size);

        (x as u32, y as u32)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(()) // TODO:
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let shapes = mem::take(&mut self.shapes);

        self.painter.extend(shapes);

        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.draw_rect(point, (point.0 + 1, point.1 + 1), &color, true)
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.shapes.push(Shape::line_segment(
            [self.pos2(from), self.pos2(to)],
            stroke(style),
        ));

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let rect = Rect {
            min: self.pos2(upper_left),
            max: self.pos2(bottom_right),
        };

        self.shapes.push(if fill {
            Shape::rect_filled(rect, 0.0, color32(style.color()))
        } else {
            Shape::rect_stroke(rect, 0.0, stroke(style))
        });

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.shapes.push(Shape::line(
            path.into_iter().map(|coord| self.pos2(coord)).collect(),
            stroke(style),
        ));

        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if fill {
            self.shapes.push(Shape::circle_filled(
                self.pos2(center),
                radius as f32,
                color32(style.color()),
            ))
        } else {
            self.shapes.push(Shape::circle_stroke(
                self.pos2(center),
                radius as f32,
                stroke(style),
            ))
        }

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.shapes.push(Shape::convex_polygon(
            vert.into_iter().map(|coord| self.pos2(coord)).collect(),
            color32(style.color()),
            Stroke::none(),
        ));

        Ok(())
    }

    // TODO: note the lack of text styling
    fn draw_text<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        use plotters_backend::text_anchor::{HPos, Pos, VPos};

        let anchor = {
            let Pos { h_pos, v_pos } = style.anchor();

            let horizontal = match h_pos {
                HPos::Left => Align::LEFT,
                HPos::Right => Align::RIGHT,
                HPos::Center => Align::Center,
            };

            let vertical = match v_pos {
                VPos::Top => Align::TOP,
                VPos::Center => Align::Center,
                VPos::Bottom => Align::BOTTOM,
            };

            Align2([horizontal, vertical])
        };

        self.shapes.push(Shape::text(
            self.painter.fonts(),
            self.pos2(pos),
            anchor,
            text,
            TextStyle::Monospace,
            color32(style.color()),
        ));

        Ok(())
    }

    // TODO:
    // fn estimate_text_size<TStyle: BackendTextStyle>(
    //     &self,
    //     text: &str,
    //     style: &TStyle,
    // ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
    //     todo!()
    // }

    // TODO: test
    fn blit_bitmap(
        &mut self,
        pos: BackendCoord,
        (iw, ih): (u32, u32),
        src: &[u8],
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let pixels = src
            .chunks_exact(4)
            .map(|pixels| Color32::from_rgba_unmultiplied(pixels[0], pixels[1], pixels[2], u8::MAX))
            .collect();

        let texture = self.frame.alloc_texture(Image {
            size: [iw as usize, ih as usize],
            pixels,
        });

        let mesh = {
            let mut mesh = Mesh::with_texture(texture);

            let rect_start = self.pos2(pos);
            let image_size = Pos2::new(iw as f32, ih as f32);

            mesh.add_rect_with_uv(
                Rect {
                    min: rect_start,
                    max: rect_start + image_size.to_vec2(),
                },
                Rect {
                    min: Pos2::ZERO,
                    max: image_size,
                },
                Color32::TRANSPARENT,
            );

            mesh
        };

        self.shapes.push(Shape::mesh(mesh));

        // TODO: maybe hashing/caching to reuse textures that do not change
        self.frame.free_texture(texture);

        Ok(())
    }
}
