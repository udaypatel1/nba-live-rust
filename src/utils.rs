
use colored::*;
use serde_json::{self, Value};

pub fn display_per_game(game: &Value) {

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
        game_status.as_str().unwrap().trim().yellow()
    );
}

pub fn display_all_games(json: Value) {

    let scoreboard = json.get("scoreboard").unwrap();
    let game_data = scoreboard.get("games").unwrap();

    for game in game_data.as_array().unwrap() {

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
            game_status.as_str().unwrap().trim().yellow()
        );

    }
    
}