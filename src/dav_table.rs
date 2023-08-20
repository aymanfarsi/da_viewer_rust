use egui::{RichText, ScrollArea, Ui};
use egui_extras::{Column, TableBuilder};

use crate::models::TableStruct;

pub fn render_table(ui: &mut Ui, table_struct: &TableStruct) {
    let font_size = 13.;
    let max_size_column = table_struct
        .columns
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    ScrollArea::new([true; 2])
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(false)
                .resizable(true)
                .columns(
                    {
                        if max_size_column == 0 {
                            Column::auto().resizable(true).clip(true)
                        } else {
                            Column::initial(max_size_column as f32 * font_size)
                                .resizable(true)
                                .clip(true)
                        }
                    },
                    table_struct.columns.len(),
                )
                .header(30., |mut header| {
                    for column in table_struct.columns.iter() {
                        header.col(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    RichText::new({
                                        if column == "null" || column == "None" || column.is_empty()
                                        {
                                            "---"
                                        } else {
                                            column
                                        }
                                    })
                                    .size(font_size + 7.)
                                    .strong(),
                                );
                            });
                        });
                    }
                })
                .body(|mut body| {
                    body.ui_mut().separator();
                    for i in 0..table_struct.rows.len() {
                        body.row(37.5, |mut row| {
                            for j in 0..table_struct.columns.len() {
                                row.col(|ui| {
                                    ui.add_space(7.5);

                                    let text: String = table_struct.rows[i][j]
                                        .replace('\"', "")
                                        .trim()
                                        .to_string();

                                    if text == "null" || text == "None" || text.is_empty() {
                                        ui.centered_and_justified(|ui| {
                                            ui.label("---");
                                        });
                                    } else {
                                        ui.horizontal_centered(|ui| {
                                            ui.label(RichText::new(text).size(font_size));
                                        });
                                    }
                                });
                            }
                        });
                        body.ui_mut().separator();
                    }
                });
        });
}
