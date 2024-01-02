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
	pub fn new_from_path(path: &str) -> Self {
		let connection = SqliteConnection::establish(path).unwrap();
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
			.expect("Error getting tasks");

		results.into_iter().map(|task| task.into()).collect()
    }

    fn update_task(&mut self, task: Task) {
		use self::schema::tasks::dsl::*;

		diesel::replace_into(tasks)
			.values(SqliteTask::from(task))
			.execute(&mut self.connection)
			.expect("Error updating task");
    }
}