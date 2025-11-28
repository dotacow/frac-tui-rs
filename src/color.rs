use ratatui::style::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Palette {
    Classic,
    Rainbow,
    Magma,
}

pub fn from_hex(hex: u32) -> Color {
    Color::Rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub fn get_palette_colors(palette: Palette) -> Vec<Color> {
    match palette {
        Palette::Classic => vec![
            Color::Red,
            Color::LightRed,
            Color::Yellow,
            Color::LightYellow,
            Color::Green,
            Color::LightGreen,
            Color::Blue,
            Color::Cyan,
        ],
        Palette::Rainbow => vec![
            Color::Magenta,
            Color::LightMagenta,
            Color::Blue,
            Color::LightBlue,
            Color::Cyan,
            Color::Green,
            Color::Yellow,
            Color::LightRed,
        ],
        Palette::Magma => vec![
            from_hex(0x030412), from_hex(0xD95269), from_hex(0x000004),
            from_hex(0x0C0927), from_hex(0x231151), from_hex(0x410F75),
            from_hex(0x5F187F), from_hex(0x7B2382), from_hex(0x982D80),
            from_hex(0xB63679), from_hex(0xD3436E), from_hex(0xEB5760),
            from_hex(0xF8765C), from_hex(0xFD9A6A), from_hex(0xFEBF84),
            from_hex(0xFDDC9E), from_hex(0xFCF2B0), from_hex(0xFCFDBF),
        ],
    }
}