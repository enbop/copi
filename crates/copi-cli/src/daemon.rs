use copi_core::{AppState, open_copi_serial, start_api_service, start_usb_cdc_service};

pub async fn start_daemon() {
    log::info!("Starting Copi daemon...");
    let (request_tx, request_rx) = tokio::sync::mpsc::unbounded_channel();
    let (response_tx, response_rx) = tokio::sync::mpsc::unbounded_channel();
    let state = AppState::new(request_tx, response_rx);

    let port = open_copi_serial();
    tokio::spawn(start_api_service(state));
    start_usb_cdc_service(port, request_rx, response_tx).await;
}
