use ratatui::style::Color;

#[derive(Clone, Copy, Debug)]
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
            from_hex(0x030412), // NIGHT_BLUE
            from_hex(0xD95269), // LIGHT_PINK
            from_hex(0x000004), // MIDNIGHT_BLACK
            from_hex(0x0C0927), // DEEP_NAVY
            from_hex(0x231151), // DARK_INDIGO
            from_hex(0x410F75), // ROYAL_PURPLE
            from_hex(0x5F187F), // PLUM
            from_hex(0x7B2382), // VIOLET
            from_hex(0x982D80), // MAUVE
            from_hex(0xB63679), // CRIMSON
            from_hex(0xD3436E), // RUBY_RED
            from_hex(0xEB5760), // SCARLET
            from_hex(0xF8765C), // SUNSET_ORANGE
            from_hex(0xFD9A6A), // CORAL
            from_hex(0xFEBF84), // PEACH
            from_hex(0xFDDC9E), // APRICOT
            from_hex(0xFCF2B0), // LIGHT_GOLD
            from_hex(0xFCFDBF), // PALE_YELLOW
        ],
    }
}