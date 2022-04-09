extern crate csv;
extern crate serde;

use macroquad::miniquad::date;
use macroquad::rand;
use macroquad::rand::ChooseRandom;
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
    moe_words: Vec<MoeWord>,
}

impl MoeDictionary {
    pub fn choose_word(&self) -> &MoeWord {
        &self.moe_words.choose().unwrap()
    }

    pub fn from_csv(moedict: &[u8]) -> Self {
        let mut reader = csv::Reader::from_reader(moedict);

        rand::srand(date::now() as _);

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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_load_moedict_from_csv() {
        let dict_source = [
            "title,bopomofo,definition",
            "一,丨,自然数",
            "一一,丨　丨,自然数",
        ]
        .join("\n");
        let moe_words = MoeDictionary::from_csv(dict_source.as_bytes()).moe_words;
        let moe_words_expected: Vec<MoeWord> = vec![
            MoeWord {
                title: "一".to_string(),
                bopomofo: "丨".to_string(),
                definition: "自然数".to_string(),
            },
            MoeWord {
                title: "一一".to_string(),
                bopomofo: "丨　丨".to_string(),
                definition: "自然数".to_string(),
            },
        ];

        assert_eq!(moe_words, moe_words_expected);
    }

    #[rstest]
    #[case('v', 'ㄒ')]
    #[case('8', 'ㄚ')]
    #[case(' ', '　')]
    #[case('%', '%')]
    fn test_alphabet_to_bopomofo(#[case] input: char, #[case] expected: char) {
        assert_eq!(alphabet_to_bopomofo(input), expected);
    }
}
