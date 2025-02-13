use iced::Color;

/// Nord theme color palette - Polar Night
/// These represent the darker, background shades
pub struct NordTheme;

impl NordTheme {
    // base dark shade, used for backgrounds
    pub const NORD0: Color = Color::from_rgb(
        0x2E as f32 / 255.0,
        0x34 as f32 / 255.0,
        0x40 as f32 / 255.0,
    );

    // lighter dark shade, used for UI elements
    pub const NORD1: Color = Color::from_rgb(
        0x3B as f32 / 255.0,
        0x42 as f32 / 255.0,
        0x52 as f32 / 255.0,
    );

    // even lighter shade, good for active elements
    pub const NORD2: Color = Color::from_rgb(
        0x43 as f32 / 255.0,
        0x4C as f32 / 255.0,
        0x5E as f32 / 255.0,
    );

    // brightest dark shade, used for text
    pub const NORD3: Color = Color::from_rgb(
        0x4C as f32 / 255.0,
        0x56 as f32 / 255.0,
        0x6A as f32 / 255.0,
    );
}
