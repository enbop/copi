use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod daemon;
mod flash;
mod query;
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

    Daemon,

    Query(Query),
}

#[derive(Debug, Parser)]
struct Query {
    args: Vec<String>,
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
            Commands::Daemon => {
                daemon::start_daemon().await;
                return;
            }
            Commands::Query(q) => {
                query::start_query(q.args).await;
                return;
            }
        }
    }
}
