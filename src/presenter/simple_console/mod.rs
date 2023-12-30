use chrono::NaiveDate;

use crate::model::Task;
use crate::command::traits::Presenter as PresenterTrait;
use std::{io::{Stdin, Stdout, Write, BufRead, BufReader}, str::FromStr, fmt::Display};

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
	fn println(&mut self, output: impl AsRef<str>) {
		// TODO: handle errors better
		writeln!(self.stdout, "{}", output.as_ref()).expect("Failed to write to stdout");
	}

	fn print(&mut self, output: impl AsRef<str>) {
		// TODO: handle errors better
		write!(self.stdout, "{}", output.as_ref()).expect("Failed to write to stdout");
	}

	fn flush_stdout(&mut self) {
		// TODO: handle errors better
		self.stdout.flush().expect("Failed to flush stdout");
	}

	fn prompt_then<T, F>(&mut self, prompt: impl AsRef<str>, transform: F) -> T
	where
		F: FnOnce(String) -> T
	{
		let prompt = prompt.as_ref();
		let prompt = format!("{prompt}: ");
		let mut input = String::new();

		self.print(prompt);
		self.flush_stdout();

		self.stdin.read_line(&mut input).expect("Failed to read from stdin");

		transform(input)
	}

	fn prompt_opt<T>(&mut self, prompt: impl AsRef<str>) -> Option<T>
	where
		T: FromStr
	{
		self.prompt_then(prompt, |result| result.parse().ok())
	}

	fn prompt_or_default<T>(&mut self, prompt: impl AsRef<str>, default: T) -> T
	where
		T: Display + FromStr
	{
		let prompt = prompt.as_ref();
		let prompt = format!("{prompt} (leave empty for '{default}')");
		self.prompt_then(prompt, |result| {
			if result.is_empty() {
				default
			} else {
				result.parse().unwrap_or_else(|_| panic!("Failed to parse input!"))
			}
		})
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

		let selection = self.prompt_opt::<usize>("Enter your selection");

		// TODO: better error handling
		let selection = selection.expect("Failed to parse");

		&tasks[selection]
	}

	fn edit_task(&mut self, task: &Task) -> Task {
		let new_name = self.prompt_or_default::<String>("Enter the new name", task.name.clone());

		let curr_due_date = task.due_date;
		let due_date_prompt = format!("Enter the new due date as YYYY-MM-DD (leave empty for '{curr_due_date}')");
		let new_due_date = self.prompt_then(due_date_prompt, |input| {
			// TODO: figure out how to make the `prompt_or_default` more abstract
			if input.is_empty() { return curr_due_date }
			let trimmed_input = input.trim();
			NaiveDate::parse_from_str(trimmed_input, "%Y-%m-%d").expect("Failed to parse input")
		});

		let new_complete = self.prompt_or_default("Enter the new completion status", task.complete);
		
		let mut new_task = task.clone();
		new_task.name = new_name;
		new_task.due_date = new_due_date;
		new_task.complete = new_complete;

		new_task
	}

}
