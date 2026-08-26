/// Weather System: Generates realistic weather conditions for matches
///
/// Features:
/// - Seasonal weather patterns (October-May in England)
/// - Regional variations (Yorkshire/Lancashire more rain than London)
/// - Probability-based cancellations
/// - Pitch condition tracking

use rand::Rng;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Datelike};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherSystem {
    pub region: String, // "Yorkshire", "Lancashire", "London", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherForecast {
    pub condition: WeatherCondition,
    pub severity: i32, // 1-10
    pub precipitation_chance: f32,
    pub temperature_celsius: i32,
    pub wind_speed_mph: i32,
    pub precipitation_mm: i32,
    pub pitch_condition: PitchCondition,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeatherCondition {
    Clear,
    Overcast,
    LightRain,
    HeavyRain,
    Storm,
    LightSnow,
    HeavySnow,
    Fog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PitchCondition {
    Firm,
    Soft,
    Muddy,
    Waterlogged,
    Frozen,
    SnowCovered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Excellent,
    Good,
    Moderate,
    Poor,
}

impl WeatherSystem {
    pub fn new(region: String) -> Self {
        Self { region }
    }

    /// Generate weather for a specific date
    pub fn generate_weather_for_date(&self, date: NaiveDate) -> WeatherForecast {
        let mut rng = rand::thread_rng();

        // Monthly probabilities (October-May in England)
        let base_rain_chance = match date.month() {
            10 | 11 | 12 => 0.40, // Autumn/Winter
            1 | 2 => 0.45,         // Deep winter
            3 | 4 => 0.35,         // Spring
            _ => 0.25,              // Summer (if playing friendlies)
        };

        // Regional modifiers
        let rain_chance = match self.region.as_str() {
            "Yorkshire" | "Lancashire" => base_rain_chance + 0.10,
            "London" => base_rain_chance - 0.05,
            _ => base_rain_chance,
        };

        let precipitation: f32 = rng.gen();

        let condition = if precipitation < rain_chance {
            // Determine severity
            let severity_roll: f32 = rng.gen();
            if date.month() >= 12 || date.month() <= 2 {
                // Winter: chance of snow
                if severity_roll < 0.15 {
                    WeatherCondition::HeavySnow
                } else if severity_roll < 0.30 {
                    WeatherCondition::LightSnow
                } else if severity_roll < 0.60 {
                    WeatherCondition::HeavyRain
                } else {
                    WeatherCondition::LightRain
                }
            } else {
                // Other months: just rain
                if severity_roll < 0.25 {
                    WeatherCondition::HeavyRain
                } else {
                    WeatherCondition::LightRain
                }
            }
        } else {
            if rng.gen::<f32>() < 0.15 {
                WeatherCondition::Fog
            } else if rng.gen::<f32>() < 0.40 {
                WeatherCondition::Overcast
            } else {
                WeatherCondition::Clear
            }
        };

        let severity = self.calculate_severity(&condition);
        let temperature = self.calculate_temperature(date.month());
        let wind_speed = rng.gen_range(5..25);
        let precipitation_mm = self.calculate_precipitation(&condition);
        let pitch_condition = self.determine_pitch_condition(&condition, temperature);
        let visibility = self.determine_visibility(&condition);

        WeatherForecast {
            condition,
            severity,
            precipitation_chance: if precipitation < rain_chance {
                precipitation
            } else {
                0.0
            },
            temperature_celsius: temperature,
            wind_speed_mph: wind_speed,
            precipitation_mm,
            pitch_condition,
            visibility,
        }
    }

    fn calculate_severity(&self, condition: &WeatherCondition) -> i32 {
        match condition {
            WeatherCondition::Clear => 1,
            WeatherCondition::Overcast => 2,
            WeatherCondition::LightRain => 4,
            WeatherCondition::Fog => 5,
            WeatherCondition::HeavyRain => 7,
            WeatherCondition::LightSnow => 8,
            WeatherCondition::Storm => 9,
            WeatherCondition::HeavySnow => 10,
        }
    }

    fn calculate_temperature(&self, month: u32) -> i32 {
        let mut rng = rand::thread_rng();
        let base_temp = match month {
            12 | 1 | 2 => 3,   // Winter: 0-6°C
            3 | 11 => 7,        // Early spring/late autumn: 4-10°C
            4 | 10 => 11,       // Mid spring/autumn: 8-14°C
            _ => 15,            // Summer: 12-18°C
        };

        base_temp + rng.gen_range(-3..4)
    }

    fn calculate_precipitation(&self, condition: &WeatherCondition) -> i32 {
        let mut rng = rand::thread_rng();
        match condition {
            WeatherCondition::Clear | WeatherCondition::Overcast | WeatherCondition::Fog => 0,
            WeatherCondition::LightRain | WeatherCondition::LightSnow => rng.gen_range(1..5),
            WeatherCondition::HeavyRain | WeatherCondition::HeavySnow => rng.gen_range(5..15),
            WeatherCondition::Storm => rng.gen_range(10..25),
        }
    }

    fn determine_pitch_condition(
        &self,
        condition: &WeatherCondition,
        temperature: i32,
    ) -> PitchCondition {
        match condition {
            WeatherCondition::Clear | WeatherCondition::Overcast => {
                if temperature < 0 {
                    PitchCondition::Frozen
                } else {
                    PitchCondition::Firm
                }
            }
            WeatherCondition::Fog => PitchCondition::Soft,
            WeatherCondition::LightRain => PitchCondition::Soft,
            WeatherCondition::HeavyRain | WeatherCondition::Storm => {
                if temperature > 0 {
                    PitchCondition::Waterlogged
                } else {
                    PitchCondition::Frozen
                }
            }
            WeatherCondition::LightSnow | WeatherCondition::HeavySnow => {
                PitchCondition::SnowCovered
            }
        }
    }

    fn determine_visibility(&self, condition: &WeatherCondition) -> Visibility {
        match condition {
            WeatherCondition::Clear => Visibility::Excellent,
            WeatherCondition::Overcast | WeatherCondition::LightRain => Visibility::Good,
            WeatherCondition::HeavyRain | WeatherCondition::LightSnow => Visibility::Moderate,
            WeatherCondition::Storm | WeatherCondition::HeavySnow | WeatherCondition::Fog => {
                Visibility::Poor
            }
        }
    }

    /// Check if match should be cancelled (3 days before match)
    pub fn should_cancel_match(
        &self,
        forecast: &WeatherForecast,
        threshold: i32,
    ) -> (bool, f32) {
        let cancel = forecast.severity >= threshold;
        let probability = if forecast.severity >= threshold {
            ((forecast.severity - threshold) as f32 / 10.0).min(0.95)
        } else {
            0.0
        };

        (cancel, probability)
    }
}

impl WeatherForecast {
    pub fn condition_str(&self) -> String {
        match self.condition {
            WeatherCondition::Clear => "clear".to_string(),
            WeatherCondition::Overcast => "overcast".to_string(),
            WeatherCondition::LightRain => "light_rain".to_string(),
            WeatherCondition::HeavyRain => "heavy_rain".to_string(),
            WeatherCondition::Storm => "storm".to_string(),
            WeatherCondition::LightSnow => "light_snow".to_string(),
            WeatherCondition::HeavySnow => "heavy_snow".to_string(),
            WeatherCondition::Fog => "fog".to_string(),
        }
    }

    pub fn pitch_condition_str(&self) -> String {
        match self.pitch_condition {
            PitchCondition::Firm => "firm".to_string(),
            PitchCondition::Soft => "soft".to_string(),
            PitchCondition::Muddy => "muddy".to_string(),
            PitchCondition::Waterlogged => "waterlogged".to_string(),
            PitchCondition::Frozen => "frozen".to_string(),
            PitchCondition::SnowCovered => "snow_covered".to_string(),
        }
    }

    pub fn visibility_str(&self) -> String {
        match self.visibility {
            Visibility::Excellent => "excellent".to_string(),
            Visibility::Good => "good".to_string(),
            Visibility::Moderate => "moderate".to_string(),
            Visibility::Poor => "poor".to_string(),
        }
    }

    /// Get user-friendly description
    pub fn description(&self) -> String {
        let condition_desc = match self.condition {
            WeatherCondition::Clear => "clear skies",
            WeatherCondition::Overcast => "overcast skies",
            WeatherCondition::LightRain => "light rain",
            WeatherCondition::HeavyRain => "heavy rain",
            WeatherCondition::Storm => "stormy conditions",
            WeatherCondition::LightSnow => "light snow",
            WeatherCondition::HeavySnow => "heavy snow",
            WeatherCondition::Fog => "thick fog",
        };

        format!(
            "{} with {} pitch, {}°C, {}mph winds",
            condition_desc,
            self.pitch_condition_str(),
            self.temperature_celsius,
            self.wind_speed_mph
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Datelike};

    #[test]
    fn test_weather_generation() {
        let weather_system = WeatherSystem::new("Yorkshire".to_string());
        let date = NaiveDate::from_ymd_opt(1867, 12, 25).unwrap();

        let forecast = weather_system.generate_weather_for_date(date);

        assert!(forecast.severity >= 1 && forecast.severity <= 10);
        assert!(forecast.temperature_celsius >= -5 && forecast.temperature_celsius <= 20);
    }

    #[test]
    fn test_cancellation_logic() {
        let weather_system = WeatherSystem::new("Yorkshire".to_string());
        let forecast = WeatherForecast {
            condition: WeatherCondition::HeavySnow,
            severity: 10,
            precipitation_chance: 0.9,
            temperature_celsius: -2,
            wind_speed_mph: 20,
            precipitation_mm: 15,
            pitch_condition: PitchCondition::SnowCovered,
            visibility: Visibility::Poor,
        };

        let (should_cancel, probability) = weather_system.should_cancel_match(&forecast, 7);

        assert!(should_cancel);
        assert!(probability > 0.0);
    }
}
