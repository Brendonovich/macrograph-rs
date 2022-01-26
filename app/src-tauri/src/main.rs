#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

use macrograph_core::api::{Request, Response};
use macrograph_core::core::CoreController;
use macrograph_core::Core;
use macrograph_packages::register_packages;

#[tauri::command]
async fn core_request(
  req: Request,
  core_controller: tauri::State<'_, CoreController>,
) -> Result<Response, ()> {
  Ok(core_controller.send(req).await)
}

#[tokio::main]
async fn main() {
  let mut core = Core::new();

  register_packages(&mut core);

  core.start_engines().await;

  let controller = core.get_controller();

  tokio::spawn(async move {
    core.start().await;
  });

  tauri::Builder::default()
    .manage(controller)
    .invoke_handler(tauri::generate_handler![core_request])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
