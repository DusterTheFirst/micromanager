use std::{
    error::Error,
    fmt::{self, Display},
    path::PathBuf,
};

use plotters::prelude::{BitMapBackend, IntoDrawingArea, SVGBackend};
use rfd::FileDialog;

use crate::{
    plot::{self, PlotProjection},
    point::Point,
};

#[derive(Debug)]
pub enum ExportError {
    NoFileSelected,
    NoExtensionProvided,
    ExtensionInvalidUnicode,
    ExtensionNotRecognized,
}

impl Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportError::NoFileSelected => write!(f, "no file was selected"),
            ExportError::NoExtensionProvided => write!(
                f,
                "selected file has no extension. a suitable file type can not be selected"
            ),
            ExportError::ExtensionInvalidUnicode => write!(
                f,
                "selected file has an extension that is not valid unicode"
            ),
            ExportError::ExtensionNotRecognized => {
                write!(f, "selected file has an unrecognized extensions")
            }
        }
    }
}

// TODO: use anyhow??
impl Error for ExportError {}

pub fn export(
    projection: PlotProjection,
    points: &[Point],
    size: (u32, u32),
) -> Result<PathBuf, ExportError> {
    let selected = FileDialog::new()
        .add_filter("Vector Graphics", &["svg"])
        .add_filter("Bitmap Raster", &["png", "jpg", "bmp"])
        .set_title("Export current view")
        .save_file()
        .ok_or(ExportError::NoFileSelected)?;

    let extension = selected
        .extension()
        .ok_or(ExportError::NoExtensionProvided)?;
    let extension = extension
        .to_str()
        .ok_or(ExportError::ExtensionInvalidUnicode)?;

    match extension {
        "svg" => {
            let drawing_area = SVGBackend::new(&selected, size).into_drawing_area();

            plot::draw_plot(drawing_area, projection, points);
        }
        "png" | "jpg" | "bmp" => {
            let drawing_area = BitMapBackend::new(&selected, size).into_drawing_area();

            plot::draw_plot(drawing_area, projection, points);
        }
        _ => return Err(ExportError::ExtensionNotRecognized),
    };

    Ok(selected)
}
