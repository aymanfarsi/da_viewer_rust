#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::AppModel;

mod about_window;
mod app;
mod dav_table;
mod enums;
mod models;
mod read_file;
mod top_bar;
mod utils;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use eframe::{HardwareAcceleration, IconData, NativeOptions, Theme};
    use egui::vec2;
    use tokio::runtime::Runtime;

    env_logger::init();

    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();

    eframe::run_native(
        "DA Viewer",
        NativeOptions {
            active: true,
            centered: true,
            decorated: true,
            resizable: true,
            transparent: true,
            default_theme: Theme::Dark,
            follow_system_theme: false,
            drag_and_drop_support: true,
            min_window_size: Some(vec2(222., 123.)),
            initial_window_size: Some(vec2(850., 590.)),
            hardware_acceleration: HardwareAcceleration::Preferred,
            icon_data: Some(
                IconData::try_from_png_bytes(&include_bytes!("../assets/dav.png")[..]).unwrap(),
            ),
            ..Default::default()
        },
        Box::new(|cc| Box::new(AppModel::new(cc))),
    )
}

// #[cfg(target_arch = "wasm32")]
// fn main() {
//     eframe::WebLogger::init(log::LevelFilter::Debug).ok();

//     let web_options = eframe::WebOptions::default();

//     wasm_bindgen_futures::spawn_local(async {
//         eframe::WebRunner::new()
//             .start(
//                 "the_canvas_id", // hardcode it
//                 web_options,
//                 Box::new(|cc| Box::new(AppModel::new(cc))),
//             )
//             .await
//             .expect("failed to start eframe");
//     });
// }
