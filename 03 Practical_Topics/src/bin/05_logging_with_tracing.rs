use anyhow::Result;
use tracing::{debug, error, info, instrument, warn};
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    // RUST_LOG 控制日志级别，例如：
    // $env:RUST_LOG="debug"
    // cargo run --bin 05_logging_with_tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    info!("application started");
    debug!("debug details are hidden unless RUST_LOG enables them");

    match process_order(42, 3) {
        Ok(total) => info!(order_id = 42, total, "order processed"),
        Err(error) => error!(%error, "order failed"),
    }

    if let Err(error) = process_order(7, 0) {
        warn!(%error, "recoverable order problem");
    }

    Ok(())
}

#[instrument]
fn process_order(order_id: u64, item_count: u32) -> Result<u32> {
    if item_count == 0 {
        anyhow::bail!("order {order_id} has no items");
    }

    Ok(item_count * 100)
}
