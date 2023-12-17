use chrono::NaiveDate;

use crate::model::Task;
use crate::command::traits::Presenter as PresenterTrait;
use std::io::{Stdout, Stdin, Write};

pub use self::task_formatter::TaskFormatter;

mod task_formatter;

pub struct Presenter {
	stdout: Stdout,
	stdin: Stdin,
	task_formatter: TaskFormatter,
}

impl Presenter {
	pub fn new(formatter: TaskFormatter) -> Self {
		Self {
			stdout: std::io::stdout(),
			stdin: std::io::stdin(),
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

	fn edit_tasks(&mut self, tasks: Vec<Task>) -> Vec<Task> {
		// Select a task
		for (index, task) in tasks.iter().enumerate() {
			let task_string = self.task_formatter.format(&task);
			writeln!(self.stdout, "{}: {}", index, task_string).expect("Failed to write to stdout");
		}

		let mut input = String::new();

		write!(self.stdout, "Enter your selection: ").expect("Failed to write to stdout");
		self.stdout.flush().unwrap();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let selection = input.trim().parse::<usize>().expect("Failed to parse input");

		// Edit the task
		let mut input = String::new();

		write!(self.stdout, "Enter the new name: ").expect("Failed to write to stdout");
		self.stdout.flush().unwrap();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let new_name = input.trim().to_string();

		let mut input = String::new();

		write!(self.stdout, "Enter the new due date: ").expect("Failed to write to stdout");
		self.stdout.flush().unwrap();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let new_due_date = input.trim();
		let new_due_date = NaiveDate::parse_from_str(new_due_date, "%Y-%m-%d").expect("Failed to parse input");

		let mut input = String::new();

		write!(self.stdout, "Enter the new completion status: ").expect("Failed to write to stdout");
		self.stdout.flush().unwrap();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let new_complete = input.trim().parse::<bool>().expect("Failed to parse input");

		// Update the task
		let mut new_task = tasks[selection].clone();

		new_task.name = new_name;
		new_task.due_date = new_due_date;
		new_task.complete = new_complete;

		println!("{:?}", new_task);

		let mut new_tasks = tasks.clone();

		new_tasks[selection] = new_task;

		tasks
	}
}