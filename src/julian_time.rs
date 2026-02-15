use chrono::{DateTime, Datelike, Timelike, Utc};

// This part is based on:
//     Jean Meeus - Astronomical Algorithm 2nd Edition 1998
//     Chapter 7

#[derive(Debug)]
pub struct JulianTime {
    pub utc: DateTime<Utc>,
    pub day: f64,
    pub century: f64,
}

impl JulianTime {
    pub fn new(dt: DateTime<Utc>) -> Self {
        let jd = Self::julian_day_number(dt);
        JulianTime {
            utc: dt,
            day: jd,
            century: Self::julian_centuries(jd),
        }
    }

    /// Computes the date UTC from a Julian Century
    ///
    /// * `jd` - The Julian Day Number
    pub fn to_utc(jd: f64) -> DateTime<Utc> {
        // Get the julian day of the unix epoch (1970-01-01_00:00:00 UTC)
        let epoch_dt = DateTime::from_timestamp(0, 0).expect("Invalid timestamp");
        let jd_epoch = Self::julian_day_number(epoch_dt);
        let day_in_seconds = 86400.0;

        let seconds_since_epoch = (jd - jd_epoch) * day_in_seconds;

        DateTime::from_timestamp(seconds_since_epoch as i64, 0).unwrap_or(epoch_dt)
    }

    /// Computes the current julian century from the Julian Day Number.
    ///
    /// * `jd` - The Julian Day
    fn julian_centuries(jd: f64) -> f64 {
        (jd - 2451545.0) / 36525.0
    }

    /// Convert UTC datetime to Julian Day Number.
    ///
    /// * `dt` - The UTC Datetime
    fn julian_day_number(dt: DateTime<Utc>) -> f64 {
        let year = dt.year() as f64;
        let month = dt.month() as f64;
        let day = dt.day() as f64;
        let hour = dt.hour() as f64;
        let min = dt.minute() as f64;
        let sec = dt.second() as f64;

        let decimal_day = day + (hour / 24.0) + (min / 1440.0) + (sec / 86400.0);
        let mut y = year;
        let mut m = month;
        if month <= 2.0 {
            y -= 1.0;
            m += 12.0;
        }
        let a = (y / 100.0).floor();
        let b = 2.0 - a + (a / 4.0).floor();
        let jd = (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor();

        jd + decimal_day + b - 1524.5
    }
}
