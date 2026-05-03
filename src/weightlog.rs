use chrono::{Local, NaiveDate};
use std::fmt;

#[derive(Clone)]
pub struct WeightLog {
    date: NaiveDate,
    weight: f32,
    note: Option<String>,
}

impl WeightLog {
    pub fn new(date: NaiveDate, weight: f32, note: Option<String>) -> Self {
        WeightLog { date, weight, note }
    }

    pub fn to_data_str(&self) -> String {
        format!(
            "{},{},{}",
            self.date.format("%Y-%m-%d"),
            self.weight,
            self.get_note().unwrap_or_else(|| String::new())
        )
    }

    pub fn get_date(&self) -> NaiveDate {
        self.date
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn get_note(&self) -> Option<String> {
        self.note.clone()
    }
}

impl Default for WeightLog {
    fn default() -> Self {
        WeightLog {
            date: Local::now().date_naive(),
            weight: 0.0,
            note: None,
        }
    }
}

impl fmt::Debug for WeightLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeightLog")
            .field("Date", &self.date.format("%Y-%m-%d"))
            .field("Weight", &self.weight)
            .field("Note", &self.note.clone().unwrap_or_default())
            .finish()
    }
}
