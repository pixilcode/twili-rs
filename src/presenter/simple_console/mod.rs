use crate::model::{Task, TaskGroup};
use crate::command::traits::Presenter as PresenterTrait;
use std::io::{Stdout, Write};

pub use self::task_formatter::TaskFormatter;

mod task_formatter;

pub struct Presenter {
	stdout: Stdout,
	task_formatter: TaskFormatter,
}

impl Presenter {
	pub fn new(formatter: TaskFormatter) -> Self {
		Self {
			stdout: std::io::stdout(),
			task_formatter: formatter,
		}
	}
}

impl PresenterTrait for Presenter {
	fn display_tasks(&mut self, tasks: Vec<Task>) {
		for task in tasks {
			let task_string = self.task_formatter.format(&task);
			writeln!(self.stdout, "{}", task_string).expect("Failed to write to stdout");
		}

		self.stdout.flush().unwrap();
	}

	fn edit_tasks(&mut self, tasks: Vec<TaskGroup>) -> Vec<TaskGroup> {
		writeln!(self.stdout, "Editing tasks").expect("Failed to write to stdout");
		tasks
	}
}