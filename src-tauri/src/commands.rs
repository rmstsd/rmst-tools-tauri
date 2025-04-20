use serde_json::json;
use serde_json::Value;
use std::fs;
use std::vec;
use tauri::AppHandle;
use tauri::Wry;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::StoreExt;

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
pub fn importSetting(app: AppHandle) {
  let file_path = app.dialog().file().blocking_pick_file();

  match file_path {
    Some(path) => {
      let path: String = path.to_string();

      let content = fs::read_to_string(path).expect("Unable to read file");

      saveSetting(app, content.clone());
    }
    None => {}
  }
}

#[tauri::command]
pub fn exportSetting(app: AppHandle) {
  dbg!("exportSetting");
  let file_path = app
    .dialog()
    .file()
    .set_file_name("cfg-2.json")
    .add_filter("My Filter", &["json"])
    .blocking_save_file()
    .unwrap();

  println!("{file_path:#?}");
  dbg!(file_path.to_string());

  let store = app.store("store.json").unwrap();
  let ans = store.get("setting");

  fs::write(file_path.to_string(), "aa").expect("Unable to read file");
}

#[tauri::command]
pub fn saveSetting(app: AppHandle, settingData: String) {
  let store = app.store("store.json").unwrap();

  dbg!("saveSetting", &settingData);

  store.set("setting", settingData);
}

#[tauri::command]
pub fn getSetting(app: AppHandle) -> Value {
  let store = app.store("store.json").unwrap();
  let val = store.get("setting");

  match val {
    Some(val) => {
      dbg!(&val);
      let t = val.to_string();
      dbg!(&t);

      val
    }
    None => Value::Null,
  }
}

struct SettingData {
  cmdPath: String,
  editorPaths: Vec<String>,
  projectPaths: Vec<String>,
  notes: Vec<String>,
}
