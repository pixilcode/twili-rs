use chrono::{NaiveDate, NaiveTime};

pub type TaskId = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
	pub id: TaskId,
	pub name: String,
	pub due_date: NaiveDate,
	pub due_time: Option<NaiveTime>,
	pub complete: bool,
}