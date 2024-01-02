use std::fs;
use std::path::{Path, PathBuf};

use diesel::prelude::*;

use crate::command::traits::TaskDao;
use crate::model::Task;
use self::models::Task as SqliteTask;

pub mod schema;
pub mod models;

pub struct SqliteTaskDao {
	connection: SqliteConnection,
}

impl SqliteTaskDao {
	pub fn new(location: impl AsRef<Path>, name: impl AsRef<str>) -> Self {
		let db_path = calc_db_path(&location, name);
		create_directory_if_not_exists(&location);
		create_db_if_not_exists(&db_path);

		// TODO: better error handling
		let db_path = db_path.to_str().expect("Invalid path");
		let connection = SqliteConnection::establish(db_path).unwrap();
		Self {
			connection,
		}
	}

}

impl TaskDao for SqliteTaskDao {
    fn get_all_tasks(&mut self) -> Vec<Task> {
		use self::schema::tasks::dsl::*;

		let results = tasks
			.select(SqliteTask::as_select())
			.load(&mut self.connection)
			// TODO: better error handling
			.expect("Error getting tasks");

		results.into_iter().map(|task| task.into()).collect()
    }

    fn update_task(&mut self, task: Task) {
		use self::schema::tasks::dsl::*;

		diesel::replace_into(tasks)
			.values(SqliteTask::from(task))
			.execute(&mut self.connection)
			// TODO: better error handling
			.expect("Error updating task");
    }
}

fn calc_db_path(location: impl AsRef<Path>, name: impl AsRef<str>) -> PathBuf {
	let db_file = format!("{}.db", name.as_ref());
	location.as_ref().join(db_file)
}

fn create_directory_if_not_exists(location: impl AsRef<Path>) {
	if !location.as_ref().exists() {
		// TODO: better error handling
		fs::create_dir_all(location).expect("Error creating data directory");
	}
}

fn create_db_if_not_exists(db_path: impl AsRef<Path>) {
	if !db_path.as_ref().exists() {
		// TODO: better error handling
		fs::File::create(db_path).expect("Error creating database file");
	}
}