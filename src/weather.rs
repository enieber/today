use today::{day_weekly, date_string};
use serde::{Serialize, Deserialize};
use std::env;

pub async fn gererate_day_weather() -> String {
    // city code based in in cptec //  244 = sao paulo
    let code_city = env::var("CODE_CITY").unwrap_or("244".to_string());
    let response_by_city_result = get_weather_city(code_city).await;
    match response_by_city_result {
        Ok(response_by_city) => {
            let weekly_text = day_weekly();
            let city = response_by_city.cidade;
            let clima = response_by_city.clima[0].clone();
            let max = clima.max.clone();
            let min = clima.min.clone();
            format!("{weekly_text} - {city} - max: {max} ºC min: {min} ºC")
        },
        Err(_err) => {
            let weekly_text = day_weekly();
            let date = date_string();
            format!("{weekly_text} - {date}")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResponseWeatherCity {
    cidade: String,
    clima: Vec<WeatherCity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WeatherCity {
    min: String,
    max: String,
    condicao_desc: String,
}

async fn get_weather_city(code_city: String) -> Result<ResponseWeatherCity, String> {
    let url = format!("https://brasilapi.com.br/api/cptec/v1/clima/previsao/{code_city}");
    let response_await = reqwest::get(&url).await;
    match (response_await) {
        Ok(response) => {
            let response_parse_await = response.json::<ResponseWeatherCity>().await;
            match response_parse_await {
                Ok(response_parse) => Ok(response_parse),
                Err(_err) => Err("Fail to parse".to_string())
            }
        },
      Err(_err) => Err("Fail to request".to_string())
    }
}
