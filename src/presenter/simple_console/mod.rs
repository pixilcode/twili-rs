use chrono::NaiveDate;

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

impl<I, O> PresenterTrait for Presenter<I, O>
where
	I: BufRead,
	O: Write,
{
	fn display_tasks(&mut self, tasks: &[Task]) {
		for task in tasks {
			let task_string = self.task_formatter.format(&task);
			// TODO: factor out printing
			writeln!(self.stdout, "{}", task_string).expect("Failed to write to stdout");
		}

		self.stdout.flush().unwrap();
	}

	fn select_task<'a>(&mut self, tasks: &'a [Task]) -> &'a Task {
		// Select a task
		for (index, task) in tasks.iter().enumerate() {
			let task_string = self.task_formatter.format(&task);
			// TODO: factor out printing
			writeln!(self.stdout, "{}: {}", index, task_string).expect("Failed to write to stdout");
		}

		// TODO: factor out the prompt
		let mut input = String::new();

		// TODO: factor out printing
		write!(self.stdout, "Enter your selection: ").expect("Failed to write to stdout");
		self.stdout.flush().unwrap();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let selection = input.trim().parse::<usize>().expect("Failed to parse input");

		&tasks[selection]
	}

	fn edit_task(&mut self, task: &Task) -> Task {
		// Edit the task
		// TODO: factor out the prompt
		let mut input = String::new();

		// TODO: factor out printing
		write!(self.stdout, "Enter the new name: ").expect("Failed to write to stdout");
		self.stdout.flush().unwrap();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let new_name = input.trim().to_string();

		// TODO: factor out the prompt
		let mut input = String::new();

		// TODO: factor out printing
		write!(self.stdout, "Enter the new due date: ").expect("Failed to write to stdout");
		self.stdout.flush().unwrap();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let new_due_date = input.trim();
		let new_due_date = NaiveDate::parse_from_str(new_due_date, "%Y-%m-%d").expect("Failed to parse input");

		// TODO: factor out the prompt
		let mut input = String::new();

		// TODO: factor out printing
		write!(self.stdout, "Enter the new completion status: ").expect("Failed to write to stdout");
		self.stdout.flush().unwrap();
		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		let new_complete = input.trim().parse::<bool>().expect("Failed to parse input");

		let mut new_task = task.clone();
		new_task.name = new_name;
		new_task.due_date = new_due_date;
		new_task.complete = new_complete;

		println!("{:?}", new_task);
		
		new_task
	}
}
