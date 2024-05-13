use clap::{command, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {

    #[command(subcommand)]
    pub subcommands: Option<SubCommands>,

    #[arg(global=true)]
    #[clap[long, short]]
    pub spotlight: Option<String>,
}

#[derive(Args, Debug)]
pub struct Live {

    // NOTE: Uncomment if you want custom flags for the `live` subcommand
    // pub spotlight: Option<String>
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    #[command(about="Toggles auto-refresh for score data",name="live")]
    LiveCommands(Live),
}