use crate::model::{Task, TaskId};

pub trait TaskDao {
	fn get_all_tasks(&mut self) -> Vec<Task>;
	fn update_task(&mut self, task_id: &TaskId, task: Task);
}

pub trait Presenter {
	fn display_tasks(&mut self, tasks: &[Task]);
	fn select_task<'a>(&mut self, tasks: &'a [Task]) -> &'a Task;
	fn edit_task(&mut self, task_id: &Task) -> Task;
}