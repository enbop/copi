use std::path::PathBuf;

use clap::{Parser, Subcommand};
use copi_core::{AppState, start_api_service, start_usb_cdc_service};

mod flash;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "Copi", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all connected boot pico devices
    List,

    /// Flash copi firmware to the pico device
    Flash {
        /// Path to the pico device
        #[arg(value_name = "PICO DEVICE")]
        pico: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Commands::List => {
                utils::list_boot_pico();
                return;
            }
            Commands::Flash { pico } => {
                println!("Flashing copi firmware to: {}", pico.display());
                flash::flash(pico);
                return;
            }
        }
    }

    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
    let state = AppState::new(cmd_tx);

    tokio::spawn(start_api_service(state));
    start_usb_cdc_service(cmd_rx).await;
}
