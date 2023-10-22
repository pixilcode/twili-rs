use chrono::{NaiveDate, NaiveTime};

struct Task {
	name: String,
	due_date: NaiveDate,
	due_time: Option<NaiveTime>,
	complete: bool,
}