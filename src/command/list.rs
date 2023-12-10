use super::Config;

pub struct ListConfig;

pub fn run(mut config: Config<ListConfig>) {
	let tasks = config.task_manager.get_all_tasks();

	// filter out completed tasks and sort by due date
	let mut tasks: Vec<_> = tasks
		.into_iter()
		.filter(|task| !task.complete)
		.collect();
	tasks.sort_by_key(|task| task.due_date);

	config.presenter.display_tasks(tasks);
}