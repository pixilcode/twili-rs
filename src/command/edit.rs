use super::{
	Config,
	traits::{
		Presenter,
		TaskDao
	}
};

pub struct EditConfig;

pub fn run<P, M>(config: &mut Config<P, M, EditConfig>)
where
	P: Presenter,
	M: TaskDao,
{
	let tasks = config.task_dao.get_all_tasks();
	let task = config.presenter.select_task(&tasks);
	let new_task = config.presenter.edit_task(task);
	config.task_dao.update_task(new_task);
}