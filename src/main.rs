use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    List,
    Edit,
}

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::List => println!("List"),
        Cli::Edit => println!("Edit"),
    }
}