use chrono::{NaiveDate, Datelike, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    if is_leap_year(year) {
        return None;
    } 
    let jan_first = NaiveDate::from_ymd_opt(year, 1, 1)?;
    let middle_day = jan_first + chrono::Duration::days(182);
    Some(middle_day.weekday())

}

fn is_leap_year(year: i32) -> bool {
    NaiveDate::from_ymd_opt(year, 2, 29).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = middle_day(2023);
        assert_eq!(result, Some(wd::Sun));
    }
}
