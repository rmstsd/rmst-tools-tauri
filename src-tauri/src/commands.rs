use rand::random;
use serde::de::value;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_reader;
use serde_json::from_value;
use serde_json::json;
use serde_json::to_string;
use serde_json::to_value;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::vec;
use tauri::AppHandle;
use tauri::Manager;
use tauri::WebviewWindow;
use tauri::Wry;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::StoreExt;
use urlencoding::encode;

static Store_Key: &str = "store.json";

static Setting_Key: &str = "setting";
static HistoryOpenedUrls_Key: &str = "historyOpenedUrls";

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
  println!("{name:?}");
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn importSetting(app: AppHandle) {
  let file_path = app.dialog().file().blocking_pick_file();

  match file_path {
    Some(path) => {
      let path: String = path.to_string();

      println!("{path:#?}");

      // let content = fs::read_to_string(path).expect("Unable to read file");
      // let data: SettingData = serde_json::from_str(content.as_str()).unwrap();
      // dbg!(&data);

      let opened = fs::File::open(path);

      match opened {
        Ok(val) => {
          // json 里的 数据和 结构体不一致会导致 error
          let data: Result<SettingData, serde_json::Error> = serde_json::from_reader(val);

          match data {
            Ok(val) => {
              dbg!(&val);
              saveSetting(app, val);
            }
            Err(err) => {
              dbg!(&err);
            }
          }
        }
        Err(err) => {
          dbg!(&err);
        }
      }

      // let content: SettingData = serde_json::from_reader(fs::File::open(path).unwrap()).unwrap();
      // saveSetting(app, content);
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

  let store = app.store(Store_Key).unwrap();
  let ans = store.get(Setting_Key);

  fs::write(file_path.to_string(), "aa").expect("Unable to read file");
}

#[tauri::command]
pub fn saveSetting(app: AppHandle, settingData: SettingData) {
  let store = app.store(Store_Key).unwrap();

  dbg!("saveSetting", &settingData);

  let val = to_value(settingData);

  match val {
    Ok(value) => {
      dbg!(&value);

      store.set(Setting_Key, value);
    }
    Err(err) => {
      dbg!(&err);
    }
  }
}

#[tauri::command]
pub fn getSetting(app: AppHandle) -> Value {
  let store = app.store(Store_Key).unwrap();
  let val = store.get(Setting_Key);

  match val {
    Some(val) => val,
    None => Value::Null,
  }
}

#[tauri::command]
pub async fn clearStore(app: AppHandle) -> Result<(), String> {
  let store = app.store(Store_Key).unwrap();
  store.delete(Setting_Key);

  Ok(())
}

#[tauri::command(async)]
pub fn openWin(app: AppHandle, url: String) {
  dbg!(&url);

  let label: i32 = random();
  let label = label.to_string();

  let webview_window =
    tauri::WebviewWindowBuilder::new(&app, label, tauri::WebviewUrl::App(url.clone().into()))
      .inner_size(1000.0, 700.0)
      .build();

  match webview_window {
    Ok(val) => {
      let store = app.store(Store_Key).unwrap();
      let val = store.get(HistoryOpenedUrls_Key).unwrap_or(json!([])); // 了解区别

      let mut list = from_value::<Vec<String>>(val).unwrap();
      dbg!(&list);

      if (!list.contains(&url)) {
        list.insert(0, url);

        if (list.len() > 5) {
          list.pop();
        }

        store.set(HistoryOpenedUrls_Key, list);
      }
    }
    Err(err) => {
      dbg!(&err);
    }
  }
}

#[tauri::command]
pub fn getHistoryOpenedUrls(app: AppHandle) -> Value {
  let store = app.store(Store_Key).unwrap();
  let val = store.get(HistoryOpenedUrls_Key);

  match val {
    Some(val) => val,
    None => {
      let emp = serde_json::from_value(json!([])).unwrap();

      emp
    }
  }
}

#[tauri::command]
pub async fn clearHistoryOpenedUrls(app: AppHandle) -> Result<(), String> {
  let store = app.store(Store_Key).unwrap();
  store.delete(HistoryOpenedUrls_Key);
  Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingData {
  cmdPath: Option<String>,
  editorPaths: Option<Vec<String>>,
  projectPaths: Option<Vec<String>>,
  notes: Option<Vec<String>>,
}
