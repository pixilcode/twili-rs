use chrono::{NaiveDate, NaiveTime};

pub struct Task {
	pub name: String,
	pub due_date: NaiveDate,
	pub due_time: Option<NaiveTime>,
	pub complete: bool,
}