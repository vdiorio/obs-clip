use anyhow::Result;
use obws::Client;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Recupera valores do .env com defaults
    let password = env::var("OBS_PASSWORD")?;
    let host = env::var("OBS_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = env::var("OBS_PORT")
        .unwrap_or_else(|_| "4455".to_string())
        .parse()
        .expect("OBS_PORT must be a number");

    // Conectar ao OBS
    let client = Client::connect(host, port, Some(password)).await?;

    // Obter status do Replay Buffer
    let is_buffer_on = client.replay_buffer().status().await?;

    if is_buffer_on {
        // Replay Buffer já está ativo → salva clipe
        client.replay_buffer().save().await?;
    } else {
        // Replay Buffer desligado → liga
        client.replay_buffer().start().await?;
    }

    Ok(())
}
