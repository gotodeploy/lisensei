use std::fmt;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Bopomofo {
    B,     // ㄅ
    P,     // ㄆ
    M,     // ㄇ
    F,     // ㄈ
    D,     // ㄉ
    T,     // ㄊ
    N,     // ㄋ
    L,     // ㄌ
    G,     // ㄍ
    K,     // ㄎ
    H,     // ㄏ
    J,     // ㄐ
    Q,     // ㄑ
    X,     // ㄒ
    Zh,    // ㄓ
    Ch,    // ㄔ
    Sh,    // ㄕ
    R,     // ㄖ
    Z,     // ㄗ
    C,     // ㄘ
    S,     // ㄙ
    A,     // ㄚ
    O,     // ㄛ
    E,     // ㄜ
    Eh,    // ㄝ
    Ai,    // ㄞ
    Ei,    // ㄟ
    Au,    // ㄠ
    Ou,    // ㄡ
    An,    // ㄢ
    En,    // ㄣ
    Ang,   // ㄤ
    Eng,   // ㄥ
    Er,    // ㄦ
    I,     // ㄧ
    U,     // ㄨ
    Iu,    // ㄩ
    Tone1, // ˉ
    Tone2, // ˊ
    Tone3, // ˇ
    Tone4, // ˋ
    Tone5, // ˙
}

impl From<Bopomofo> for char {
    fn from(bopomofo: Bopomofo) -> Self {
        match bopomofo {
            Bopomofo::B => 'ㄅ',
            Bopomofo::P => 'ㄆ',
            Bopomofo::M => 'ㄇ',
            Bopomofo::F => 'ㄈ',
            Bopomofo::D => 'ㄉ',
            Bopomofo::T => 'ㄊ',
            Bopomofo::N => 'ㄋ',
            Bopomofo::L => 'ㄌ',
            Bopomofo::G => 'ㄍ',
            Bopomofo::K => 'ㄎ',
            Bopomofo::H => 'ㄏ',
            Bopomofo::J => 'ㄐ',
            Bopomofo::Q => 'ㄑ',
            Bopomofo::X => 'ㄒ',
            Bopomofo::Zh => 'ㄓ',
            Bopomofo::Ch => 'ㄔ',
            Bopomofo::Sh => 'ㄕ',
            Bopomofo::R => 'ㄖ',
            Bopomofo::Z => 'ㄗ',
            Bopomofo::C => 'ㄘ',
            Bopomofo::S => 'ㄙ',
            Bopomofo::A => 'ㄚ',
            Bopomofo::O => 'ㄛ',
            Bopomofo::E => 'ㄜ',
            Bopomofo::Eh => 'ㄝ',
            Bopomofo::Ai => 'ㄞ',
            Bopomofo::Ei => 'ㄟ',
            Bopomofo::Au => 'ㄠ',
            Bopomofo::Ou => 'ㄡ',
            Bopomofo::An => 'ㄢ',
            Bopomofo::En => 'ㄣ',
            Bopomofo::Ang => 'ㄤ',
            Bopomofo::Eng => 'ㄥ',
            Bopomofo::Er => 'ㄦ',
            Bopomofo::I => 'ㄧ',
            Bopomofo::U => 'ㄨ',
            Bopomofo::Iu => 'ㄩ',
            Bopomofo::Tone1 => 'ˉ',
            Bopomofo::Tone2 => 'ˊ',
            Bopomofo::Tone3 => 'ˇ',
            Bopomofo::Tone4 => 'ˋ',
            Bopomofo::Tone5 => '˙',
        }
    }
}

