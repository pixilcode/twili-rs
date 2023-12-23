use chrono::{NaiveDate, format::format};

use crate::model::Task;
use crate::command::traits::Presenter as PresenterTrait;
use std::io::{Stdin, Stdout, Write, BufRead, BufReader};

pub use self::task_formatter::TaskFormatter;

mod task_formatter;

pub struct Presenter<I, O>
where
	I: BufRead,
	O: Write,
{
	stdin: I,
	stdout: O,
	task_formatter: TaskFormatter,
}

impl Presenter<BufReader<Stdin>, Stdout> {
	pub fn new(formatter: TaskFormatter) -> Presenter<BufReader<Stdin>, Stdout> {
		let stdin = BufReader::new(std::io::stdin());
		let stdout = std::io::stdout();

		Self {
			stdin,
			stdout,
			task_formatter: formatter,
		}
	}
}

impl<I, O> Presenter<I, O>
where
	I: BufRead,
	O: Write,
{
	fn println(&mut self, s: impl AsRef<str>) {
		// TODO: handle errors better
		writeln!(self.stdout, "{}", s.as_ref()).expect("Failed to write to stdout");
	}

	fn print(&mut self, s: impl AsRef<str>) {
		// TODO: handle errors better
		write!(self.stdout, "{}", s.as_ref()).expect("Failed to write to stdout");
	}

	fn flush_stdout(&mut self) {
		// TODO: handle errors better
		self.stdout.flush().expect("Failed to flush stdout");
	}
}

impl<I, O> PresenterTrait for Presenter<I, O>
where
	I: BufRead,
	O: Write,
{
	fn display_tasks(&mut self, tasks: &[Task]) {
		for task in tasks {
			let task_string = self.task_formatter.format(&task);
			self.println(task_string);
		}

		self.flush_stdout();
	}

	fn select_task<'a>(&mut self, tasks: &'a [Task]) -> &'a Task {
		// Select a task
		for (index, task) in tasks.iter().enumerate() {
			let task_string = self.task_formatter.format(&task);
			self.println(format!("{}: {}", index, task_string));
		}

		// TODO: factor out the prompt
		let mut input = String::new();

		self.print("Enter your selection: ");
		self.flush_stdout();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let selection = input.trim().parse::<usize>().expect("Failed to parse input");

		&tasks[selection]
	}

	fn edit_task(&mut self, task: &Task) -> Task {
		// Edit the task
		// TODO: factor out the prompt
		let mut input = String::new();

		self.print("Enter the new name: ");
		self.stdout.flush().unwrap();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let new_name = input.trim().to_string();

		// TODO: factor out the prompt
		let mut input = String::new();

		self.print("Enter the new due date: ");
		self.flush_stdout();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let new_due_date = input.trim();
		let new_due_date = NaiveDate::parse_from_str(new_due_date, "%Y-%m-%d").expect("Failed to parse input");

		// TODO: factor out the prompt
		let mut input = String::new();

		self.print("Enter the new completion status: ");
		self.stdout.flush().unwrap();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let new_complete = input.trim().parse::<bool>().expect("Failed to parse input");

		let mut new_task = task.clone();
		new_task.name = new_name;
		new_task.due_date = new_due_date;
		new_task.complete = new_complete;

		new_task
	}

}
