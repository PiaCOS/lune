use crate::julian_time::JulianTime;
use crate::moon::Phases;
use crate::utils;

use chrono::{DateTime, Utc};

// This part is based on:
//     Jean Meeus - Astronomical Algorithm 2nd Edition 1998
//     Chapters 47 and 49

/// Difference between uniform Astronomical Time and variable Earth Time (2026)
const DELTA_T: f64 = 70.0;

// ---------------- public ----------------

/// Computes the illuminated fraction.
///
/// * `julian` - The Julian Time
pub fn illuminated_fraction(julian: &JulianTime) -> f64 {
    let d = moon_elongation(julian);
    let m = sun_mean_anomaly(julian);
    let m_prime = moon_mean_anomaly(julian);

    let i = selenocentric_elongation(d, m, m_prime);

    (1.0 + i.to_radians().cos()) / 2.0
}

/// Computes the moon elongation.
///
/// * `julian` - The Julian Time
pub fn moon_elongation(julian: &JulianTime) -> f64 {
    let t = julian.century;
    let coefs = [
        297.8501921,
        445_267.1114034,
        -0.0018819,
        1.0 / 545868.0,
        -1.0 / 113065000.0,
    ];
    utils::map_to_deg(utils::polynomial_eval(t, &coefs))
}

pub fn get_phase(julian: &JulianTime) -> u8 {
    let fraction = illuminated_fraction(julian);
    let d = moon_elongation(julian);
    let is_waning = d > 180.0;
    if is_waning {
        4 + ((1.0 - fraction) * 4.0).floor() as u8
    } else {
        (fraction * 4.0).floor() as u8
    }
}

/// Computes the moon phases around the current date.
///
/// * `julian` - The Julian Time
pub fn phases_around(julian: &JulianTime) -> ((Phases, DateTime<Utc>), (Phases, DateTime<Utc>)) {
    let k = index_lunation(julian);

    let phase_index = ((k % 1.0) * 4.0).floor();
    let k_base = k.floor() + (phase_index * 0.25);

    let prev_phase = (
        Phases::from_int((phase_index as u8 * 2) % 8).unwrap(),
        JulianTime::to_utc(julian_day_of_phase(k_base)),
    );

    let next_phase = (
        Phases::from_int(((phase_index as u8 + 1) * 2) % 8).unwrap(),
        JulianTime::to_utc(julian_day_of_phase(k_base + 0.25)),
    );

    (prev_phase, next_phase)
}

// ---------------- private ----------------

/// Computes the Selenocentric Elongation in degrees.
///
/// * `d` - The mean elongaion of the Moon
/// * `m` - The Sun's mean anomaly
/// * `m_prime` - The Moon's mean anomaly
fn selenocentric_elongation(d: f64, m: f64, m_prime: f64) -> f64 {
    let i = 180.0 - d - 6.289 * m_prime.to_radians().sin() + 2.1 * m.to_radians().sin()
        - 1.274 * (2.0 * d.to_radians() - m_prime.to_radians()).sin()
        - 0.658 * (2.0 * d.to_radians()).sin()
        - 0.214 * (2.0 * m_prime.to_radians()).sin()
        - 0.11 * d.to_radians().sin();
    utils::map_to_deg(i)
}

/// Computes the sun mean anomaly in degrees.
///
/// * `julian` - The Julian Time
fn sun_mean_anomaly(julian: &JulianTime) -> f64 {
    let t = julian.century;
    let coefs = [357.5291092, 35999.0502909, -0.0001536, 1.0 / 24490000.0];
    utils::map_to_deg(utils::polynomial_eval(t, &coefs))
}

/// Computes the moon mean anomaly in degrees.
///
/// * `julian` - The Julian Time
fn moon_mean_anomaly(julian: &JulianTime) -> f64 {
    let t = julian.century;
    let coefs = [
        134.9633964,
        477198.8675055,
        0.0087414,
        1.0 / 69699.0,
        -1.0 / 14712000.0,
    ];
    utils::map_to_deg(utils::polynomial_eval(t, &coefs))
}

/// Computes the julian ephemeris day.
///
/// * `k` - THe index of lunation
fn julian_day_of_phase(k: f64) -> f64 {
    let t = k / 1236.85;
    let coefs = [
        2451550.09766 + 29.530588861 * k,
        0.0,
        0.00015437,
        -0.000000150,
        0.00000000073,
    ];
    // // Julian Ephemeris Day (Dynamical Time)
    let jde_mean = utils::polynomial_eval(t, &coefs);

    let t = k / 1236.85;

    // Sun's Mean Anomaly at JDE
    let m = 2.5534 + 29.1053567 * k - 0.00000218 * t.powi(2);
    // Moon's Mean Anomaly at JDE
    let m_prime = 201.5643 + 385.81693528 * k + 0.0107438 * t.powi(2);

    // The coefs of the correction change with the phase
    let mut correction = 0.0;
    let is_full_or_new = (k % 1.0).abs() < 0.1 || (k % 1.0 - 0.5).abs() < 0.1;
    if is_full_or_new {
        correction += (0.1734 - 0.000393 * t) * m.to_radians().sin();
        correction += 0.0021 * (2.0 * m).to_radians().sin();
        correction -= 0.4068 * m_prime.to_radians().sin();
    } else {
        correction += (0.1721 - 0.0004 * t) * m.to_radians().sin();
        correction -= 0.6280 * m_prime.to_radians().sin();
    }

    let jde_true = jde_mean + correction;

    // Convert to Universal Time
    jde_true - DELTA_T / 86400.0
}

/// Estimates k, the index of lunation.
///
/// * `julian` - The Julian Time
fn index_lunation(julian: &JulianTime) -> f64 {
    let jd = julian.day;
    let ref_new_moon = 2451550.09766;
    let avg_synodic_month = 29.530588861;
    (jd - ref_new_moon) / avg_synodic_month
}
