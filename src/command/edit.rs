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
	let task = config.presenter.select_task(&tasks);
	let new_task = config.presenter.edit_task(task);
	config.task_manager.update_task(&task.id, new_task);
}