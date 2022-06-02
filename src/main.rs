use std::time::Duration;

use color_eyre::{eyre::Context, Report, Result};
use tokio::time::sleep;
use tokio_graceful_shutdown::{SubsystemHandle, Toplevel};
use tracing::info;

mod_use::mod_use![config];

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;
    tracing_subscriber::fmt().init();

    // Load the config for first time, then cache it
    let config = Config::get();
    info!(?config, "Config loaded!");

    // For hyper example, see: https://github.com/Finomnis/tokio-graceful-shutdown/blob/main/examples/hyper.rs
    Toplevel::new()
        .start("app1", app)
        .catch_signals()
        .handle_shutdown_requests::<Report>(Duration::from_millis(1000))
        .await
        .wrap_err("App failed")
}

async fn app(sys: SubsystemHandle) -> Result<()> {
    // Load the config from cache, which is cheap
    let _config = Config::get();

    let mut count = 3;

    while count > 0 {
        // Do some job
        sleep(Duration::from_secs(1)).await;
        info!("Tick!");
        count -= 1;
    }

    info!("Gonna go!");

    sys.request_shutdown();

    Ok(())
}
