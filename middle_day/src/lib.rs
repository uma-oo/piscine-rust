use chrono::prelude::*;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let mut is_leap = false;
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                is_leap = true;
            }
        } else {
            is_leap = true;
        }
    }

    if is_leap {
        return None;
    }

    let day = NaiveDate::from_yo_opt(year as i32, 183).unwrap();
    Some(day.weekday())
}
