use clap::{ArgAction, ArgGroup, Parser, Subcommand, ValueEnum};
#[cfg(feature = "codegen")]
use dotenvy::dotenv;

#[cfg(feature = "codegen")]
use crate::{handle_error, run_generate_command, run_migrate_command};
use crate::arguments::generate::GenerateEntityArguments;

#[derive(Parser, Debug)]
#[command(
    version,
    author,
    help_template = r#"{before-help}{name} {version}
{about-with-newline}

{usage-heading} {usage}

{all-args}{after-help}

AUTHORS:
    {author}
"#,
    about = r#"
   ____                 ___   ____   __  __        /\
  / ___|   ___   __ _  / _ \ |  _ \ |  \/  |      {.-}
  \___ \  / _ \ / _` || | | || |_) || |\/| |     ;_.-'\
   ___) ||  __/| (_| || |_| ||  _ < | |  | |    {    _.}_
  |____/  \___| \__,_| \___/ |_| \_\|_|  |_|     \.-' /  `,
                                                  \  |    /
  An async & dynamic ORM for Rust                  \ |  ,/
  ===============================                   \|_/

  Getting Started
    - Documentation: https://www.sea-ql.org/SeaORM
    - Tutorial: https://www.sea-ql.org/sea-orm-tutorial
    - Examples: https://github.com/SeaQL/sea-orm/tree/master/examples
    - Cookbook: https://www.sea-ql.org/sea-orm-cookbook

  Join our Discord server to chat with others in the SeaQL community!
    - Invitation: https://discord.com/invite/uCPdDXzbdv

  SeaQL Community Survey 2024
    - Link: https://sea-ql.org/community-survey

  If you like what we do, consider starring, sharing and contributing!
"#
)]
pub struct Cli {
    #[arg(global = true, short, long, help = "Show debug messages")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, PartialEq, Eq, Debug)]
pub enum Commands {
    #[command(
        about = "Codegen related commands",
        arg_required_else_help = true,
        display_order = 10
    )]
    Generate {
        #[command(subcommand)]
        command: GenerateSubcommands,
    },
    #[command(about = "Migration related commands", display_order = 20)]
    Migrate {
        #[arg(
            global = true,
            short = 'd',
            long,
            env = "MIGRATION_DIR",
            help = "Migration script directory.
If your migrations are in their own crate,
you can provide the root of that crate.
If your migrations are in a submodule of your app,
you should provide the directory of that submodule.",
            default_value = "./migration"
        )]
        migration_dir: String,

        #[arg(
            global = true,
            short = 's',
            long,
            env = "DATABASE_SCHEMA",
            long_help = "Database schema\n \
                        - For MySQL and SQLite, this argument is ignored.\n \
                        - For PostgreSQL, this argument is optional with default value 'public'.\n"
        )]
        database_schema: Option<String>,

        #[arg(
            global = true,
            short = 'u',
            long,
            env = "DATABASE_URL",
            help = "Database URL",
            hide_env_values = true
        )]
        database_url: Option<String>,

        #[command(subcommand)]
        command: Option<MigrateSubcommands>,
    },
}

#[derive(Subcommand, PartialEq, Eq, Debug)]
pub enum MigrateSubcommands {
    #[command(about = "Initialize migration directory", display_order = 10)]
    Init,
    #[command(about = "Generate a new, empty migration", display_order = 20)]
    Generate {
        #[arg(required = true, help = "Name of the new migration")]
        migration_name: String,

        #[arg(
            long,
            default_value = "true",
            help = "Generate migration file based on Utc time",
            conflicts_with = "local_time",
            display_order = 1001
        )]
        universal_time: bool,

        #[arg(
            long,
            help = "Generate migration file based on Local time",
            conflicts_with = "universal_time",
            display_order = 1002
        )]
        local_time: bool,
    },
    #[command(
        about = "Drop all tables from the database, then reapply all migrations",
        display_order = 30
    )]
    Fresh,
    #[command(
        about = "Rollback all applied migrations, then reapply all migrations",
        display_order = 40
    )]
    Refresh,
    #[command(about = "Rollback all applied migrations", display_order = 50)]
    Reset,
    #[command(about = "Check the status of all migrations", display_order = 60)]
    Status,
    #[command(about = "Apply pending migrations", display_order = 70)]
    Up {
        #[arg(short, long, help = "Number of pending migrations to apply")]
        num: Option<u32>,
    },
    #[command(about = "Rollback applied migrations", display_order = 80)]
    Down {
        #[arg(
            short,
            long,
            default_value = "1",
            help = "Number of applied migrations to be rolled back",
            display_order = 90
        )]
        num: u32,
    },
}

#[derive(Subcommand, PartialEq, Eq, Debug)]
pub enum GenerateSubcommands {
    #[command(about = "Generate entity")]
    #[command(group(ArgGroup::new("formats").args(&["compact_format", "expanded_format", "frontend_format"])))]
    #[command(group(ArgGroup::new("group-tables").args(&["tables", "include_hidden_tables"])))]
    Entity(GenerateEntityArguments),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum, Default)]
pub enum DateTimeCrate {
    #[default]
    Chrono,
    Time,
}

/// Use this to build a local, version-controlled `sea-orm-cli` in dependent projects
/// (see [example use case](https://github.com/SeaQL/sea-orm/discussions/1889)).
#[cfg(feature = "codegen")]
pub async fn main() {
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
