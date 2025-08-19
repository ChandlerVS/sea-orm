use crate::DateTimeCrate;

use clap::{ArgAction, Parser};

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct GenerateEntityArguments {
    #[arg(long, help = "Generate entity file of compact format")]
    pub compact_format: bool,

    #[arg(long, help = "Generate entity file of expanded format")]
    pub expanded_format: bool,

    #[arg(long, help = "Generate entity file of frontend format")]
    pub frontend_format: bool,

    #[arg(
        long,
        help = "Generate entity file for hidden tables (i.e. table name starts with an underscore)"
    )]
    pub include_hidden_tables: bool,

    #[arg(
        short = 't',
        long,
        value_delimiter = ',',
        help = "Generate entity file for specified tables only (comma separated)"
    )]
    pub tables: Vec<String>,

    #[arg(
        long,
        value_delimiter = ',',
        default_value = "seaql_migrations",
        help = "Skip generating entity file for specified tables (comma separated)"
    )]
    pub ignore_tables: Vec<String>,

    #[arg(
        long,
        default_value = "1",
        help = "The maximum amount of connections to use when connecting to the database."
    )]
    pub max_connections: u32,

    #[arg(
        long,
        default_value = "30",
        long_help = "Acquire timeout in seconds of the connection used for schema discovery"
    )]
    pub acquire_timeout: u64,

    #[arg(
        short = 'o',
        long,
        default_value = "./",
        help = "Entity file output directory"
    )]
    pub output_dir: String,

    #[arg(
        short = 's',
        long,
        env = "DATABASE_SCHEMA",
        long_help = "Database schema\n \
                        - For MySQL, this argument is ignored.\n \
                        - For PostgreSQL, this argument is optional with default value 'public'."
    )]
    pub database_schema: Option<String>,

    #[arg(
        short = 'u',
        long,
        env = "DATABASE_URL",
        help = "Database URL",
        hide_env_values = true
    )]
    pub database_url: String,

    #[arg(
        long,
        default_value = "all",
        help = "Generate prelude.rs file (all, none, all-allow-unused-imports)"
    )]
    pub with_prelude: String,

    #[arg(
        long,
        default_value = "none",
        help = "Automatically derive serde Serialize / Deserialize traits for the entity (none, \
                serialize, deserialize, both)"
    )]
    pub with_serde: String,

    #[arg(
        long,
        help = "Generate a serde field attribute, '#[serde(skip_deserializing)]', for the primary key fields to skip them during deserialization, this flag will be affective only when '--with-serde' is 'both' or 'deserialize'"
    )]
    pub serde_skip_deserializing_primary_key: bool,

    #[arg(
        long,
        default_value = "false",
        help = "Opt-in to add skip attributes to hidden columns (i.e. when 'with-serde' enabled and column name starts with an underscore)"
    )]
    pub serde_skip_hidden_column: bool,

    #[arg(
        long,
        default_value = "false",
        long_help = "Automatically derive the Copy trait on generated enums.\n\
            Enums generated from a database don't have associated data by default, and as such can \
            derive Copy.
            "
    )]
    pub with_copy_enums: bool,

    #[arg(
        long,
        default_value_t,
        value_enum,
        help = "The datetime crate to use for generating entities."
    )]
    pub date_time_crate: DateTimeCrate,

    #[arg(
        long,
        short = 'l',
        default_value = "false",
        help = "Generate index file as `lib.rs` instead of `mod.rs`."
    )]
    pub lib: bool,

    #[arg(
        long,
        value_delimiter = ',',
        help = "Add extra derive macros to generated model struct (comma separated), e.g. `--model-extra-derives 'ts_rs::Ts','CustomDerive'`"
    )]
    pub model_extra_derives: Vec<String>,

    #[arg(
        long,
        value_delimiter = ',',
        help = r#"Add extra attributes to generated model struct, no need for `#[]` (comma separated), e.g. `--model-extra-attributes 'serde(rename_all = "camelCase")','ts(export)'`"#
    )]
    pub model_extra_attributes: Vec<String>,

    #[arg(
        long,
        value_delimiter = ',',
        help = "Add extra derive macros to generated enums (comma separated), e.g. `--enum-extra-derives 'ts_rs::Ts','CustomDerive'`"
    )]
    pub enum_extra_derives: Vec<String>,

    #[arg(
        long,
        value_delimiter = ',',
        help = r#"Add extra attributes to generated enums, no need for `#[]` (comma separated), e.g. `--enum-extra-attributes 'serde(rename_all = "camelCase")','ts(export)'`"#
    )]
    pub enum_extra_attributes: Vec<String>,

    #[arg(
        long,
        default_value = "false",
        long_help = "Generate helper Enumerations that are used by Seaography."
    )]
    pub seaography: bool,

    #[arg(
            long,
            default_value = "true",
            default_missing_value = "true",
            num_args = 0..=1,
            require_equals = true,
            action = ArgAction::Set,
            long_help = "Generate empty ActiveModelBehavior impls."
    )]
    pub impl_active_model_behavior: bool,
}
