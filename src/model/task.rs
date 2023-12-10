use chrono::{NaiveDate, NaiveTime};

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
	pub id: String,
	pub name: String,
	pub due_date: NaiveDate,
	pub due_time: Option<NaiveTime>,
	pub complete: bool,
}