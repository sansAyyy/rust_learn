use tracing::{debug, info, instrument, warn};
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 默认 info。可以设置：
    // $env:RUST_LOG="debug"
    // cargo run --bin 02_logging_debug
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    info!("application started");
    let result = process_batch(&[1, 2, 3, 4]);
    info!(result, "batch processed");

    if process_batch(&[]).is_none() {
        warn!("empty batch was skipped");
    }

    Ok(())
}

#[instrument]
fn process_batch(values: &[i32]) -> Option<i32> {
    debug!(len = values.len(), "processing batch");
    values.iter().copied().reduce(|acc, value| acc + value)
}
