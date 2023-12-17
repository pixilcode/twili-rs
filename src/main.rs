use chrono::NaiveDate;
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

    let task_manager = task_manager::InMemoryTaskManager::new_from_groups(
        vec![
            model::TaskGroup {
                name: "Food A".to_string(),
                tasks: vec![
                    model::Task {
                        id: "buy-milk".to_string(),
                        name: "Buy milk".to_string(),
                        due_date: NaiveDate::from_ymd_opt(2021, 7, 31).unwrap(),
                        due_time: None,
                        complete: false,
                    },
                    model::Task {
                        id: "buy-cheese".to_string(),
                        name: "Buy cheese".to_string(),
                        due_date: NaiveDate::from_ymd_opt(2021, 8, 1).unwrap(),
                        due_time: None,
                        complete: false,
                    },
                ],
            },
            model::TaskGroup {
                name: "Food B".to_string(),
                tasks: vec![
                    model::Task {
                        id: "buy-bread".to_string(),
                        name: "Buy bread".to_string(),
                        due_date: NaiveDate::from_ymd_opt(2021, 8, 3).unwrap(),
                        due_time: None,
                        complete: false,
                    },
                    model::Task {
                        id: "buy-butter".to_string(),
                        name: "Buy butter".to_string(),
                        due_date: NaiveDate::from_ymd_opt(2021, 8, 4).unwrap(),
                        due_time: None,
                        complete: false,
                    },
                ],
            },
        ]
    );

    match cli {
        Cli::List => {
            let config = command::Config::new(
                presenter::console::Presenter::new(presenter::console::TaskFormatter::Basic),
                task_manager,
                command::ListConfig,
            );
            command::list(config);
        }
        Cli::Edit => {
            let config = command::Config::new(
                presenter::console::Presenter::new(presenter::console::TaskFormatter::Basic),
                task_manager,
                command::EditConfig,
            );
            command::edit(config);
        }
    }
}