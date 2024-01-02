use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
	pub id: String,
	pub name: String,
	pub due_date: NaiveDate,
	pub due_time: Option<NaiveTime>,
	pub complete: bool,
}

impl From<Task> for crate::model::Task {
	fn from(task: Task) -> Self {
		Self {
			id: task.id,
			name: task.name,
			due_date: task.due_date,
			due_time: task.due_time,
			complete: task.complete,
		}
	}
}

impl From<crate::model::Task> for Task {
	fn from(task: crate::model::Task) -> Self {
		Self {
			id: task.id,
			name: task.name,
			due_date: task.due_date,
			due_time: task.due_time,
			complete: task.complete,
		}
	}
}