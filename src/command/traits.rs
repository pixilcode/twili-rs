use crate::model::Task;

pub trait TaskManager {
	fn get_all_tasks(&self) -> Vec<Task>;
	fn save_tasks(&mut self, tasks: Vec<Task>);
}

pub trait Presenter {
	fn display_tasks(&mut self, tasks: Vec<Task>);
	fn edit_tasks(&mut self, tasks: Vec<Task>) -> Vec<Task>;
}