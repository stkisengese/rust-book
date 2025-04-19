use std::collections::HashMap;
use chrono::{DateTime, Datelike, Utc};
use json::JsonValue;


pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for commit in data.members() {
        if let Some(author) = commit["author"]["login"].as_str() {
            *map.entry(author.to_string()).or_insert(0) += 1;
        }
    }

    map
}

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                let date_utc = date.with_timezone(&Utc);
                let iso_week = date_utc.iso_week();
                let week_str = format!("{}-W{}", iso_week.year(), iso_week.week());
                *map.entry(week_str).or_insert(0) += 1;
            }
        }
    }
    map
}