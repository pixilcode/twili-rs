# Setup the development environment + useful scripts

## Install the ADR tool
sudo apt install adr-tools

## Install the diesel cli (specifically for SQLite)
cargo install diesel_cli --no-default-features --features sqlite-bundled

## Create a local database
diesel setup --database-url ./db/test.db

## Create migrations and run them
## (Don't forget to write the up and down migrations)
diesel migration generate create_tasks_table
diesel migration run --database-url ./db/test.db