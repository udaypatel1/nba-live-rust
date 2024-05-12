mod args;
mod utils;

use args::CliArgs;
use reqwest;
use serde_json::{self, Value};
use tokio;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let url = "https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json";

    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;

    let scoreboard = json.get("scoreboard").unwrap();
    let game_data = scoreboard.get("games").unwrap();

    let cli_args = args::parse_arguments();

    if game_data.as_array().unwrap().len() > 0 {

        match cli_args {

            CliArgs {
                team_spotlight: Some(spotlight_tricode)
            } => {
                let mut is_found = false;

                for game in game_data.as_array().ok_or("Game Data JSON is Unstructured")? {
                    let away_team = game.get("awayTeam").unwrap();
                    let home_team = game.get("homeTeam").unwrap();
            
                    let away_team_tricode = away_team.get("teamTricode").unwrap();
                    let home_team_tricode = home_team.get("teamTricode").unwrap();

                    if spotlight_tricode.to_uppercase().eq(away_team_tricode) || spotlight_tricode.to_uppercase().eq(home_team_tricode) {
                        is_found = true;
                        utils::display_per_game(game);
                        break;
                    }
                }

                if !is_found { println!("😔 Team data for Tricode [{}] is Unavailable", spotlight_tricode.bold()); }
            },

            CliArgs {
                team_spotlight: None
            } => utils::display_all_games(json),
        }

    } 
    else {
        println!("😔 There are no games at the moment");
    }
    
    Ok(())

}