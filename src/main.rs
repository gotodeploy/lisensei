mod font;
mod moedict;

use lisensei::moedict::MoeDictionary;
use macroquad::miniquad::date;
use macroquad::prelude::*;
use macroquad::rand::*;
use rust_embed::RustEmbed;

use font::load_font;
use moedict::alphabet_to_bopomofo;

#[derive(RustEmbed)]
#[folder = "$PATH_ASSETS"]
#[exclude = "raw_dict.csv"]
struct Asset;

fn window_conf() -> Conf {
    Conf {
        window_title: "李先生".to_string(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let file_font = Asset::get("font.ttf").unwrap();
    let file_moedict = Asset::get("dict.csv").unwrap();
    let font = load_font(file_font.data.as_ref());
    let moedict = MoeDictionary::from_csv(file_moedict.data.as_ref()).moe_words;

    rand::srand(date::now() as _);
    let mut word = moedict.choose().unwrap();
    let mut bopomofo = &word.bopomofo;
    let mut bopomofo_input = String::new();

    loop {
        if bopomofo == &bopomofo_input {
            word = moedict.choose().unwrap();
            bopomofo_input = String::new();
            bopomofo = &word.bopomofo;
        }

        let pressed = get_char_pressed().unwrap_or_default();
        let pressed_bopomofo = alphabet_to_bopomofo(pressed);

        if bopomofo.starts_with(&format!("{}{}", bopomofo_input, pressed_bopomofo)) {
            bopomofo_input.push(pressed_bopomofo)
        }

        font.draw_text(&word.title, 20.0, 0.0, 70, WHITE);
        font.draw_text(&word.bopomofo, 40.0, 100.0, 30, WHITE);
        font.draw_text(&bopomofo_input, 40.0, 150.0, 30, WHITE);

        next_frame().await
    }
}
