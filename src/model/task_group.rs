use crate::model::task::Task;

pub struct TaskGroup {
	pub name: String,
	pub tasks: Vec<Task>,
}