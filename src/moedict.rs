extern crate csv;
extern crate serde;

use macroquad::miniquad::date;
use macroquad::rand;
use macroquad::rand::ChooseRandom;
use serde::Deserialize;

use crate::bopomofo::Bopomofo;

#[derive(PartialEq, Eq, Debug, Deserialize)]
pub struct MoeWord {
    title: String,      // 正體字形
    bopomofo: String,   // 臺灣音讀
    definition: String, // 釋義１
}

impl MoeWord {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn bopomofo(&self) -> String {
        self.bopomofo
            .replace('丨', &Bopomofo::I.to_string())
            .replace('，', "")
            .split('　')
            .map(|c| {
                if c.starts_with(char::from(Bopomofo::Tone5)) {
                    c.replace(char::from(Bopomofo::Tone5), "") + &Bopomofo::Tone5.to_string()
                } else {
                    match Bopomofo::from(c.chars().rev().next().unwrap()) {
                        Bopomofo::Tone2 => c.to_string(),
                        Bopomofo::Tone3 => c.to_string(),
                        Bopomofo::Tone4 => c.to_string(),
                        _ => c.to_string() + &Bopomofo::Tone1.to_string(),
                    }
                }
            })
            .collect::<String>()
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
        self.moe_words.choose().unwrap()
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
    #[case("ㄏㄨˊ　ㄌㄨㄣˊ　ㄊㄨㄣ　ㄗㄠˇ", "ㄏㄨˊㄌㄨㄣˊㄊㄨㄣˉㄗㄠˇ")]
    #[case("ㄍㄨ　˙ㄍㄨ", "ㄍㄨˉㄍㄨ˙")]
    #[case("ㄍㄨ　˙ㄋ丨ㄤ", "ㄍㄨˉㄋㄧㄤ˙")]
    #[case(
        "丨ˋ　ㄈㄣ　ㄑ丨ㄢˊ，丨ˋ　ㄈㄣ　ㄏㄨㄛˋ",
        "ㄧˋㄈㄣˉㄑㄧㄢˊㄧˋㄈㄣˉㄏㄨㄛˋ"
    )]
    fn test_bopomofo(#[case] bopomofo: String, #[case] bopomofo_expected: String) {
        let moe_word = MoeWord {
            title: "DUMMY".to_string(),
            bopomofo,
            definition: "DUMMY".to_string(),
        };

        assert_eq!(moe_word.bopomofo(), bopomofo_expected);
    }
}
