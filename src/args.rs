
use clap::*;

#[derive(Debug)]
pub struct CliArgs {
    pub team_spotlight: Option<String>,
}

pub fn parse_arguments() -> CliArgs {

    let matches = Command::new("NBA Live!")
        .about("A terminal based CLI to highlight real time game data")
        .arg(
            Arg::new("spotlight")
                .help("Filter to spotlight a specific team")
                .long("spotlight")
                .short('s')
                .value_name("SPOTLIGHT")
                .num_args(1)
                .required(false),
        )
        .get_matches();

    let team_spotlight: Option<String> = matches.get_one::<String>("spotlight").cloned();

    CliArgs {
        team_spotlight,
    }
}