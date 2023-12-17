use crate::command::traits::TaskManager;
use crate::model::Task;

pub struct InMemoryTaskManager {
	tasks: Vec<Task>,
}

impl InMemoryTaskManager {
	pub fn new() -> Self {
		Self { tasks: vec![] }
	}

	pub fn new_from_list(tasks: Vec<Task>) -> Self {
		Self { tasks }
	}
}

impl TaskManager for InMemoryTaskManager {
    fn get_all_tasks(&self) -> Vec<Task> {
		self.tasks.clone()
    }

    fn save_tasks(&mut self, tasks: Vec<Task>) {
		self.tasks = tasks;
    }
}