use crate::command::traits::TaskManager;
use crate::model::{Task, TaskId};

pub struct InMemoryTaskManager {
	tasks: Vec<Task>,
}

impl InMemoryTaskManager {
	pub fn new_from_list(tasks: Vec<Task>) -> Self {
		Self { tasks }
	}
}

impl TaskManager for InMemoryTaskManager {
    fn get_all_tasks(&mut self) -> Vec<Task> {
		self.tasks.clone()
    }

	fn update_task(&mut self, task_id: &TaskId, task: Task) {
		let task_index = self.tasks.iter().position(|t| &t.id == task_id).expect("Task not found");
		self.tasks[task_index] = task;
	}
}