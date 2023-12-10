use crate::model::{Task, TaskGroup};
use crate::command::traits::Presenter;
use std::io::{Stdout, Write};

pub struct ConsolePresenter {
	stdout: Stdout,
}

impl ConsolePresenter {
	pub fn new() -> Self {
		Self {
			stdout: std::io::stdout(),
		}
	}
}

impl Presenter for ConsolePresenter {
	fn display_tasks(&mut self, tasks: Vec<Task>) {
		for task in tasks {
			writeln!(self.stdout, "{} {}", task.due_date, task.due_date.format("%a %d-%m-%Y")).expect("Failed to write to stdout");
		}

		self.stdout.flush().unwrap();
	}

	fn edit_tasks(&mut self, tasks: Vec<TaskGroup>) -> Vec<TaskGroup> {
		writeln!(self.stdout, "Editing tasks").expect("Failed to write to stdout");
		tasks
	}
}