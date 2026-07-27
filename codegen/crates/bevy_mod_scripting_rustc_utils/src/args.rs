use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser, serde::Serialize, serde::Deserialize)]
#[command(
    version,
    about,
    long_about,
    disable_help_flag = true,
    bin_name = "cargo bms-rustc-utils",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, serde::Serialize, serde::Deserialize)]
pub enum Command {
    /// Generate implementation for a function argument.
    ArgImplements(ArgImplementsArgs),
}

#[derive(Debug, Args, serde::Serialize, serde::Deserialize)]
pub struct ArgImplementsArgs {
    /// Name of the trait to implement.
    #[arg(long = "trait")]
    pub trait_name: String,

    /// Type containing the function.
    #[arg(long = "type")]
    pub type_name: String,

    /// Function name.
    #[arg(long)]
    pub function: String,

    /// Function argument name.
    #[arg(long)]
    pub arg: String,
}