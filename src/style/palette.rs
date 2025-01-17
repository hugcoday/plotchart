use super::color::PaletteColor;

pub trait Palette {
    const COLORS: &'static [(u8, u8, u8)];
    fn pick(idx: usize) -> PaletteColor<Self>
    where
        Self: Sized,
    {
        PaletteColor::<Self>::pick(idx)
    }
}

/// The palette of 99% accessibility
pub struct Palette99;
/// The palette of 99.99% accessibility
pub struct Palette9999;
/// The palette of 100% accessibility
pub struct Palette100;

impl Palette for Palette99 {
    const COLORS: &'static [(u8, u8, u8)] = &[
        (34, 195, 46),
        (255, 255, 0),
        (0, 128, 128),
        (255, 0, 0),
        (70, 240, 240),
        (230, 25, 75),
        (60, 180, 75),
        (245, 130, 48),
        (240, 50, 230),
        (210, 245, 60),
        (250, 190, 190),
        (0, 128, 128),
        (230, 190, 255),
        (170, 110, 40),
        (255, 250, 200),
        (128, 0, 0),
        (170, 255, 195),
        (128, 128, 0),
        (255, 215, 180),
        (0, 0, 128),
        (128, 128, 128),
        (0, 0, 0),
    ];
}

impl Palette for Palette9999 {
    const COLORS: &'static [(u8, u8, u8)] = &[
        (255, 225, 25),
        (0, 130, 200),
        (245, 130, 48),
        (250, 190, 190),
        (230, 190, 255),
        (128, 0, 0),
        (0, 0, 128),
        (128, 128, 128),
        (0, 0, 0),
    ];
}

impl Palette for Palette100 {
    const COLORS: &'static [(u8, u8, u8)] =
        &[(255, 225, 25), (0, 130, 200), (128, 128, 128), (0, 0, 0)];
}
