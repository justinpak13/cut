use chrono::{Local, NaiveDate};

pub struct WeightLog {
    date: NaiveDate,
    weight: f32,
}

impl WeightLog {
    pub fn new(date: NaiveDate, weight: f32) -> Self {
        WeightLog { date, weight }
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
        format!("{},{}", self.date.format("%Y-%m-%d"), self.weight)
    }

    pub fn get_date(&self) -> NaiveDate {
        self.date
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }
}

impl Default for WeightLog {
    fn default() -> Self {
        WeightLog {
            date: Local::now().date_naive(),
            weight: 0.0,
        }
    }
}
