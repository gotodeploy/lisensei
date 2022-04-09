extern crate csv;
extern crate serde;

use serde::Deserialize;

#[derive(PartialEq, Eq, Debug, Deserialize)]
pub struct MoeWord {
    pub title: String,      // 正體字形
    pub bopomofo: String,   // 臺灣音讀
    pub definition: String, // 釋義１
}

impl MoeWord {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn bopomofo(&self) -> &String {
        &self.bopomofo
    }

    pub fn definition(&self) -> &String {
        &self.definition
    }
}

pub struct MoeDictionary {
    pub moe_words: Vec<MoeWord>,
}

impl MoeDictionary {
    pub fn from_csv(moedict: &[u8]) -> Self {
        let mut reader = csv::Reader::from_reader(moedict);

        MoeDictionary {
            moe_words: reader
                .deserialize()
                .map(|result| result.unwrap())
                .collect::<Vec<MoeWord>>(),
        }
    }
}

pub fn alphabet_to_bopomofo(character: char) -> char {
    match character {
        '1' => 'ㄅ',
        'q' => 'ㄆ',
        'a' => 'ㄇ',
        'z' => 'ㄈ',
        '2' => 'ㄉ',
        'w' => 'ㄊ',
        's' => 'ㄋ',
        'x' => 'ㄌ',
        'e' => 'ㄍ',
        'd' => 'ㄎ',
        'c' => 'ㄏ',
        'r' => 'ㄐ',
        'f' => 'ㄑ',
        'v' => 'ㄒ',
        '5' => 'ㄓ',
        't' => 'ㄔ',
        'g' => 'ㄕ',
        'b' => 'ㄖ',
        'y' => 'ㄗ',
        'h' => 'ㄘ',
        'n' => 'ㄙ',
        'u' => 'ㄧ',
        'j' => 'ㄨ',
        'm' => 'ㄩ',
        '8' => 'ㄚ',
        'i' => 'ㄛ',
        'k' => 'ㄜ',
        ',' => 'ㄝ',
        '9' => 'ㄞ',
        'o' => 'ㄟ',
        'l' => 'ㄠ',
        '.' => 'ㄡ',
        '0' => 'ㄢ',
        'p' => 'ㄣ',
        ';' => 'ㄤ',
        '/' => 'ㄥ',
        '-' => 'ㄦ',
        ' ' => '　',
        '6' => 'ˊ',
        '3' => 'ˇ',
        '4' => 'ˋ',
        '7' => '˙',
        _ => character,
    }
}
