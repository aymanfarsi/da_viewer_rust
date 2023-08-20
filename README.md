# Discord Watcher

## Project description

`Data Analysis Viewer (DaV)` is a Rust application that uses [egui](https://github.com/emilk/egui) for the UI and [polars](https://github.com/pola-rs/polars) to read different file formats (uses [calamine](https://github.com/tafia/calamine) for Excel files).

## Features

- Minimal application (low resources used)
- Builds to a single executable
- Read Excel, CSV, Parquet, and more coming soon
- Simple table to view the data from a file (cannot modify the data)

## Dependencies

The only dependencies are rust and cargo. For the crates, it needs the following:

- `serenity`: 0.11.6 (features: client, gateway, rustls_backend, model, cache)
- `tokio`: 1.28.2 (features: sync, rt, rt-multi-thread)
- `rfd`: 0.11.4
- `directories`: 5.0.1
- `egui`, `eframe`, & `egui_extra`: 0.22.0
- `egui-phosphor`: 0.2.0
- `polars`: 0.31.0
- `calamine`: 0.21.2
- `catppuccin-egui`: 3.0.0

## Build & Run

1. Clone the repo
2. Navigate to the project root
3. run `cargo build -r` (release mode)
4. You will find your executable in diractory `$crate/target/release/`


