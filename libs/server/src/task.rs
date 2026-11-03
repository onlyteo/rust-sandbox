use tokio::signal::unix::{SignalKind, signal};
use tokio::task::JoinError;

const SIGTERM: &str = "SIGTERM";
const SIGINT: &str = "SIGINT";

pub async fn shutdown_signal() -> anyhow::Result<&'static str> {
    let mut term_signal = signal(SignalKind::terminate())?;
    let mut interrupt_signal = signal(SignalKind::interrupt())?;
    tokio::select! {
        _ = term_signal.recv() => Ok(SIGTERM),
        _ = interrupt_signal.recv() => Ok(SIGINT)
    }
}

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
