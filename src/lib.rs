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
    let weekday = current_time.date().weekday();

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
