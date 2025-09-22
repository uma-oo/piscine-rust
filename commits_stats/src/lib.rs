use json;
use std::collections::HashMap;
use chrono::prelude::*;
use chrono::{ NaiveDate, Datelike };
pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_week: HashMap<String, u32> = HashMap::new();
    for i in 0..data.len() {
        let datetime = DateTime::parse_from_rfc3339(&data[i]["commit"]["author"]["date"].to_string()).unwrap().with_timezone(&Utc);
        let year = datetime.year();
        let month = datetime.month();
        let day = datetime.day();
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let iso_week = date.iso_week();
        let nbr_week = iso_week.week();
        let key = format!("{}-W{}", year, nbr_week); 
        let commits = commits_week.entry(key).or_insert(0);
        *commits += 1;
    }
    commits_week
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_users: HashMap<String, u32> = HashMap::new();
    for i in 0..data.len() {
        let commits = commits_users.entry(data[i]["author"]["login"].to_string()).or_insert(0);
        *commits += 1;
    }

    commits_users
}
