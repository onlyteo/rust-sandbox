use tokio::task::JoinError;

pub fn run(name: &str, result: Result<anyhow::Result<()>, JoinError>) -> anyhow::Result<()> {
    match result {
        Ok(Ok(())) => {
            tracing::info!("{} completed ", name);
            Ok(())
        }
        Ok(Err(e)) => {
            tracing::error!("{} avsluttet med feil: {}", name, e);
            Err(e)
        }
        Err(e) => {
            tracing::error!("Feil i spawned task for {}: {}", name, e);
            Err(e.into())
        }
    }
}

pub fn shutdown(result: Result<&str, anyhow::Error>) -> anyhow::Result<()> {
    tracing::info!("Recieved shutdown signal: {:?}", result);
    Ok(())
}
