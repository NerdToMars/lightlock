// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::thread;
use std::time::Duration;
use tauri::Manager;
use tokio::io::AsyncReadExt;
use tauri::Emitter;

// use tauri::Manager; // for window.emit


// We'll use opencv to capture frames
use opencv::{
    core::Vector,
    imgcodecs,
    prelude::*,
    videoio::{self, VideoCapture},
};

use tauri::{AppHandle, ipc::Channel};
use serde::Serialize;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref CAMERA: Mutex<Option<VideoCapture>> = Mutex::new(None);
}

lazy_static! {
    static ref FRAME_ID: Mutex<usize> = Mutex::new(0);
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
  Finished {
    download_id: usize,
  },
}

#[tauri::command]
fn fetch_camera_feed(app: AppHandle, on_event: Channel<DownloadEvent>) {
    let mut camera = CAMERA.lock().unwrap();
    if camera.is_none() {
        *camera = Some(VideoCapture::new(0, videoio::CAP_ANY).unwrap());
    }

    let mut frame_id = FRAME_ID.lock().unwrap();
    *frame_id += 1;
    
    // include jpg file in the resources
    println!("Fetching camera feed...");
    // println!("Message from Rust: {}", reader.id());

    // let jpg_file = include_bytes!("test.jpg");
    let mut frame = Mat::default();
    if let Ok(_) = camera.as_mut().unwrap().read(&mut frame) {
        // Encode frame as JPEG
        let mut buf = Vector::new();
        let params = Vector::new();
        imgcodecs::imencode(".jpg", &frame, &mut buf, &params).unwrap();
        
        // Convert to base64
        let base64 = STANDARD.encode(&buf.to_vec());
        
        on_event.send(DownloadEvent::Started {
            image_data: &base64,
            download_id: *frame_id,
            content_length: 0,
        }).unwrap();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Running Tauri application...");

    tauri::Builder::default()
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
