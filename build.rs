extern crate winresource;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/dav.ico");
        res.compile().unwrap();
    }

    if cfg!(target_os = "macos") {
        eprintln!("build.rs for macOS is not implemented yet!");
    }

    if cfg!(target_os = "linux") {
        eprintln!("build.rs for Linux is not implemented yet!");
    }

    if cfg!(target_arch = "wasm32") {
        std::env::set_var("RUSTFLAGS", "'--cfg=web_sys_unstable_apis'");
    }
}
