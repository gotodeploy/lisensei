use std::env;
use std::fs::File;
use std::io::Error;
use std::process::Command;

fn main() -> Result<(), Error> {
    // Belows are unexpectedly needed
    option_env!("PATH_ASSETS");
    option_env!("DOWNLOAD_ASSETS");
    option_env!("URL_MOEDICT");
    option_env!("URL_FONT");

    if env::var_os("PATH_ASSETS").is_none() {
        println!("cargo:rustc-env=PATH_ASSETS=tests/assets/");
    }

    // Ignore this script if the environment variable set for cargo test
    if env::var_os("DOWNLOAD_ASSETS").is_none() {
        println!("cargo:warning=Skipped assets downloading process.");
        return Ok(());
    }

    // Download Moe dictionary, a Chinese dictionary distributed by the government of Taiwan a.k.a g0v
    // under CC BY-NC-ND 4.0
    let url_default_moedict =
        "https://github.com/g0v/moedict-data-csld/raw/master/中華語文大辭典全稿-20160620.csv"
            .to_string();
    let url_moedict: String = env::var("URL_MOEDICT").unwrap_or(url_default_moedict);

    // Download a CNS11643 font distributed by the government of Taiwan under Open Government Data License
    let url_default_font =
        "https://github.com/m80126colin/cns11643/raw/main/data/Fonts/TW-Kai-98_1.ttf".to_string();
    let url_font: String = env::var("URL_FONT").unwrap_or(url_default_font);

    Command::new("mkdir")
        .args(&["-p", "assets"])
        .status()
        .unwrap();
    Command::new("wget")
        .args(&[url_font.as_str(), "-nc", "-O", "assets/font.ttf"])
        .status()
        .unwrap();
    Command::new("wget")
        .args(&[url_moedict.as_str(), "-nc", "-O", "assets/raw_dict.csv"])
        .status()
        .unwrap();

    Command::new("sqlite3")
        .args(&[
            "-csv",
            "./assets/dict.sqlite",
            ".import assets/raw_dict.csv raw_dict",
        ])
        .status()
        .unwrap();

    Command::new("sqlite3")
        .args(&[
            "./assets/dict.sqlite",
            concat!(
                "CREATE TABLE moe_words AS SELECT ",
                "正體字形 AS title,",
                "臺灣音讀 AS bopomofo,",
                "釋義１ AS definition ",
                "FROM raw_dict;",
                "DROP TABLE raw_dict;",
            ),
        ])
        .status()
        .unwrap();

    Command::new("sqlite3")
        .args(&[
            "-header",
            "-csv",
            "./assets/dict.sqlite",
            "SELECT * FROM moe_words;",
        ])
        .stdout(File::create("./assets/dict.csv")?)
        .spawn()
        .unwrap();

    Ok(())
}
