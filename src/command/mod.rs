use self::traits::{TaskDao, Presenter};

mod edit;
mod list;
pub mod traits;

pub use edit::run as edit;
pub use edit::EditConfig;
pub use list::run as list;
pub use list::ListConfig;

pub struct Config<P, M, T> {
	pub presenter: P,
	pub task_dao: M,
	pub extended: T,
}

impl<P, M, T> Config<P, M, T> {
	pub fn new(presenter: P, task_dao: M, extended: T) -> Self
	where
		P: Presenter + 'static,
		M: TaskDao + 'static,
	{
		Self {
			presenter,
			task_dao,
			extended,
		}
	}
}