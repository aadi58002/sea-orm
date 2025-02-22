use clap::Parser;
use dotenvy::dotenv;
use sea_orm_cli::{handle_error, run_generate_command, run_migrate_command, Cli, Commands};

#[async_std::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();
    let verbose = cli.verbose;

    match cli.command {
        Commands::Generate { command } => {
            run_generate_command(command, verbose)
                .await
                .unwrap_or_else(handle_error);
        }
        Commands::Migrate {
            migration_dir,
            database_schema,
            database_url,
            command,
        } => run_migrate_command(
            command,
            &migration_dir,
            database_schema,
            database_url,
            verbose,
        )
        .unwrap_or_else(handle_error),
    }
}
