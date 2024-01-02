use crate::command::traits::TaskDao;
use crate::model::Task;

pub struct InMemoryTaskDao {
	tasks: Vec<Task>,
}

impl InMemoryTaskDao {
	pub fn new_from_list(tasks: Vec<Task>) -> Self {
		Self { tasks }
	}
}

impl TaskDao for InMemoryTaskDao {
    fn get_all_tasks(&mut self) -> Vec<Task> {
		self.tasks.clone()
    }

	fn update_task(&mut self, task: Task) {
		let task_id = &task.id;
		let task_index = self.tasks.iter().position(|t| &t.id == task_id).expect("Task not found");
		self.tasks[task_index] = task;
	}
}