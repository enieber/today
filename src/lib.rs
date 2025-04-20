use chrono::{Datelike, Local};
use std::fmt::Write;

// ddmmYY.md = 8 chars
pub fn generate_name_file_today() -> String {
    let now = Local::now();
    let mut nome_arquivo = String::with_capacity(10);

    write!(
        &mut nome_arquivo,
        "{:02}{:02}{:02}.md",
        now.day(),
        now.month(),
        now.year()
    )
    .expect("Falha ao formatar a data");

    nome_arquivo
}

pub fn day_weekly() -> String {
    let current_time = chrono::offset::Local::now();
    let weekly = format!("{}", current_time.date().weekday());
    weekly
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
