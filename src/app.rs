use chrono::{Datelike, Local, NaiveDate};
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fs::DirEntry;
use std::fs::read_dir;
use std::fs::{self};
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use std::{env, io};
use time::Date;

use ratatui::widgets::TableState;

use crate::weightlog::WeightLog;

#[derive(PartialEq, Eq)]
pub enum CurrentDisplay {
    Table(TableDisplay),
    Graph(GraphDisplay),
    Add(AddState),
    Delete,
}

#[derive(PartialEq, Eq)]
pub enum TableDisplay {
    Total,
    Week,
}

#[derive(PartialEq, Eq)]
pub enum GraphDisplay {
    Total,
    Week,
    AverageWeekly,
}

#[derive(PartialEq, Eq)]
pub enum AddState {
    Calendar,
    WeightInput(Input),
    NoteInput,
}

#[derive(PartialEq, Eq)]
pub enum Input {
    Valid,
    Invalid(String),
}

pub struct AppState {
    file: DirEntry,
    data: Vec<WeightLog>,
    pub display: CurrentDisplay,
    pub table_state: TableState,
    pub current_date: Date,
    pub current_weight: f32,
    pub char_buf: String,
    pub character_index: usize,
    pub max_weight: f32,
    pub min_weight: f32,
    pub min_date: Date,
    pub max_date: Date,
    pub edited: bool,
}

impl AppState {
    pub fn init() -> Self {
        let file = if let Some(dir_entry) = check_file() {
            dir_entry
        } else {
            let _ = create_file();
            check_file().expect("should have created file")
        };

        let today = Local::now().date_naive();

        if let Ok(mut data) = get_data(&file) {
            data.sort_by_key(|a| a.get_date());

            let recent_weight = data.last().map_or(0.0, |x| x.get_weight());
            let mut max_weight = f32::MIN;
            let mut min_weight = f32::MAX;

            for value in &data {
                max_weight = max_weight.max(value.get_weight());
                min_weight = min_weight.min(value.get_weight());
            }

            let current_date = Date::from_ordinal_date(
                today.year(),
                u16::try_from(today.ordinal()).expect("should not go past"),
            )
            .expect("date should not cause error");

            let min_date = data
                .first()
                .map(|log| log.get_date())
                .and_then(convert_naive_date_to_date)
                .unwrap_or(current_date);

            let max_date = data
                .last()
                .map(|log| log.get_date())
                .and_then(convert_naive_date_to_date)
                .unwrap_or(current_date);

            return AppState {
                file,
                data,
                display: CurrentDisplay::Table(TableDisplay::Total),
                table_state: TableState::default(),
                current_date,
                current_weight: recent_weight,
                char_buf: recent_weight.to_string(),
                character_index: recent_weight.to_string().len(),
                edited: false,
                max_weight,
                min_weight,
                min_date,
                max_date,
            };
        }

        panic!("Error reading the data file. Please check to make sure it is in the correct format")
    }

    pub fn delete_log(&mut self, index: usize) {
        let deleted_log = self.data.remove(index);
        self.edited = true;
        let handle = self.save();
        if convert_naive_date_to_date(deleted_log.get_date())
            .expect("should not have issues converting dates")
            == self.min_date
        {
            self.min_date = self
                .data
                .iter()
                .map(|x| x.get_date())
                .min()
                .and_then(convert_naive_date_to_date)
                .unwrap_or_else(|| {
                    let today = Local::now();
                    Date::from_ordinal_date(
                        today.year(),
                        u16::try_from(today.ordinal()).expect("should not go passed"),
                    )
                    .expect("shoudl not have issues converting date")
                });
        }

        if convert_naive_date_to_date(deleted_log.get_date())
            .expect("should not have issues converting dates")
            == self.max_date
        {
            self.max_date = self
                .data
                .iter()
                .map(|x| x.get_date())
                .max()
                .and_then(convert_naive_date_to_date)
                .unwrap_or_else(|| {
                    let today = Local::now();
                    Date::from_ordinal_date(
                        today.year(),
                        u16::try_from(today.ordinal()).expect("should not go past"),
                    )
                    .expect("shoudl not have issues converting date")
                });
        }

        if deleted_log.get_weight() == self.max_weight {
            self.max_weight = self
                .data
                .iter()
                .map(|x| x.get_weight())
                .max_by(|a, b| a.total_cmp(b))
                .unwrap_or(0.0);
        }
        if deleted_log.get_weight() == self.min_weight {
            self.min_weight = self
                .data
                .iter()
                .map(|x| x.get_weight())
                .min_by(|a, b| a.total_cmp(b))
                .unwrap_or(0.0);
        }
        handle.join().ok();
    }

