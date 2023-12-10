use crate::model::task::Task;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskGroup {
	pub name: String,
	pub tasks: Vec<Task>,
}