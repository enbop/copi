use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex, atomic::AtomicU16},
};

use anyhow::{Context, Result};
use axum::{
    Router,
    routing::{get, post},
};
use copi_protocol::{CopiRequest, CopiResponse, DeviceMessage, HostMessage};
use tokio::{
    io::AsyncWriteExt,
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task::JoinHandle,
};

mod api;
// #[cfg(target_os = "android")]
pub mod mobile;
mod types;

pub const MAX_USB_PACKET_SIZE: usize = 64;

struct NonZeroU16Count(AtomicU16);

impl NonZeroU16Count {
    pub fn new() -> Self {
        NonZeroU16Count(AtomicU16::new(1))
    }

    pub fn next(&self) -> u16 {
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
    non_zero_count: Arc<NonZeroU16Count>,
    callbacks: Arc<Mutex<HashMap<u16, oneshot::Sender<DeviceMessage>>>>,
    request_tx: Arc<UnboundedSender<CopiRequest>>,
}

impl DeviceChannel {
    pub async fn fetch(&mut self, msg: HostMessage) -> Result<DeviceMessage> {
        let id = self.non_zero_count.next();
        let (tx, rx) = oneshot::channel();
        {
            let mut callbacks = self.callbacks.lock().unwrap();
            callbacks.insert(id, tx);
        }

        let request = CopiRequest::new(id, msg);
        self.request_tx
            .send(request)
            .with_context(|| "Failed to send request")?;

        let res = rx
            .await
            .with_context(|| "Failed to receive response, sender dropped")?;
        Ok(res)
    }

    pub fn send(&self, msg: HostMessage) -> Result<()> {
        let request = CopiRequest::new_without_id(msg);
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
            non_zero_count: Arc::new(NonZeroU16Count::new()),
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
        callbacks: Arc<Mutex<HashMap<u16, oneshot::Sender<DeviceMessage>>>>,
    ) {
        while let Some(resp) = response_rx.recv().await {
            let id = resp.request_id();
            if id == 0 {
                log::warn!("Received response with ID 0, ignoring");
                continue;
            }

            let mut callbacks = callbacks.lock().unwrap();
            if let Some(sender) = callbacks.remove(&id) {
                if sender.send(resp.into_message()).is_err() {
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
    use tokio::io::AsyncReadExt;

    let mut request_buf = [0u8; MAX_USB_PACKET_SIZE];
    let mut response_buf = [0u8; MAX_USB_PACKET_SIZE];
    loop {
        tokio::select! {
            req = request_rx.recv() => {
                if let Some(cmd) = req {
                    let len = minicbor::len(&cmd);
                    minicbor::encode(&cmd, request_buf.as_mut()).unwrap();

                    match port.write_all(&request_buf[..len]).await {
                        Ok(_) => {
                            log::info!("Sent command: {:?}", cmd);
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

                        let response = minicbor::decode::<CopiResponse>(&response_buf[..n]);
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
        .route("/gpio/output-init", post(api::gpio::output_init))
        .route("/gpio/output-set", post(api::gpio::output_set))
        .route("/pwm/init", post(api::pwm::init))
        .route(
            "/pwm/set-duty-cycle-percent",
            post(api::pwm::set_duty_cycle_percent),
        )
        .route("/pio/load_program", post(api::pio::load_program))
        .route("/pio/sm_init", post(api::pio::sm_init))
        .route("/pio/sm_set_enabled", post(api::pio::sm_set_enabled))
        .route("/pio/sm_push", post(api::pio::sm_push))
        .route("/pio/sm_exec_instr", post(api::pio::sm_exec_instr))
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
