use chrono::{Local, NaiveDate};

pub struct WeightLog {
    date: NaiveDate,
    weight: f32,
    note: Option<String>
}

impl WeightLog {
    pub fn new(date: NaiveDate, weight: f32, note: Option<String>) -> Self {
        WeightLog { date, weight, note }
    }

    pub fn to_str(&self) -> String {
        let result = format!(
            "Date: {}, Weight: {}",
            self.date.format("%Y-%m-%d"),
            self.weight
        );

        result
    }

    pub fn to_data_str(&self) -> String {
        format!("{},{},{}", self.date.format("%Y-%m-%d"), self.weight, self.get_note().unwrap_or_default())
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
            note: None
        }
    }
}
