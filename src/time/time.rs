use super::r#type::from_env_var;
use std::fmt::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Leap Year
pub static LEAP_YEAR: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
/// Common Year
pub static COMMON_YEAR: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
/// Days
pub static DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
/// Months
pub static MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Determines if a year is a leap year.
///
/// # Parameters
/// `u64`: The year
///
/// # Returns
/// `bool`: Whether it is a leap year
fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Calculates the current year, month, day, and the number of seconds remaining in the day.
///
/// # Returns
/// A tuple containing:
/// - `year`: The current year
/// - `month`: The current month
/// - `day`: The current day
/// - `remaining_seconds`: The number of seconds passed today
fn calculate_current_date() -> (u64, u64, u64, u64) {
    // Get the current time
    let start: SystemTime = SystemTime::now();
    let duration: Duration = start.duration_since(UNIX_EPOCH).unwrap();
    let total_seconds: u64 = duration.as_secs();
    let mut total_days: u64 = total_seconds / 86400;
    let mut year: u64 = 1970;
    while total_days >= if is_leap_year(year) { 366 } else { 365 } {
        total_days -= if is_leap_year(year) { 366 } else { 365 };
        year += 1;
    }
    let mut month: u64 = 1;
    let month_days: [u64; 12] = if is_leap_year(year) {
        LEAP_YEAR
    } else {
        COMMON_YEAR
    };
    while total_days >= month_days[month as usize - 1] {
        total_days -= month_days[month as usize - 1];
        month += 1;
    }
    let day: u64 = total_days + 1;
    let remaining_seconds: u64 = total_seconds % 86400;
    (year, month, day, remaining_seconds)
}

/// Gets the current time, including the date and time.
///
/// # Returns
/// `String`: The formatted time as "YYYY-MM-DD HH:MM:SS"
pub fn current_time() -> String {
    let (year, month, day, remaining_seconds) = calculate_current_date();
    let timezone_offset: u64 = from_env_var().value(); // Assuming from_env_var() is defined elsewhere
    let hours: u64 = ((remaining_seconds + timezone_offset) / 3600) % 24;
    let minutes: u64 = (remaining_seconds % 3600) / 60;
    let seconds: u64 = remaining_seconds % 60;
    let mut date_time: String = String::new();
    write!(
        &mut date_time,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
    .unwrap_or_default();
    date_time
}

/// Gets the current day, without the time.
///
/// # Returns
/// `String`: The formatted date as "YYYY-MM-DD"
pub fn current_date() -> String {
    let (year, month, day, _) = calculate_current_date();
    let mut date_time: String = String::new();
    write!(&mut date_time, "{:04}-{:02}-{:02}", year, month, day).unwrap_or_default();
    date_time
}

/// Computes the year, month, and day from days since Unix epoch (1970-01-01).
///
/// - `days_since_epoch`: Number of days since `1970-01-01`.
/// - Returns: `(year, month, day)`
fn compute_date(mut days_since_epoch: u64) -> (u64, u64, u64) {
    let mut year: u64 = 1970;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days_since_epoch < days_in_year {
            break;
        }
        days_since_epoch -= days_in_year as u64;
        year += 1;
    }
    let mut month: u64 = 0;
    for (i, &days) in COMMON_YEAR.iter().enumerate() {
        let days_in_month = if i == 1 && is_leap_year(year) {
            days + 1
        } else {
            days
        };
        if days_since_epoch < days_in_month as u64 {
            month = i as u64 + 1;
            return (year, month, (days_since_epoch + 1) as u64);
        }
        days_since_epoch -= days_in_month as u64;
    }

    (year, month, 1)
}

pub fn current_date_gmt() -> String {
    let now: SystemTime = SystemTime::now();
    let duration_since_epoch: Duration = now.duration_since(UNIX_EPOCH).unwrap();
    let timestamp: u64 = duration_since_epoch.as_secs();
    let seconds_in_day: u64 = 86_400;
    let days_since_epoch: u64 = timestamp / seconds_in_day;
    let seconds_of_day: u64 = timestamp % seconds_in_day;
    let hours: u64 = (seconds_of_day / 3600) as u64;
    let minutes: u64 = ((seconds_of_day % 3600) / 60) as u64;
    let seconds: u64 = (seconds_of_day % 60) as u64;
    let (year, month, day) = compute_date(days_since_epoch);
    let weekday: usize = ((days_since_epoch + 4) % 7) as usize;
    format!(
        "{}, {:02} {} {} {:02}:{:02}:{:02} GMT",
        DAYS[weekday],
        day,
        MONTHS[month as usize - 1],
        year,
        hours,
        minutes,
        seconds
    )
}
