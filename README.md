# NBA Live: A TUI tool to view live game data

## Get Started

1) Clone the repository `git clone https://github.com/udaypatel1/nba-live-rust.git`
2) Install [Rust]("https://www.rust-lang.org/tools/install") to compile the code with Cargo
3) Run `cargo --version` to make sure Rust is installed properly
4) Execute `cargo run` to build the binary and run the tool

## Contributions

* Feel free to clone and contribute! You can make a branch and submit a PR for review
* Full JSON data endpoint can be found [here]("https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json")

## Todo List

* Add a color crate to stylize the output
* Add proper CLI support with `clap` or other tooling to allow for future customizations
* Establish a proper Release Branch to create releases for version management
* Package Release Branch state and setup code for Homebrew public forum
