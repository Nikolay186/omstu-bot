use chrono::*;
use reqwest::Response;
use serde_json::Value;
use std::ops::{Add, Sub};

pub async fn get_json(group: String) -> Response {
    let group_id = search_group_id(group).await.1;
    let (week_start, week_end) = get_dates();

    let url = format!(
        "https://rasp.omgtu.ru/api/schedule/group/{id}?start={start}&finish={end}&lng=1",
        id = group_id,
        start = week_start,
        end = week_end
    );

    reqwest::get(&url).await.unwrap()
}

async fn search_group_id(group: String) -> (String, String) {
    let url = format!(
        "https://rasp.omgtu.ru/api/search?term={s_group}&type=group",
        s_group = group
    );
    let result = reqwest::get(&url).await.unwrap();

    let data: Value = serde_json::from_str(&result.text().await.unwrap()).unwrap();

    (data[0]["label"].to_string(), data[0]["id"].to_string())
}

fn get_dates() -> (String, String) {
    let days_from_mon = Utc::now().weekday().num_days_from_monday();
    let week_start = Utc::now()
        .sub(Duration::days(days_from_mon as i64))
        .format("%Y.%m.%d");

    let days_to_sat = 5 - days_from_mon + 7;
    let week_end = Utc::now()
        .add(Duration::days(days_to_sat as i64))
        .format("%Y.%m.%d");

    (week_start.to_string(), week_end.to_string())
}
