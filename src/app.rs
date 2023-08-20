use egui::{vec2, Align2, Button, CentralPanel, Color32, Id, LayerId, Order, RichText, TextStyle};
use std::sync::mpsc::{self, Receiver, Sender};

use crate::{
    about_window::AboutWindow,
    dav_table::render_table,
    enums::{ChannelMessage, ThemeMode},
    models::TableStruct,
    top_bar::render_top_bar,
    utils::{handle_keyboard_shortcuts, thrd_read_file, thrd_select_file},
};

#[derive(Debug)]
pub struct AppModel {
    pub file_path: Option<String>,
    pub table_struct: Option<TableStruct>,

    pub selected_theme: ThemeMode,

    about_window: Box<AboutWindow>,
    pub is_about_window_open: bool,

    error: Option<String>,

    pub tx: Sender<ChannelMessage>,
    rx: Receiver<ChannelMessage>,
}

impl AppModel {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = mpsc::channel::<ChannelMessage>();

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

        cc.egui_ctx.set_fonts(fonts);

        AppModel {
            file_path: None,
            table_struct: None,

            selected_theme: ThemeMode::Dark,

            about_window: Box::<AboutWindow>::default(),
            is_about_window_open: false,

            error: None,

            tx,
            rx,
        }
    }
}

impl eframe::App for AppModel {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // * Handle windows
        if self.is_about_window_open {
            self.about_window.show(ctx, &mut self.is_about_window_open);
        }

        // * Top bar
        render_top_bar(self, ctx, frame);

        // * Central panel
        CentralPanel::default().show(ctx, |ui| {
            // ! Handle shortcuts
            handle_keyboard_shortcuts(ui, self, frame);

            // ! Check for messages from other threads
            if let Ok(rx_type) = self.rx.try_recv() {
                match rx_type {
                    ChannelMessage::OpenFile(file_path) => {
                        self.file_path = Some(file_path.clone());
                        thrd_read_file(self.tx.clone(), file_path);
                    }
                    ChannelMessage::ReadFile(table_struct) => {
                        self.table_struct = Some(table_struct);
                    }
                    ChannelMessage::ReadFileError(err) => {
                        self.error = Some(err);
                    }
                }
            }

            // ! Error message
            if let Some(err) = &self.error {
                ui.centered_and_justified(|ui| {
                    ui.label(err);
                });
                return;
            }

            if self.file_path.is_none() {
                // ! Drop/Open file
                let response = ui
                    .allocate_ui(ui.available_size_before_wrap(), |ui| {
                        ui.centered_and_justified(|ui| {
                            let open_btn = Button::new(
                                RichText::new(
                                    "Double click anywhere to open a file\n\t\t\t\tor drop it here",
                                )
                                .color(Color32::from_rgb(255, 255, 255))
                                .text_style(TextStyle::Button),
                            )
                            .frame(false);

                            let btn_response = ui.add_sized(vec2(200., 50.), open_btn);

                            if btn_response.double_clicked() {
                                thrd_select_file(self.tx.clone());
                            }

                            // if btn_response.hovered() {
                            //     ui.output_mut(|opt| opt.cursor_icon = CursorIcon::PointingHand);
                            // }
                        });
                    })
                    .response;

                // ui.painter()
                //     .rect_stroke(response.rect, 7.5, (1.0, ui.visuals().text_color()));

                // ! Context & Rect
                let ctx = ui.ctx();
                let screen_rect = response.rect;

                // ! Preview hovering files:
                if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
                    let painter = ctx.layer_painter(LayerId::new(
                        Order::Foreground,
                        Id::new("file_drop_target"),
                    ));
                    painter.rect_filled(
                        screen_rect,
                        7.5,
                        Color32::from_rgba_premultiplied(0, 0, 0, 128),
                    );
                    painter.text(
                        screen_rect.center(),
                        Align2::CENTER_CENTER,
                        "Drop it here",
                        TextStyle::Heading.resolve(&ctx.style()),
                        ui.visuals().text_color(),
                    );
                }

                // ! Collect dropped files:
                ctx.input(|i| {
                    if !i.raw.dropped_files.is_empty() {
                        if let Some(pos) = i.pointer.hover_pos() {
                            if screen_rect.contains(pos) {
                                let first_path = i.raw.dropped_files[0]
                                    .clone()
                                    .path
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .replace('\\', "/");
                                self.file_path = Some(first_path.clone());
                                thrd_read_file(self.tx.clone(), first_path);
                            }
                        }
                    }
                });
            }

            if let Some(table_struct) = &self.table_struct {
                // ! Render table
                ui.group(|ui| {
                    render_table(ui, table_struct);
                });
            }
        });
    }
}
