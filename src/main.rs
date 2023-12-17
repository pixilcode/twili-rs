use chrono::NaiveDate;
use clap::Parser;

mod command;
mod model;
mod presenter;
mod dao;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    List,
    Edit,
}

fn main() {
    let cli = Cli::parse();

    // TODO: use a real task manager
    let task_dao = dao::task_dao::InMemoryTaskDao::new_from_list(
        vec![
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
    );

    use crate::presenter::simple_console::*;

    match cli {
        Cli::List => {
            let config = command::Config::new(
                Presenter::new(TaskFormatter::Basic),
                task_dao,
                command::ListConfig,
            );
            command::list(config);
        }
        Cli::Edit => {
            let config = command::Config::new(
                Presenter::new(TaskFormatter::Basic),
                task_dao,
                command::EditConfig,
            );
            command::edit(config);
        }
        // TODO: add an 'interactive session' command
    }
}