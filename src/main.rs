use anyhow::Result;
use obws::Client;

#[tokio::main]
async fn main() -> Result<()> {

    // CONFIGURAÇÃO MANUAL
    let password = "SUA_SENHA_OBS";
    let host = "localhost";
    let port: u16 = 4455;

    // Conectar ao OBS
    let client = Client::connect(host, port, Some(password)).await?;

    // Verificar status do Replay Buffer
    let is_buffer_on = client.replay_buffer().status().await?;

    if is_buffer_on {
        // Replay já ativo → salva clip
        client.replay_buffer().save().await?;
        println!("Replay salvo!");
    } else {
        // Replay desligado → inicia
        client.replay_buffer().start().await?;
        println!("Replay buffer iniciado!");
    }

    Ok(())
}