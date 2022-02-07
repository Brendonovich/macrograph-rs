#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::env;
use std::path::Path;

use tauri::Manager;

use macrograph::api::{Request, Response};
use macrograph::core::{Core, CoreController};

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

  let lib_extension = match env::consts::OS {
    "macos" => "dylib",
    "windows" => "dll",
    _ => "so",
  };

  for package in ["logic", "utils", "keyboard", "obs"] {
    let mut current_exe_path = env::current_exe().unwrap();
    current_exe_path.pop();
    let path = Path::new(&current_exe_path)
      .join(format!("libmg_pkg_{}.{}", package, lib_extension))
      .canonicalize()
      .unwrap();

    core.load_library(&path.to_str().unwrap());
  }

  core.setup();

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
