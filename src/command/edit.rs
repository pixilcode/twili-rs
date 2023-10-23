use super::Config;

pub struct EditConfig;

fn run(config: Config<EditConfig>) {
	let tasks = config.task_manager.get_grouped_tasks();
	let tasks = config.presenter.edit_tasks(tasks);
	config.task_manager.save_tasks(tasks);
}