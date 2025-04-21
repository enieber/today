use chrono::{Datelike, Local};
use tracing::info;
use std::fmt::Write;
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::env;

fn read_base_file() -> String {
    match env::var("TODAY_BASE_FILE") {
        Ok(val) => format!("{}", val),
        Err(_e) => format!("{}", "/tmp"),
    }
}

// ddmmYY.md = 8 chars
pub fn generate_name_file_today() -> String {
    let now = Local::now();
    let mut nome_arquivo = String::with_capacity(10);

    write!(
        &mut nome_arquivo,
        "{:02}-{:02}-{:02}.md",
        now.year(),
        now.month(),
        now.day()

    )
    .expect("Falha ao formatar a data");

    let base_file = read_base_file();
    let file = format!("{}/{}", base_file, nome_arquivo);
    info!("use file in: {}", file);
    file
}

pub fn day_weekly() -> String {
    let current_time = chrono::offset::Local::now();
    let weekday = current_time.date_naive().weekday();

    let weekday_in_portuguese = match weekday {
        chrono::Weekday::Mon => "Segunda",
        chrono::Weekday::Tue => "Terça",
        chrono::Weekday::Wed => "Quarta",
        chrono::Weekday::Thu => "Quinta",
        chrono::Weekday::Fri => "Sexta",
        chrono::Weekday::Sat => "Sábado",
        chrono::Weekday::Sun => "Domingo",
    };
    weekday_in_portuguese.to_string()
}

pub fn date_string() -> String {
    let now = Local::now();
    let mut nome_arquivo = String::with_capacity(10);

    write!(
        &mut nome_arquivo,
        "{:02}/{:02}/{:02}",
        now.day(),
        now.month(),
        now.year()
    )
    .expect("Falha ao formatar a data");

    nome_arquivo
}

pub fn create_file(file_path: &str) -> Result<File, Error> {
    let file = File::create(file_path);
    return file;
}


pub fn open_file(file_path: &str) -> Result<File, Error> {
    let file = OpenOptions::new().write(true).append(true).open(file_path);
    return file;
}
