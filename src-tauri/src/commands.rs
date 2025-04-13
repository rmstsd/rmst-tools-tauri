use std::fs;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
  println!("{name:?}");
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(async)]
pub fn openWin(app: AppHandle) {
  println!("open win");

  let webview_window = tauri::WebviewWindowBuilder::new(
    &app,
    "label",
    tauri::WebviewUrl::App("https://www.bilibili.com/".into()),
  )
  .inner_size(800.0, 600.0)
  .build()
  .unwrap();
}

#[tauri::command]
pub fn importSetting(app: AppHandle) -> Result<String, String> {
  let file_path = app.dialog().file().blocking_pick_file();
  dbg!(&file_path);

  match file_path {
    Some(path) => {
      let path: String = path.to_string();
      println!("{path:#?}");

      let content = fs::read_to_string(path).expect("Unable to read file");
      Ok(content)
    }
    None => Err("".to_string()),
  }
}

#[tauri::command]
pub fn saveSetting(settingData: String) {
  println!("{settingData:#?}");
}
