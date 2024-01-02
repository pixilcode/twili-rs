use std::{env, path::PathBuf};

use clap::Parser;
use command::traits::TaskDao;

mod command;
mod model;
mod presenter;
mod dao;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// TODO: action is optional, default to `interact`
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    List,
    Edit,
    Interact,
    Version,
}

fn main() {
    #[cfg(debug_assertions)]
    {
        // Only load .env in debug mode
        // TODO: better error handling
        dotenvy::dotenv().ok();
    }

    let data_dir = default_data_directory();
    let list_name = DEFAULT_LIST_NAME;

    let mut task_dao = dao::sqlite::SqliteTaskDao::new(data_dir, list_name);

    task_dao.get_all_tasks().iter().for_each(|task| {
        println!("{:?}", task);
    });

    /*
    let cli = Cli::parse();

    // TODO: use a real task manager
    let task_dao = dao::in_memory::InMemoryTaskDao::new_from_list(
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
            let mut config = command::Config::new(
                Presenter::new(TaskFormatter::Basic),
                task_dao,
                command::ListConfig,
            );
            command::list(&mut config);
        }
        Cli::Edit => {
            let mut config = command::Config::new(
                Presenter::new(TaskFormatter::Basic),
                task_dao,
                command::EditConfig,
            );
            command::edit(&mut config);
        }
        Cli::Interact => {
            let mut config = command::Config::new(
                Presenter::new(TaskFormatter::Basic),
                task_dao,
                command::InteractConfig,
            );
            command::interact(&mut config);
        }
        // TODO: add an 'interactive session' command
    }
    */
}


// DEFAULTS

fn default_data_directory() -> PathBuf {
    let user_data_dir = dirs::data_local_dir();
    user_data_dir.unwrap_or(".".into()).join(".twili")
}

const DEFAULT_LIST_NAME: &'static str = "default";