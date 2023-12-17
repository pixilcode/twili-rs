use crate::model::Task;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskFormatter {
	Basic,
}

impl TaskFormatter {
	pub fn format(&self, task: &Task) -> String {
		match self {
			Self::Basic => format!(
				"{} {} | {}",
				if task.complete { "[x]" } else { "[ ]" },
				task.due_date,
				task.name,
			),
		}
	}
}