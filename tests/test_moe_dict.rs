use rstest::rstest;

use lisensei::moedict::*;

#[test]
fn test_load_moedict() {
    let dict_source = [
        "title,bopomofo,definition",
        "一,丨,自然数",
        "一一,丨　丨,自然数",
    ]
    .join("\n");
    let moedict = load_moedict(dict_source.as_bytes()).unwrap();
    let dict_expected: Vec<MoeWord> = vec![
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

    assert_eq!(moedict, dict_expected);
}

#[rstest]
#[case('v', 'ㄒ')]
#[case('8', 'ㄚ')]
#[case(' ', '　')]
#[case('%', '%')]
fn test_alphabet_to_bopomofo(#[case] input: char, #[case] expected: char) {
    assert_eq!(alphabet_to_bopomofo(input), expected);
}