    pub fn get_display(&self) -> &CurrentDisplay {
        &self.display
    }

    pub fn set_display(&mut self, display: CurrentDisplay) {
        self.display = display;
    }

    pub fn go_to_prev_date(&mut self) {
        let index = self.data.partition_point(|x| {
            convert_naive_date_to_date(x.get_date()).expect("should not have problems converting")
                < self.current_date
        });

        self.current_date =
            convert_naive_date_to_date(self.data[index.checked_sub(1).unwrap_or(0)].get_date())
                .expect("shout not have problems converting");
    }

    pub fn go_to_next_date(&mut self) {
        let index = self.data.partition_point(|x| {
            convert_naive_date_to_date(x.get_date()).expect("should not have problems converting")
                <= self.current_date
        });

        self.current_date =
            convert_naive_date_to_date(self.data[index.min(self.data.len() - 1)].get_date())
                .expect("shout not have problems converting");
    }

    pub fn get_average_daily_data(&self) -> BTreeMap<NaiveDate, f32> {
        let mut btree_map = BTreeMap::new();
        let mut count = HashMap::new();

        for line in &self.data {
            let date = line.get_date();
            let weight = line.get_weight();

            let n = *count.entry(date).and_modify(|x| *x += 1).or_insert(1);
            btree_map
                .entry(date)
                .and_modify(|x| {
                    *x += (weight - *x) / n as f32;
                })
                .or_insert(weight);
        }

        btree_map
    }

    pub fn get_average_weekly_data(&self) -> BTreeMap<NaiveDate, f32> {
        let mut btree_map = BTreeMap::new();
        let mut count = HashMap::new();

        for line in &self.data {
            let date = line.get_date().week(chrono::Weekday::Mon).last_day();
            let weight = line.get_weight();

            let n = *count.entry(date).and_modify(|x| *x += 1).or_insert(1);
            btree_map
                .entry(date)
                .and_modify(|x| {
                    *x += (weight - *x) / n as f32;
                })
                .or_insert(weight);
        }

        btree_map
    }

    pub fn save(&mut self) -> JoinHandle<io::Result<()>> {
        let data = self.data.clone();
        let path = self.file.path();

        let handle = thread::spawn(move || -> io::Result<()> {
            let mut temp_file = PathBuf::from(path.parent().expect("shoudld not be in root"));
            temp_file.push("temp_cut.txt");

            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&temp_file)?;
            for log in &data {
                writeln!(file, "{}", log.to_data_str())?;
            }
            fs::rename(temp_file, path)?;
            Ok(())
        });

        self.edited = false;

