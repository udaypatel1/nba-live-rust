mod args;
mod utils;

use std::{thread::sleep, time::Duration};

use args::{CliArgs, SubCommands};
use clap::Parser;
use serde_json::Value;
use tokio;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let url = "https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json";

    let json = utils::fetch_data(url).await;

    let game_data: &Value = &json["scoreboard"]["games"];

    let cli_args = args::CliArgs::parse();

    if game_data.as_array().unwrap().len() > 0 {

        match cli_args {

            CliArgs {
                subcommands: Some(command),
                spotlight: Some(spotlight_tricode)
            } => {

                // match is here to pattern match potential other subcommands
                match command {
                    SubCommands::LiveCommands(_) => {

                        loop {

                            let json = utils::fetch_data(url).await;

                            let game_data = &json["scoreboard"]["games"];

                            let mut is_found = false;

                            for game in game_data.as_array().ok_or("Game Data JSON is Unstructured")? {

                                let away_team_tricode = &game["awayTeam"]["teamTricode"];
                                let home_team_tricode = &game["homeTeam"]["teamTricode"];
            
                                if spotlight_tricode.to_uppercase().eq(away_team_tricode) || spotlight_tricode.to_uppercase().eq(home_team_tricode) {
                                    is_found = true;
                                    utils::display_per_game(game);
                                    break;
                                }
                            }
            
                            if !is_found { println!("😔 Team data for Tricode [{}] is Unavailable", spotlight_tricode.bold()); break; }
                            
                            sleep(Duration::from_secs(2));
                        }
                    }
                }   
            },

            CliArgs {
                subcommands: Some(command),
                spotlight: None
            } => {
            },

            CliArgs {
                subcommands: None,
                spotlight: Some(spotlight_tricode)
            } => {
                let mut is_found = false;

                for game in game_data.as_array().ok_or("Game Data JSON is Unstructured")? {

                    let away_team_tricode = &game["awayTeam"]["teamTricode"];
                    let home_team_tricode = &game["homeTeam"]["teamTricode"];

                    if spotlight_tricode.to_uppercase().eq(away_team_tricode) || spotlight_tricode.to_uppercase().eq(home_team_tricode) {
                        is_found = true;
                        utils::display_per_game(game);
                        break;
                    }
                }

                if !is_found { println!("😔 Team data for Tricode [{}] is Unavailable", spotlight_tricode.bold()); }
            },

            CliArgs {
                subcommands: None,
                spotlight: None
            } => utils::display_all_games(json),
        }

    } 
    else {
        println!("😔 There are no games at the moment");
    }
    
    Ok(())

}