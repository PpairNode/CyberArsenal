use tracing_subscriber::{
    prelude::*,
    fmt,
    Registry
};
use std::fs::{OpenOptions, create_dir_all};
use std::path::Path;
use anyhow::Result;


pub(crate) fn init_tracing() -> Result<()> {
    let path = "logs/app.log";

    // Create parent directories if they don't exist
    if let Some(parent) = Path::new(path).parent() {
        create_dir_all(parent)?;
    }

    let log_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    let subscriber = Registry::default()
        .with(
            // log-error file, to log the errors that arise
            fmt::layer()
                .with_writer(log_file)
        );

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing default failed");

    Ok(())
}