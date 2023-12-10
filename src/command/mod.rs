use self::traits::{TaskManager, Presenter};

mod edit;
mod list;
pub mod traits;

pub use edit::run as edit;
pub use edit::EditConfig;
pub use list::run as list;
pub use list::ListConfig;

pub struct Config<T> {
	pub presenter: Box<dyn Presenter>,
	pub task_manager: Box<dyn TaskManager>,
	pub extended: T,
}

impl<T> Config<T> {
	pub fn new<P, M, E>(presenter: P, task_manager: M, extended: T) -> Self
	where
		P: Presenter + 'static,
		M: TaskManager + 'static,
	{
		Self {
			presenter: Box::new(presenter),
			task_manager: Box::new(task_manager),
			extended,
		}
	}
}