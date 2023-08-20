use std::{path::Path, sync::mpsc::Sender};

use egui::{Key, Ui};
use rfd::AsyncFileDialog;

use crate::{app::AppModel, enums::ChannelMessage, read_file::load_data_from_file};

pub fn handle_keyboard_shortcuts(ui: &Ui, app: &mut AppModel, frame: &mut eframe::Frame) {
    let ctx = ui.ctx();

    // ! Open file shortcut
    if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::O)) {
        thrd_select_file(app.tx.clone());
    }

    // ! Close file shortcut
    if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::W)) {
        app.file_path = None;
        app.table_struct = None;
    }

    // ! Quit shortcut
    if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::Q)) {
        frame.close();
    }

    // ! Zoom in shortcut
    if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::PlusEquals)) {
        let current_pixels_per_point = ui.ctx().pixels_per_point();
        ui.ctx()
            .set_pixels_per_point(current_pixels_per_point + 0.1);
    }

    // ! Zoom out shortcut
    if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::Minus)) {
        let current_pixels_per_point = ui.ctx().pixels_per_point();
        ui.ctx()
            .set_pixels_per_point(current_pixels_per_point - 0.1);
    }

    // ! Reset zoom shortcut
    if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::Num0)) {
        ui.ctx().set_pixels_per_point(1.);
    }
}

pub fn thrd_select_file(tx: Sender<ChannelMessage>) {
    println!("[*] Selecting file...");
    tokio::spawn(async move {
        let user_dirs = directories::UserDirs::new().unwrap();
        let desktop_dir: &Path = user_dirs.desktop_dir().unwrap();

        let res = AsyncFileDialog::new()
            .add_filter("All", &["csv", "parquet", "xlsx"])
            .add_filter("csv", &["csv"])
            .add_filter("parquet", &["parquet"])
            // .add_filter("json", &["json"])
            .add_filter("xlsx", &["xlsx"])
            .set_directory(desktop_dir)
            .pick_file()
            .await;

        if let Some(file_path) = res {
            let path = file_path
                .path()
                .to_str()
                .unwrap()
                .to_owned()
                .replace('\\', "/");
            tx.send(ChannelMessage::OpenFile(path.clone())).unwrap();

            // ! Read file
            read_file(tx, path);
        }
    });
}

pub fn thrd_read_file(tx: Sender<ChannelMessage>, file_path: String) {
    println!("[*] Reading file at {}...", file_path.clone());
    tokio::spawn(async move {
        read_file(tx, file_path);
    });
}

fn read_file(tx: Sender<ChannelMessage>, file_path: String) {
    let table_struct = load_data_from_file(file_path);
    match table_struct {
        Ok(table_struct) => {
            tx.send(ChannelMessage::ReadFile(table_struct)).unwrap();
        }
        Err(err) => {
            tx.send(ChannelMessage::ReadFileError(err)).unwrap();
        }
    }
}