        handle
    }

    pub fn get_data(&self) -> &Vec<WeightLog> {
        &self.data
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.char_buf.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    pub fn byte_index(&self) -> usize {
        self.char_buf
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.character_index)
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.char_buf.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.char_buf.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.char_buf = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.char_buf.chars().count())
    }

    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit(&mut self) {
        match self.input_data() {
            Ok(weightlog) => {
                let date = convert_naive_date_to_date(weightlog.get_date())
                    .expect("should not have problems converting");
                self.max_weight = self.max_weight.max(weightlog.get_weight());
                self.min_weight = self.min_weight.min(weightlog.get_weight());
                self.max_date = self.max_date.max(date);
                self.min_date = self.min_date.min(date);
                let index = self
                    .data
                    .partition_point(|x| x.get_date() < weightlog.get_date());
                self.data.insert(index, weightlog);
                self.edited = true;
            }
            Err(e) => {
                self.set_display(CurrentDisplay::Add(AddState::WeightInput(Input::Invalid(
                    e.to_string(),
                ))));
            }
        }
        self.char_buf.clear();
    }

    fn input_data(&mut self) -> Result<WeightLog, Box<dyn Error>> {
        let new_log = WeightLog::new(
            NaiveDate::from_yo_opt(
                self.current_date.year(),
                u32::from(self.current_date.ordinal()),
            )
            .expect("should not have issues with dates"),
            self.current_weight,
            if self.char_buf.is_empty() {
                None
            } else {
                Some(self.char_buf.clone())
            },
        );

        Ok(new_log)
    }
}

fn get_data(path: &DirEntry) -> Result<Vec<WeightLog>, Box<dyn Error>> {
    let file = File::open(path.path())?;

    let mut data = vec![];

    let reader = BufReader::new(file).lines();

    for line in reader {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut fields = line.split(',');

        let date = fields
            .next()
            .and_then(|x| NaiveDate::parse_from_str(x.trim(), "%Y-%m-%d").ok())
            .ok_or(FileReadError::new("error reading date"))?;

        let weight = fields
            .next()
            .and_then(|x| x.parse::<f32>().ok())
            .ok_or(FileReadError::new("error reading weight"))?;

        let note = fields.next().and_then(|x| Some(x.to_string()));

        data.push(WeightLog::new(date, weight, note));
    }

    Ok(data)
}

fn check_file() -> Option<DirEntry> {
    const FILE_NAME: &str = "cut.txt";
    // first check paths env variables
    let path_variable = env::var("PATH");

    if let Ok(path) = path_variable {
        for p in path.split(':') {
            if let Some(entry) = search_for_file(p, FILE_NAME) {
                return Some(entry);
            }
        }
    }
    // then check data directory

    if let Some(mut data_dir) = dirs::data_dir() {
        data_dir.push("cut");
        let data_dir_str = data_dir.to_str()?;

        println!("{data_dir_str}");

        return search_for_file(data_dir_str, FILE_NAME);
    }

    None
}

#[derive(Debug)]
struct PathDoesNotExist;

impl std::fmt::Display for PathDoesNotExist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "directory does not exist")
    }
}

impl std::error::Error for PathDoesNotExist {}

#[derive(Debug)]
struct FileReadError<'a> {
    message: &'a str,
}

impl<'a> FileReadError<'a> {
    fn new(message: &'a str) -> Self {
        FileReadError { message }
    }
}

impl std::fmt::Display for FileReadError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while reading cut.txt data file: {}", self.message)
    }
}

impl std::error::Error for FileReadError<'_> {}

fn create_file() -> Result<(), Box<dyn Error>> {
    match dirs::data_dir() {
        Some(mut path) => {
            path.push("cut");
            fs::create_dir(&path)?;
            path.push("cut");
            path.set_extension("txt");
            fs::File::create_new(path)?;
            Ok(())
        }
        None => Err(Box::new(PathDoesNotExist)),
    }
}

fn search_for_file(path: &str, data_file: &str) -> Option<DirEntry> {
    if let Ok(files) = read_dir(path) {
        for file in files {
            if let Ok(filename) = file
                && filename.file_name() == data_file
            {
                return Some(filename);
            }
        }
    }
    None
}

fn convert_naive_date_to_date(naive_date: NaiveDate) -> Option<Date> {
    let year = naive_date.year();
    let ordinal = naive_date.ordinal();

    Date::from_ordinal_date(year, u16::try_from(ordinal).expect("should not go passed")).ok()
}
