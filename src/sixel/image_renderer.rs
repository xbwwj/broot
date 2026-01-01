use std::io::Write;

use crokey::crossterm::{QueueableCommand, cursor, style::Color};
use icy_sixel::{EncodeOptions, sixel_encode};

use crate::{errors::ProgramError, image::zune_compat::DynamicImage};

pub type W = std::io::BufWriter<std::io::Stderr>;

// TODO: previous sixel image is not cleared
pub fn try_print_image(
    w: &mut W,
    src: &DynamicImage,
    width: usize,
    height: usize,
    left: u16,
    top: u16,
    _bg: Color,
) -> Result<(), ProgramError> {
    let encoded = match src.as_rgba8() {
        Some(rgba8) => sixel_encode(&rgba8.as_raw(), width, height, &EncodeOptions::default())
            .map_err(|e| ProgramError::ImageError {
                details: e.to_string(),
            }),
        None => {
            let rgba = src.to_rgba8();
            sixel_encode(&rgba.as_raw(), width, height, &EncodeOptions::default()).map_err(|e| {
                ProgramError::ImageError {
                    details: e.to_string(),
                }
            })
        }
    }?;
    w.queue(cursor::MoveTo(left, top))?;
    write!(w, "{encoded}")?;
    Ok(())
}
