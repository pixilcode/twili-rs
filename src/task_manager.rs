use crate::command::traits::TaskManager;
use crate::model::{Task, TaskGroup};

pub struct InMemoryTaskManager {
	task_groups: Vec<TaskGroup>,
}

impl InMemoryTaskManager {
	pub fn new() -> Self {
		Self { task_groups: vec![] }
	}
}

impl TaskManager for InMemoryTaskManager {
    fn get_all_tasks(&self) -> Vec<Task> {
        self.task_groups.iter()
			.flat_map(|group| group.tasks.clone())
			.collect()
    }

    fn get_grouped_tasks(&self) -> Vec<TaskGroup> {
		self.task_groups.clone()
    }

    fn save_tasks(&mut self, tasks: Vec<TaskGroup>) {
		self.task_groups = tasks;
    }
}