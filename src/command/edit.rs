use super::{
	Config,
	traits::{
		Presenter,
		TaskManager
	}
};

pub struct EditConfig;

pub fn run<P, M>(mut config: Config<P, M, EditConfig>)
where
	P: Presenter,
	M: TaskManager,
{
	let tasks = config.task_manager.get_all_tasks();
	let tasks = config.presenter.edit_tasks(tasks);
	config.task_manager.save_tasks(tasks);
}