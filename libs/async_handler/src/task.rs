use tokio::signal::unix::{SignalKind, signal};

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
