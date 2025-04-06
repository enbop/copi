use std::path::PathBuf;

use clap::{Parser, Subcommand};
use copi_core::{AppState, open_copi_serial, start_api_service, start_usb_cdc_service};

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
    let env = env_logger::Env::default().filter_or("COPI_LOG", "info");
    env_logger::init_from_env(env);

    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Commands::List => {
                utils::list_boot_pico();
                return;
            }
            Commands::Flash { pico } => {
                log::info!("Flashing copi firmware to: {}", pico.display());
                flash::flash(pico);
                return;
            }
        }
    }

    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
    let state = AppState::new(cmd_tx);

    let port = open_copi_serial();
    tokio::spawn(start_api_service(state));
    start_usb_cdc_service(port, cmd_rx).await;
}
