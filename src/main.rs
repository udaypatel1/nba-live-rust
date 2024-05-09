mod args;

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
    let spotlight = cli_args.team_spotlight;

    if game_data.as_array().unwrap().len() > 0 {

        for game in game_data.as_array().ok_or("Expected an array")? {
        
            let away_team = game.get("awayTeam").unwrap();
            let home_team = game.get("homeTeam").unwrap();
    
            let away_team_tricode = away_team.get("teamTricode").unwrap();
            let home_team_tricode = home_team.get("teamTricode").unwrap();

            if spotlight.is_some() && (
                spotlight.clone().unwrap().to_uppercase().eq(away_team_tricode) ||
                spotlight.clone().unwrap().to_uppercase().eq(home_team_tricode)
            ) {
                display(game);
                break;
            } else if spotlight.is_some() {
                println!("😔 That team is not currently playing");
                break;
            }

            display(game);
        }
    }
    else{
        println!("😔 There are no games at the moment");
    }
    
    Ok(())

}

fn display(game: &Value) {

    let away_team = game.get("awayTeam").unwrap();
    let home_team = game.get("homeTeam").unwrap();

    let away_team_tricode = away_team.get("teamTricode").unwrap();
    let away_team_score = away_team.get("score").unwrap();

    let home_team_tricode = home_team.get("teamTricode").unwrap();
    let home_team_score = home_team.get("score").unwrap();

    let game_status = game.get("gameStatusText").unwrap();

    println!("🏀 {}: {} - {}: {} ({})", 
        away_team_tricode.as_str().unwrap().bold(),
        away_team_score.to_string().bright_blue(),
        home_team_tricode.as_str().unwrap().bold(),
        home_team_score.to_string().bright_blue(),
        game_status.as_str().unwrap().yellow()
    );

}
