#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::Manager;

use crate::{core::Core, packages::load_packages};
use crate::packages::keyboard::key_event::KeyEvent;
use futures::executor::block_on;

mod core;
mod packages;

#[tokio::main]
async fn main() {
    let mut core = Core::new();

    load_packages(&mut core);

    core.start_engines().await;

    tokio::spawn(async move {
        let key_pressed_node = core.create_node(
            "keyboard",
            "key_a",
        ).unwrap();

        let midi_node = core.create_node(
            "midi",
            "first_output",
        ).unwrap();

        let print_node = core.create_node(
            "utils",
            "print"
        ).unwrap();

        core.connect_io(
            midi_node.id,
            "output",
            print_node.id,
            "value",
        ).unwrap();
        core.connect_io(
            key_pressed_node.id,
            "execute",
            midi_node.id,
            "execute",
        ).unwrap();
        core.connect_io(
            midi_node.id,
            "execute",
            print_node.id,
            "execute",
        ).unwrap();

        core.start().await;
    });

    tauri::Builder::default()
        .setup(|app| {
            app.listen_global("core", |event| {
                println!("{}", event.payload().unwrap());
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
