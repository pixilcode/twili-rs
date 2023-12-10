use clap::Parser;

mod command;
mod model;
mod presenter;
mod task_manager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    List,
    Edit,
}

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::List => {
            let config = command::Config::new(
                presenter::ConsolePresenter::new(),
                task_manager::InMemoryTaskManager::new(),
                command::ListConfig,
            );
            command::list(config);
        }
        Cli::Edit => {
            let config = command::Config::new(
                presenter::ConsolePresenter::new(),
                task_manager::InMemoryTaskManager::new(),
                command::EditConfig,
            );
            command::edit(config);
        }
    }
}