use egui::{menu, Button, Context, TopBottomPanel, Visuals};

use crate::{app::AppModel, enums::ThemeMode, utils::thrd_select_file};

pub fn render_top_bar(app: &mut AppModel, ctx: &Context, frame: &mut eframe::Frame) {
    TopBottomPanel::top("top_bar")
        .exact_height(20.)
        .resizable(false)
        .show_separator_line(true)
        .show(ctx, |ui| {
            menu::bar(ui, |ui| {
                // ! File
                ui.menu_button("File", |ui| {
                    // ! Open file
                    if ui
                        .add(Button::new("Open file").shortcut_text({
                            if cfg!(target_os = "macos") {
                                "⌘O"
                            } else {
                                "Ctrl+O"
                            }
                        }))
                        .clicked()
                    {
                        thrd_select_file(app.tx.clone());
                        ui.close_menu();
                    }

                    // ! Close file
                    if ui
                        .add_enabled(
                            app.file_path.is_some(),
                            Button::new("Close file").shortcut_text({
                                if cfg!(target_os = "macos") {
                                    "⌘W"
                                } else {
                                    "Ctrl+W"
                                }
                            }),
                        )
                        .clicked()
                    {
                        app.file_path = None;
                        app.table_struct = None;
                        ui.close_menu();
                    }

                    ui.separator();

                    // ! Quit
                    if ui
                        .add(Button::new("Quit").shortcut_text({
                            if cfg!(target_os = "macos") {
                                "⌘Q"
                            } else {
                                "Ctrl+Q"
                            }
                        }))
                        .clicked()
                    {
                        frame.close();
                    }
                });

                // // ! Edit
                // ui.menu_button("Copy", |ui| {
                //     if ui.button("Copy").clicked() {
                //         ui.close_menu();
                //     }

                //     if ui.button("Paste").clicked() {
                //         ui.close_menu();
                //     }
                // });

                // ! View
                ui.menu_button("View", |ui| {
                    ui.menu_button("Zoom", |ui| {
                        if ui
                            .add(Button::new("Zoom in").shortcut_text({
                                if cfg!(target_os = "macos") {
                                    "⌘+"
                                } else {
                                    "Ctrl+"
                                }
                            }))
                            .clicked()
                        {
                            let current_pixels_per_point = ui.ctx().pixels_per_point();
                            ui.ctx()
                                .set_pixels_per_point(current_pixels_per_point + 0.1);
                        }

                        if ui
                            .add(Button::new("Zoom out").shortcut_text({
                                if cfg!(target_os = "macos") {
                                    "⌘-"
                                } else {
                                    "Ctrl-"
                                }
                            }))
                            .clicked()
                        {
                            let current_pixels_per_point = ui.ctx().pixels_per_point();
                            ui.ctx()
                                .set_pixels_per_point(current_pixels_per_point - 0.1);
                        }

                        if ui
                            .add(Button::new("Reset zoom").shortcut_text({
                                if cfg!(target_os = "macos") {
                                    "⌘0"
                                } else {
                                    "Ctrl+0"
                                }
                            }))
                            .clicked()
                        {
                            ui.ctx().set_pixels_per_point(1.);
                            ui.close_menu();
                        }
                    });
                });

                // ! Tools
                ui.menu_button("Tools", |ui| {
                    ui.menu_button("Theme", |ui| {
                        // LIGHT
                        if ui
                            .button({
                                if app.selected_theme == ThemeMode::Light {
                                    format!("{} Light", egui_phosphor::regular::CHECK)
                                } else {
                                    "Light".to_owned()
                                }
                            })
                            .clicked()
                        {
                            app.selected_theme = ThemeMode::Light;
                            ui.ctx().set_visuals(Visuals::light());
                            ui.close_menu();
                        }

                        // DARK
                        if ui
                            .button(if app.selected_theme == ThemeMode::Dark {
                                format!("{} Dark", egui_phosphor::regular::CHECK)
                            } else {
                                "Dark".to_owned()
                            })
                            .clicked()
                        {
                            app.selected_theme = ThemeMode::Dark;
                            ui.ctx().set_visuals(Visuals::dark());
                            ui.close_menu();
                        }

                        // FRAPPE
                        if ui
                            .button(if app.selected_theme == ThemeMode::Frappe {
                                format!("{} Frappe", egui_phosphor::regular::CHECK)
                            } else {
                                "Frappe".to_owned()
                            })
                            .clicked()
                        {
                            app.selected_theme = ThemeMode::Frappe;
                            catppuccin_egui::set_theme(ui.ctx(), catppuccin_egui::FRAPPE);
                            ui.close_menu();
                        }

                        // LATTE
                        if ui
                            .button(if app.selected_theme == ThemeMode::Latte {
                                format!("{} Latte", egui_phosphor::regular::CHECK)
                            } else {
                                "Latte".to_owned()
                            })
                            .clicked()
                        {
                            app.selected_theme = ThemeMode::Latte;
                            catppuccin_egui::set_theme(ui.ctx(), catppuccin_egui::LATTE);
                            ui.close_menu();
                        }

                        // MACCHIATO
                        if ui
                            .button(if app.selected_theme == ThemeMode::Macchiato {
                                format!("{} Macchiato", egui_phosphor::regular::CHECK)
                            } else {
                                "Macchiato".to_owned()
                            })
                            .clicked()
                        {
                            app.selected_theme = ThemeMode::Macchiato;
                            catppuccin_egui::set_theme(ui.ctx(), catppuccin_egui::MACCHIATO);
                            ui.close_menu();
                        }

                        // MOCHA
                        if ui
                            .button(if app.selected_theme == ThemeMode::Mocha {
                                format!("{} Mocha", egui_phosphor::regular::CHECK)
                            } else {
                                "Mocha".to_owned()
                            })
                            .clicked()
                        {
                            app.selected_theme = ThemeMode::Mocha;
                            catppuccin_egui::set_theme(ui.ctx(), catppuccin_egui::MOCHA);
                            ui.close_menu();
                        }
                    });
                });

                // ! Help
                ui.menu_button("Help", |ui| {
                    // ! About window
                    if ui.button("About").clicked() {
                        app.is_about_window_open = true;
                        ui.close_menu();
                    }
                });
            });
        });
}
