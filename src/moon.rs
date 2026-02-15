use crate::astro;
use crate::julian_time::JulianTime;

use chrono::Utc;

#[derive(Debug)]
pub struct Lune {
    fraction: f64,
    current_phase: Phases,
    next_phase: Phases,
    prev_phase: Phases,
    delta_next: f64,
    delta_prev: f64,
}

impl Lune {
    pub fn new() -> Self {
        let now = Utc::now();
        let julian = JulianTime::new(now);

        let fraction = astro::illuminated_fraction(&julian);
        let phase = astro::get_phase(&julian);

        let around = astro::phases_around(&julian);
        let prev_phase = around.0.0;
        let prev_ts = around.0.1;
        let next_phase = around.1.0;
        let next_ts = around.1.1;

        Self {
            fraction,
            current_phase: Phases::from_int(phase).unwrap(),
            next_phase,
            prev_phase,
            delta_next: (next_ts.signed_duration_since(now).num_hours() as f64 / 24.0).round(),
            delta_prev: -(prev_ts.signed_duration_since(now).num_hours() as f64 / 24.0).round(),
        }
    }

    pub fn get_current_phase(&self) -> String {
        format!("{} ({:.1}%)", self.current_phase, self.fraction * 100.0)
    }

    pub fn get_phase_summary(&self) -> String {
        format!(
            "{} was {} days ago - {} is in {} days",
            self.prev_phase, self.delta_prev, self.next_phase, self.delta_next
        )
    }

    pub fn get_summary(&self) -> String {
        format!(
            "Phase: {}\nIllumination: {:.1}%\n{} in {} days\n{} was {} days ago",
            self.current_phase,
            self.fraction * 100.0,
            self.next_phase,
            self.delta_next,
            self.prev_phase,
            self.delta_prev
        )
    }
}

#[derive(Debug)]
pub enum Phases {
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
    pub fn from_int(n: u8) -> Result<Self, String> {
        match n {
            0 | 8 => Ok(Phases::NewMoon),
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

    // pub fn to_int(&self) -> u8 {
    //     match self {
    //         Phases::NewMoon => 0,
    //         Phases::WaxingCrescent => 1,
    //         Phases::FirstQuarter => 2,
    //         Phases::WaxingGibbous => 3,
    //         Phases::FullMoon => 4,
    //         Phases::WaningGibbous => 5,
    //         Phases::LastQuarter => 6,
    //         Phases::WaningCrescent => 7,
    //     }
    // }
}

impl std::fmt::Display for Phases {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Phases::NewMoon => "New Moon",
            Phases::WaxingCrescent => "Waxing Crescent",
            Phases::FirstQuarter => "First Quarter",
            Phases::WaxingGibbous => "Waxing Gibbous",
            Phases::FullMoon => "Full Moon",
            Phases::WaningGibbous => "Waning Gibbous",
            Phases::LastQuarter => "Last Quarter",
            Phases::WaningCrescent => "Waning Crescent",
        };
        write!(f, "{}", s)
    }
}
