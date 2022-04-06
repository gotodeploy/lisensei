use macroquad_text::Fonts;

pub fn load_font(font_file: &[u8]) -> Fonts {
    let mut font = Fonts::default();
    font.load_font_from_bytes(font_file).unwrap();
    font
}
