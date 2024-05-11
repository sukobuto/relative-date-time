use std::str::FromStr;
use chrono::{DateTime, TimeZone, Utc};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if input.is_empty() {
        return Err(anyhow::anyhow!("input is empty"));
    }
    let timestamp = i64::from_str(input.trim())?;
    let date_time: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).unwrap();
    let now = Utc::now();
    println!("{}", relative_date_time(date_time, now));
    
    Ok(())
}

fn relative_date_time(date_time: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let duration = now.signed_duration_since(date_time);
    if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes ago", duration.num_minutes())
    } else {
        "just now".to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use chrono::FixedOffset;
    use super::*;

    fn dt(s: &str) -> DateTime<FixedOffset> {
        chrono::DateTime::from_str(s).unwrap()
    }
    
    #[test]
    fn relative_date_time_test_just_now() {
        let now = dt("2021-01-01T00:00:00+00:00");
        let date_time = dt("2021-01-01T00:00:00+00:00");
        assert_eq!(relative_date_time(date_time.to_utc(), now.to_utc()), "just now");
    }

    #[test]
    fn relative_date_time_test_minute() {
        let now = dt("2021-01-01T00:00:00+00:00");
        let date_time = dt("2020-12-31T23:59:00+00:00");
        assert_eq!(relative_date_time(date_time.to_utc(), now.to_utc()), "1 minutes ago");
    }
    
    #[test]
    fn relative_date_time_test_hour() {
        let now = dt("2021-01-01T00:00:00+00:00");
        let date_time = dt("2020-12-31T23:00:00+00:00");
        assert_eq!(relative_date_time(date_time.to_utc(), now.to_utc()), "1 hours ago");
    }
    
    #[test]
    fn relative_date_time_test_day() {
        let now = dt("2021-01-01T00:00:00+00:00");
        let date_time = dt("2020-12-31T00:00:00+00:00");
        assert_eq!(relative_date_time(date_time.to_utc(), now.to_utc()), "1 days ago");
    }
}
