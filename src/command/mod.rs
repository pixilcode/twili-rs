use self::traits::{TaskManager, Presenter};

mod edit;
mod list;
pub mod traits;

pub use edit::run as edit;
pub use edit::EditConfig;
pub use list::run as list;
pub use list::ListConfig;

pub struct Config<P, M, T> {
	pub presenter: P,
	pub task_manager: M,
	pub extended: T,
}

impl<P, M, T> Config<P, M, T> {
	pub fn new(presenter: P, task_manager: M, extended: T) -> Self
	where
		P: Presenter + 'static,
		M: TaskManager + 'static,
	{
		Self {
			presenter: presenter,
			task_manager: task_manager,
			extended,
		}
	}
}