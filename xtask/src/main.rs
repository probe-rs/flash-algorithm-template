mod export;
mod iterate;

use clap::Parser;
use color_eyre::eyre::Result;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

pub type DynError = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    color_eyre::install()?;

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    match Cli::parse() {
        Cli::Export => export::export().map(|_| ()),
        Cli::Iterate => iterate::iterate(),
    }
}

#[derive(clap::Parser)]
#[clap(
    about = "Utilities to generate flash algorithms for probe-rs",
    author = "Noah Hüsser <yatekii@yatekii.ch> / Dominik Böhi <dominik.boehi@gmail.ch> / Dario Nieuwenhuis <dirbaio@dirbaio.net>"
)]
enum Cli {
    /// Export the flash algorithm to the probe-rs format.
    Export,
    /// Run the algoritm on the target and show its output.
    Iterate,
}
