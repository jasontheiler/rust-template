#![forbid(unsafe_code)]
#![deny(nonstandard_style)]
#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]

#[macro_use]
extern crate log;

mod config;

use crate::config::Config;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let config = Config::new()?;

    env_logger::builder()
        .parse_filters(&config.log)
        .try_init()?;

    info!("{:?}", config);

    Ok(())
}
