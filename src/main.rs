
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

    // println!("team spotlight: {:?}", cli_args.team_spotlight);

    if game_data.as_array().unwrap().len() > 0 {

        for game in game_data.as_array().ok_or("Expected an array")? {
        
            let away_team = game.get("awayTeam").unwrap();
            let home_team = game.get("homeTeam").unwrap();
    
            let away_team_tricode = away_team.get("teamTricode").unwrap();
            let away_team_score = away_team.get("score").unwrap();
            let home_team_tricode = home_team.get("teamTricode").unwrap();
            let home_team_score = home_team.get("score").unwrap();
    
            let game_status = game.get("gameStatusText").unwrap();
    
            println!("🏀 {}: {} - {}: {} ({})", 
                away_team_tricode.as_str().unwrap().bold(), 
                away_team_score.to_string().blue(), 
                home_team_tricode.as_str().unwrap().bold(), 
                home_team_score.to_string().blue(),
                game_status.as_str().unwrap().yellow()
            );
        }
    }
    else{
        println!("😔 There are no games at the moment");
    }
    
    Ok(())

}
