mod bopomofo;
mod font;
mod moedict;

use std::convert::TryInto;

use macroquad::audio::*;
use macroquad::prelude::*;
use rust_embed::RustEmbed;

use bopomofo::Bopomofo;
use bopomofo::BopomofoSound;
use font::load_font;
use moedict::MoeDictionary;

#[derive(RustEmbed)]
#[folder = "$PATH_ASSETS"]
#[include = "dict.csv"]
#[include = "font.ttf"]
#[include = "audio/F*.WAV"]
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
    let mut sounds: Vec<(Bopomofo, Sound)> = Vec::new();
    for i in 1..37 {
        let file_audio = Asset::get(format!("audio/F{i}.WAV").as_str()).unwrap();
        sounds.push((
            Bopomofo::from(i),
            load_sound_from_bytes(file_audio.data.as_ref())
                .await
                .unwrap(),
        ));
    }
    let bopomofo_sound = BopomofoSound::new(sounds.try_into().unwrap());

    let file_font = Asset::get("font.ttf").unwrap();
    let font = load_font(file_font.data.as_ref());
    let file_moedict = Asset::get("dict.csv").unwrap();
    let moedict: MoeDictionary = MoeDictionary::from_csv(file_moedict.data.as_ref());

    let mut word = moedict.choose_word();
    let mut bopomofo_input = String::new();

    loop {
        if word.bopomofo() == bopomofo_input {
            word = moedict.choose_word();
            bopomofo_input = String::new();
        }

        let pressed: char = Bopomofo::from(get_char_pressed().unwrap_or_default()).into();

        if word
            .bopomofo()
            .starts_with(&format!("{}{}", bopomofo_input, pressed))
        {
            bopomofo_sound.play(&Bopomofo::from(pressed));
            bopomofo_input.push(pressed)
        }

        font.draw_text(word.title(), 20.0, 0.0, 70, WHITE);
        font.draw_text(word.bopomofo().as_str(), 40.0, 100.0, 30, WHITE);
        font.draw_text(&bopomofo_input, 40.0, 150.0, 30, WHITE);

        next_frame().await
    }
}
