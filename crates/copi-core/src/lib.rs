mod api;
// #[cfg(target_os = "android")]
pub mod mobile;
// mod types;
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/copi.rs"));
}

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex, atomic::AtomicU32},
};

use anyhow::{Context, Result};
use axum::{
    Router,
    routing::{get, post},
};
use generated::request_body::Message;
use generated::*;
use prost::Message as _;
use tokio::io::AsyncReadExt;
use tokio::{
    io::AsyncWriteExt,
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task::JoinHandle,
};

pub const MAX_USB_PACKET_SIZE: usize = 64;

struct NonZeroU32Count(AtomicU32);

impl NonZeroU32Count {
    pub fn new() -> Self {
        NonZeroU32Count(AtomicU32::new(1))
    }

    pub fn next(&self) -> u32 {
        // This operation wraps around on overflow.
        let v = self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if v == 0 {
            return self.next();
        }
        v
    }
}

#[derive(Clone)]
struct DeviceChannel {
    non_zero_count: Arc<NonZeroU32Count>,
    callbacks: Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseBody>>>>,
    request_tx: Arc<UnboundedSender<CopiRequest>>,
}

impl DeviceChannel {
    pub async fn query(&self, msg: RequestBody) -> Result<ResponseBody> {
        let id = self.non_zero_count.next();
        let (tx, rx) = oneshot::channel();
        {
            let mut callbacks = self.callbacks.lock().unwrap();
            callbacks.insert(id, tx);
        }

        let mut request = CopiRequest::default(); // (id, msg);
        request.request_id = id;
        request.payload.replace(msg);
        self.request_tx
            .send(request)
            .with_context(|| "Failed to send request")?;

        let res = rx
            .await
            .with_context(|| "Failed to receive response, sender dropped")?;
        Ok(res)
    }

    pub fn send(&self, msg: RequestBody) -> Result<()> {
        let mut request = CopiRequest::default();
        request.payload.replace(msg);
        self.request_tx
            .send(request)
            .with_context(|| "Failed to send request")?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct AppState {
    device_channel: DeviceChannel,
    response_task: Arc<JoinHandle<()>>,
}

impl AppState {
    pub fn new(
        request_tx: UnboundedSender<CopiRequest>,
        response_rx: UnboundedReceiver<CopiResponse>,
        #[cfg(target_os = "android")] runtime: &tokio::runtime::Runtime,
    ) -> Self {
        let device_channel = DeviceChannel {
            non_zero_count: Arc::new(NonZeroU32Count::new()),
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            request_tx: Arc::new(request_tx),
        };

        let callbacks = device_channel.callbacks.clone();
        #[cfg(not(target_os = "android"))]
        let response_task = tokio::spawn(Self::handle_response(response_rx, callbacks));
        #[cfg(target_os = "android")]
        let response_task = runtime.spawn(Self::handle_response(response_rx, callbacks));

        Self {
            device_channel,
            response_task: Arc::new(response_task),
        }
    }

    async fn handle_response(
        mut response_rx: UnboundedReceiver<CopiResponse>,
        callbacks: Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseBody>>>>,
    ) {
        while let Some(resp) = response_rx.recv().await {
            let id = resp.request_id;
            if id == 0 {
                log::warn!("Received response with ID 0, ignoring");
                continue;
            }
            let Some(payload) = resp.payload else {
                log::warn!("Received response with no payload, ignoring");
                continue;
            };

            let mut callbacks = callbacks.lock().unwrap();
            if let Some(sender) = callbacks.remove(&id) {
                if sender.send(payload).is_err() {
                    log::warn!("Failed to send response to callback");
                }
            } else {
                log::warn!("No callback found for response ID {}", id);
            }
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
pub fn open_copi_serial() -> tokio_serial::SerialStream {
    use tokio_serial::SerialPortBuilderExt as _;

    let Some(device) = serialport::available_ports()
        .unwrap()
        .into_iter()
        .find(|s| match &s.port_type {
            serialport::SerialPortType::UsbPort(info) => info.vid == 0x9527 && info.pid == 0xacdc,
            _ => false,
        })
    else {
        log::warn!("Device not found");
        std::process::exit(1);
    };

    log::info!("Found device: {:?}", device.port_name);

    tokio_serial::new(device.port_name, 0)
        .open_native_async()
        .unwrap()
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
pub async fn start_usb_cdc_service(
    mut port: tokio_serial::SerialStream,
    mut request_rx: UnboundedReceiver<CopiRequest>,
    response_tx: UnboundedSender<CopiResponse>,
) {
    let mut request_buf = [0u8; MAX_USB_PACKET_SIZE];
    let mut response_buf = [0u8; MAX_USB_PACKET_SIZE];
    loop {
        tokio::select! {
            req = request_rx.recv() => {
                if let Some(req) = req {
                    // TODO use buf
                    // TODO check size
                    match port.write_all(&req.encode_to_vec()).await {
                        Ok(_) => {
                            log::info!("Sent command: {:?}", req);
                        }
                        Err(e) => {
                            log::error!("Failed to send command: {:?}", e);
                        }
                    }
                } else {
                    log::warn!("Command receiver closed");
                    break;
                }
            }
            res = port.read(&mut response_buf) => {
                match res {
                    Ok(n) => {
                        if n == 0 {
                            log::warn!("Received empty response");
                            continue;
                        }

                        let response = CopiResponse::decode(&response_buf[..n]);
                        match response {
                            Ok(resp) => {
                                log::info!("Received response: {:?}", resp);
                                if response_tx.send(resp).is_err() {
                                    log::warn!("Failed to send response to receiver");
                                    break;
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to decode response: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to read response: {:?}", e);
                        break;
                    }
                }
            }
        }
    }
}

pub async fn start_api_service(state: AppState) {
    let app = Router::new()
        .route("/query", post(api::query))
        .route("/command", post(api::command))
        .route("/playground", get(api::playground::playground))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8899").await.unwrap();
    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
