use crate::model::{Task, TaskGroup};

pub trait TaskManager {
	fn get_all_tasks(&self) -> Vec<Task>;
	fn get_grouped_tasks(&self) -> Vec<TaskGroup>;
	fn save_tasks(&self, tasks: Vec<TaskGroup>);
}

pub trait Presenter {
	fn display_tasks(&mut self, tasks: Vec<Task>);
	fn edit_tasks(&mut self, tasks: Vec<TaskGroup>) -> Vec<TaskGroup>;
}