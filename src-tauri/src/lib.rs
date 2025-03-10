use std::sync::Arc;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::thread;
use std::time::Duration;
use tauri::async_runtime::spawn;
use tauri::Emitter;
use tauri::Manager;

use tauri::State;
use tokio::io::AsyncReadExt;

// use tauri::Manager; // for window.emit

// We'll use opencv to capture frames
use opencv::{
    core::Vector,
    imgcodecs,
    prelude::*,
    videoio::{self, VideoCapture},
};

use base64::{engine::general_purpose::STANDARD, Engine as _};

use cameleon::payload::Payload;
use cameleon::payload::PayloadReceiver;
use cameleon::u3v;
use cameleon::Camera;
use cameleon::PayloadStream;

use lazy_static::lazy_static;
use serde::Serialize;
use std::sync::Mutex;
use tauri::{ipc::Channel, AppHandle};
// use aravis::prelude::*;
use tracing::{debug, error, info, trace, warn};

#[derive(Default)]
struct CameraController {
    pub camera_handles: Vec<Camera<u3v::ControlHandle, u3v::StreamHandle>>,
    pub camera_payload_rx: Vec<PayloadReceiver>,
}

#[derive(Default)]
struct ImageDataFrame {
    pub image_data: Vec<Payload>,
}

impl ImageDataFrame {
    pub fn add_payload(&mut self, payload: Payload) {
        self.image_data.push(payload);
    }

    pub fn is_empty(&self) -> bool {
        self.image_data.is_empty()
    }
}

#[derive(Default)]
struct DataQueue {
    pub queue: Vec<ImageDataFrame>,
}

impl DataQueue {
    pub fn add_image_data_frame(&mut self, image_data_frame: ImageDataFrame) {
        // TODO: check if the queue is full
        self.queue.push(image_data_frame);
    }

    pub fn get_latest_image_data_frame(&mut self) -> Option<ImageDataFrame> {
        self.queue.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl CameraController {
    pub fn init() -> Option<Self> {
        // Enumerates all cameras connected to the host.
        let mut cameras: Vec<cameleon::Camera<u3v::ControlHandle, u3v::StreamHandle>> =
            u3v::enumerate_cameras()
                .expect("All connected cameras should have GenICam 3.0 interface");

        if cameras.is_empty() {
            println!("init failed: no camera found");
            info!("init failed: no camera found");
            return None;
        } else {
            let mut payload_rxs = vec![];
            for camera in &mut cameras {
                println!("Get camera info: {:#?}", camera.info());

                // Open the camera.
                camera.open().unwrap();
                // Load `GenApi` context.
                let gen_api_context = camera.load_context().unwrap();
                println!("GenApi context: {:#?}", gen_api_context);

                // Start streaming. Channel capacity is set to 3.
                let payload_rx = camera.start_streaming(3).unwrap();
                let _img = payload_rx.recv_blocking();
                match _img {
                    Ok(payload) => {
                        println!("payload: {:?}", payload.image_info());
                    }
                    Err(e) => {
                        println!("blocking recv stream data error: {:?}", e);
                    }
                }
                payload_rxs.push(payload_rx);
            }

            Some(CameraController {
                camera_handles: cameras,
                camera_payload_rx: payload_rxs,
            })
        }
    }

    pub async fn get_latest_frame(payload_rx: &Vec<PayloadReceiver>) -> ImageDataFrame {
        let mut image_data_frame = ImageDataFrame::default();
        for payload_rx in payload_rx {
            match payload_rx.recv().await {
                Ok(payload) => {
                    image_data_frame.add_payload(payload);
                }
                Err(e) => {
                    println!("stream data error: {:?}", e);
                }
            }
        }
        image_data_frame
    }
}

// struct ImageBuffer {

// }

lazy_static! {
    static ref CAMERA: Mutex<Option<VideoCapture>> = Mutex::new(None);
}

lazy_static! {
    static ref FRAME_ID: Mutex<usize> = Mutex::new(0);
}

lazy_static! {
    static ref CAMERA_HANDLE: Mutex<Vec<cameleon::Camera<u3v::ControlHandle, u3v::StreamHandle>>> =
        Mutex::new(vec![]);
}

lazy_static! {
    static ref CAMERA_CONTROLLER: Mutex<CameraController> = Mutex::new(CameraController::default());
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
enum DownloadEvent<'a> {
    #[serde(rename_all = "camelCase")]
    Started {
        image_data: &'a str,
        download_id: usize,
        content_length: usize,
    },
    #[serde(rename_all = "camelCase")]
    Progress {
        download_id: usize,
        chunk_length: usize,
    },
    #[serde(rename_all = "camelCase")]
    Finished { download_id: usize },
}

type DataQueueRef = Arc<Mutex<DataQueue>>;

#[tauri::command]
fn fetch_camera_feed(data_queue: State<DataQueueRef>, on_event: Channel<DownloadEvent>) {
    let frame = {
        info!("fetching camera feed from app state");
        let mut binding = data_queue.lock().unwrap();
        binding.get_latest_image_data_frame()
    };

    let Some(image_data_frame) = frame else {
        // println!("No frame found");
        return;
    };

    println!("Fetching camera feed...");
    let mut frame_id = FRAME_ID.lock().unwrap();
    *frame_id += 1;

    // include jpg file in the resources

    // let mut buf = Vector::new();
    // let params = Vector::new();
    // imgcodecs::imencode(".jpg", &frame, &mut buf, &params).unwrap();

    // Convert to base64
    let base64 = "test";
    // print first 100 characters of the base64 string
    // println!("Base64: {}", &base64[1400..1500]);
    println!("frame_id: {}", *frame_id);
    on_event
        .send(DownloadEvent::Started {
            image_data: &base64,
            download_id: *frame_id,
            content_length: 0,
        })
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Running Tauri application...");
    let data_queue = Arc::new(Mutex::new(DataQueue::default()));

    let mut camera_payload_rx: Vec<PayloadReceiver> = vec![];
    if let Some(camera_controller) = CameraController::init() {
        camera_payload_rx = camera_controller.camera_payload_rx.clone();
    }

    tauri::Builder::default()
        .manage(Mutex::new(CameraController::init()))
        .manage(data_queue.clone())
        .setup(|_app| {
            if !camera_payload_rx.is_empty() {
                // let camera_payload_rx = camera_controller.lock().unwrap().camera_payload_rx.clone();
                spawn(async move {
                    loop {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        println!("try to get latest frame");
                        let image_data_frame =
                            CameraController::get_latest_frame(&camera_payload_rx).await;

                        println!("get latest frame");
                        if !image_data_frame.is_empty() {
                            println!(
                                "data queue size: {}",
                                data_queue.lock().unwrap().queue.len()
                            );
                            data_queue
                                .lock()
                                .unwrap()
                                .add_image_data_frame(image_data_frame);
                        } else {
                            println!("but it's empty");
                        }
                    }
                });
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, fetch_camera_feed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // tauri::Builder::default()
    // .setup(|app| {
    //   #[cfg(debug_assertions)] // only include this code on debug builds
    //   {
    //     let window = app.get_webview_window("main").unwrap();
    //     window.open_devtools();
    //     window.close_devtools();
    //   }
    //   Ok(())
    // }).plugin(tauri_plugin_opener::init())
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
