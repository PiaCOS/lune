use chrono::{DateTime, Datelike, Timelike, Utc};

// ---------------- Main ----------------

fn main() {
    let now = Utc::now();
    let jd = date_to_julian_date(now);
    let t = get_julian_centuries(jd);
    let fraction = illuminated_fraction(t);

    let d = moon_elongation(t);
    let is_waning = d > 180.0;

    let phase = match (fraction / 8.0).floor() as u8 {
        8 => Phases::from_int(0),
        n => Phases::from_int(n),
    };
    println!("{:?} - {:?} - {:?}", fraction, is_waning, phase);
}

// ---------------- Moon Stuff ----------------

#[derive(Debug)]
enum Phases {
    NewMoon,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

impl Phases {
    fn from_int(n: u8) -> Result<Self, String> {
        match n {
            0 => Ok(Phases::NewMoon),
            1 => Ok(Phases::WaxingCrescent),
            2 => Ok(Phases::FirstQuarter),
            3 => Ok(Phases::WaxingGibbous),
            4 => Ok(Phases::FullMoon),
            5 => Ok(Phases::WaningGibbous),
            6 => Ok(Phases::LastQuarter),
            7 => Ok(Phases::WaningCrescent),
            _ => Err("Invalid moon phase number".to_owned()),
        }
    }

    fn to_int(&self) -> u8 {
        match self {
            Phases::NewMoon => 0,
            Phases::WaxingCrescent => 1,
            Phases::FirstQuarter => 2,
            Phases::WaxingGibbous => 3,
            Phases::FullMoon => 4,
            Phases::WaningGibbous => 5,
            Phases::LastQuarter => 6,
            Phases::WaningCrescent => 7,
        }
    }
}

// ---------------- Astronomy ----------------

// This part is based on:
//     Jean Meeus - Astronomical Algorithm 2nd Edition 1998
//     Chapters 7, 47 and 49

/// Computes the illuminated fraction
///
/// * `t` - The Julian Time
fn illuminated_fraction(t: f64) -> f64 {
    let d = moon_elongation(t);
    let m = sun_mean_anomaly(t);
    let m_prime = moon_mean_anomaly(t);

    let i = selenocentric_elongation(d, m, m_prime);

    (1.0 + i.to_radians().cos()) / 2.0
}

/// Computes the Selenocentric Elongation
///
/// * `d` - The mean elongaion of the Moon
/// * `m` - The Sun's mean anomaly
/// * `m_prime` - The Moon's mean anomaly
fn selenocentric_elongation(d: f64, m: f64, m_prime: f64) -> f64 {
    let mut i = 180.0 - d - 6.289 * m_prime.to_radians().sin() + 2.1 * m.to_radians().sin()
        - 1.274 * (2.0 * d.to_radians() - m_prime.to_radians()).sin()
        - 0.658 * (2.0 * d.to_radians()).sin()
        - 0.214 * (2.0 * m_prime.to_radians()).sin()
        - 0.11 * d.to_radians().sin();
    map_to_deg(i)
}

/// Computes the moon elongation
///
/// * `t` - The Julian Time
fn moon_elongation(t: f64) -> f64 {
    let coefs = [
        297.8501921,
        445_267.1114034,
        -0.0018819,
        1.0 / 545868.0,
        -1.0 / 113065000.0,
    ];
    map_to_deg(polynomial_eval(t, &coefs))
}

/// Computes the sun mean anomaly
///
/// * `t` - The Julian Time
fn sun_mean_anomaly(t: f64) -> f64 {
    let coefs = [357.5291092, 35999.0502909, -0.0001536, 1.0 / 24490000.0];
    map_to_deg(polynomial_eval(t, &coefs))
}

/// Computes the moon mean anomaly
///
/// * `t` - The Julian Time
fn moon_mean_anomaly(t: f64) -> f64 {
    let coefs = [
        134.9633964,
        477198.8675055,
        0.0087414,
        1.0 / 69699.0,
        -1.0 / 14712000.0,
    ];
    map_to_deg(polynomial_eval(t, &coefs))
}

/// Convert date to julian date
fn date_to_julian_date(dt: DateTime<Utc>) -> f64 {
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

fn get_julian_centuries(jd: f64) -> f64 {
    (jd - 2451545.0) / 36525.0
}

// ---------------- Utils ----------------

/// Evaluate Polynomal with Horner's Method
fn polynomial_eval(t: f64, coefs: &[f64]) -> f64 {
    coefs.iter().rev().fold(0.0, |acc, &coef| acc * t + coef)
}

/// 360 Modulo
fn map_to_deg(angle: f64) -> f64 {
    let m = angle % 360.0;
    if m < 0.0 { m + 360.0 } else { m }
}
