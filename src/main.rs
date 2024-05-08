use reqwest;
use serde_json::{self, Value};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let url = "https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json";

    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;

    let game_data = json.get("scoreboard").unwrap();

    println!("Data: {}", game_data);

    Ok(())

}
