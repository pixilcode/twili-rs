use super::{
	Config,
	traits::{
		Presenter,
		TaskDao
	}
};

pub struct ListConfig;

pub fn run<P, M>(config: &mut Config<P, M, ListConfig>) 
where
	P: Presenter,
	M: TaskDao,
{
	let tasks = config.task_dao.get_all_tasks();

	// filter out completed tasks and sort by due date
	// TODO: allow config to specify sorting and filtering
	let mut tasks: Vec<_> = tasks
		.into_iter()
		.filter(|task| !task.complete)
		.collect();
	tasks.sort_by_key(|task| task.due_date);

	config.presenter.display_tasks(&tasks);
}