use copi_core::{AppState, start_api_service, start_usb_cdc_service};

#[tokio::main]
async fn main() {
    env_logger::init();

    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
    let state = AppState::new(cmd_tx);

    start_usb_cdc_service(cmd_rx);
    start_api_service(state).await;
}
