# NBA Live: A TUI tool to view live game data

## Consumer Usage

As of version 1.0.0, the brew package is only available for Mac ARM OS users

#### Installation

1) `brew tap udaypatel1/nba`
2) `brew install nba`

## Get Started

1) Clone the repository `git clone https://github.com/udaypatel1/nba-live-rust.git`
2) Install [Rust](https://www.rust-lang.org/tools/install) to compile the code with Cargo
3) Run `cargo --version` to make sure Rust is installed properly
4) Execute `cargo run` to build the binary and run the tool

## Contributions

* Feel free to clone and contribute! You can make a branch and submit a PR for review
* Full JSON data endpoint can be found [here](https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json)

## Todo List

* ~~Add a color crate to stylize the output~~
* ~~Add proper CLI support with `clap` or other tooling to allow for future customizations~~
* Create a better visual output of score printouts (boxed? grid? centered? maybe behind a flag?)
* Allow for and pre-build cross-platform builds in target (x86_64 MacOS, Windows, Linux)
* Establish a proper Release Branch to create releases for version management
* Package Release Branch state and setup code for Homebrew public forum
* Make a similar tool for MLB with [this](https://statsapi.mlb.com/api/v1/schedule?sportId=1&sportId=51&sportId=21&startDate=2024-05-08&endDate=2024-05-08&timeZone=America/New_York&gameType=E&&gameType=S&&gameType=R&&gameType=F&&gameType=D&&gameType=L&&gameType=W&&gameType=A&&gameType=C&language=en&leagueId=104&&leagueId=103&&leagueId=160&&leagueId=590&hydrate=team,linescore(matchup,runners),xrefId,story,flags,statusFlags,broadcasts(all),venue(location),decisions,person,probablePitcher,stats,game(content(media(epg),summary),tickets),seriesStatus(useOverride=true)&sortBy=gameDate,gameStatus,gameType)