impl From<char> for Bopomofo {
    fn from(character: char) -> Self {
        match character {
            'ㄅ' => Bopomofo::B,
            '1' => Bopomofo::B,
            'ㄆ' => Bopomofo::P,
            'q' => Bopomofo::P,
            'ㄇ' => Bopomofo::M,
            'a' => Bopomofo::M,
            'ㄈ' => Bopomofo::F,
            'z' => Bopomofo::F,
            'ㄉ' => Bopomofo::D,
            '2' => Bopomofo::D,
            'ㄊ' => Bopomofo::T,
            'w' => Bopomofo::T,
            'ㄋ' => Bopomofo::N,
            's' => Bopomofo::N,
            'ㄌ' => Bopomofo::L,
            'x' => Bopomofo::L,
            'ㄍ' => Bopomofo::G,
            'e' => Bopomofo::G,
            'ㄎ' => Bopomofo::K,
            'd' => Bopomofo::K,
            'ㄏ' => Bopomofo::H,
            'c' => Bopomofo::H,
            'ㄐ' => Bopomofo::J,
            'r' => Bopomofo::J,
            'ㄑ' => Bopomofo::Q,
            'f' => Bopomofo::Q,
            'ㄒ' => Bopomofo::X,
            'v' => Bopomofo::X,
            'ㄓ' => Bopomofo::Zh,
            '5' => Bopomofo::Zh,
            'ㄔ' => Bopomofo::Ch,
            't' => Bopomofo::Ch,
            'ㄕ' => Bopomofo::Sh,
            'g' => Bopomofo::Sh,
            'ㄖ' => Bopomofo::R,
            'b' => Bopomofo::R,
            'ㄗ' => Bopomofo::Z,
            'y' => Bopomofo::Z,
            'ㄘ' => Bopomofo::C,
            'h' => Bopomofo::C,
            'ㄙ' => Bopomofo::S,
            'n' => Bopomofo::S,
            'ㄚ' => Bopomofo::A,
            '8' => Bopomofo::A,
            'ㄛ' => Bopomofo::O,
            'i' => Bopomofo::O,
            'ㄜ' => Bopomofo::E,
            'k' => Bopomofo::E,
            'ㄝ' => Bopomofo::Eh,
            ',' => Bopomofo::Eh,
            'ㄞ' => Bopomofo::Ai,
            '9' => Bopomofo::Ai,
            'ㄟ' => Bopomofo::Ei,
            'o' => Bopomofo::Ei,
            'ㄠ' => Bopomofo::Au,
            'l' => Bopomofo::Au,
            'ㄡ' => Bopomofo::Ou,
            '.' => Bopomofo::Ou,
            'ㄢ' => Bopomofo::An,
            '0' => Bopomofo::An,
            'ㄣ' => Bopomofo::En,
            'p' => Bopomofo::En,
            'ㄤ' => Bopomofo::Ang,
            ';' => Bopomofo::Ang,
            'ㄥ' => Bopomofo::Eng,
            '?' => Bopomofo::Eng,
            'ㄦ' => Bopomofo::Er,
            '-' => Bopomofo::Er,
            'ㄧ' => Bopomofo::I,
            'u' => Bopomofo::I,
            'ㄨ' => Bopomofo::U,
            'j' => Bopomofo::U,
            'ㄩ' => Bopomofo::Iu,
            'm' => Bopomofo::Iu,
            'ˉ' => Bopomofo::Tone1,
            ' ' => Bopomofo::Tone1,
            'ˊ' => Bopomofo::Tone2,
            '6' => Bopomofo::Tone2,
            'ˇ' => Bopomofo::Tone3,
            '3' => Bopomofo::Tone3,
            'ˋ' => Bopomofo::Tone4,
            '4' => Bopomofo::Tone4,
            '˙' => Bopomofo::Tone5,
            '7' => Bopomofo::Tone5,
            _ => Bopomofo::Tone1,
        }
    }
}

impl fmt::Display for Bopomofo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bopomofo: char = (*self).into();
        write!(f, "{}", bopomofo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case('v', 'ㄒ')]
    #[case('8', 'ㄚ')]
    #[case(' ', 'ˉ')]
    fn test_alphabet_to_bopomofo(#[case] input: char, #[case] expected: char) {
        let bopomofo: char = Bopomofo::from(input).into();
        assert_eq!(bopomofo, expected);
    }
}
