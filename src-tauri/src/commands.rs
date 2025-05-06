use port_killer::kill;
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
use std::fs::metadata;
use std::fs::read_dir;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::vec;
use tauri::AppHandle;
use tauri::Listener;
use tauri::LogicalSize;
use tauri::Manager;
use tauri::Size;
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
    Ok(ww) => {
      ww.once("created", |evt| {
        dbg!(&345);
      });

      let store = app.store(Store_Key).unwrap();
      let listVal = store.get(HistoryOpenedUrls_Key).unwrap_or(json!([]));

      let mut list = from_value::<Vec<String>>(listVal).unwrap();
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

#[tauri::command]
pub fn killPort(port: u16) -> Result<bool, bool> {
  dbg!(&port);
  let r = kill_process_by_port(port);
  match r {
    Ok(()) => Ok(true),
    Err(err) => {
      dbg!(&err);

      Err(false)
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingData {
  cmdPath: Option<String>,
  editorPaths: Option<Vec<String>>,
  projectPaths: Option<Vec<String>>,
  notes: Option<Vec<String>>,
}

use std::process::Command;
fn kill_process_by_port(port: u16) -> Result<(), std::io::Error> {
  #[cfg(target_os = "windows")]
  {
    // 在 Windows 上查找占用指定端口的进程 ID
    let output = Command::new("cmd")
      .args(&["/C", &format!("netstat -ano | findstr :{}", port)])
      .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() >= 5 {
        let pid_str = parts[4];
        if let Ok(pid) = pid_str.parse::<u32>() {
          // 杀死找到的进程
          Command::new("taskkill")
            .args(&["/F", "/PID", &pid.to_string()])
            .output()?;
        }
      }
    }
  }
  #[cfg(target_os = "linux")]
  {
    // 在 Linux 上查找占用指定端口的进程 ID
    let output = Command::new("sh")
      .arg("-c")
      .arg(&format!("lsof -t -i:{}", port))
      .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
      if let Ok(pid) = line.parse::<u32>() {
        // 杀死找到的进程
        Command::new("kill")
          .args(&["-9", &pid.to_string()])
          .output()?;
      }
    }
  }
  #[cfg(target_os = "macos")]
  {
    // 在 macOS 上查找占用指定端口的进程 ID
    let output = Command::new("sh")
      .arg("-c")
      .arg(&format!("lsof -t -i:{}", port))
      .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
      if let Ok(pid) = line.parse::<u32>() {
        // 杀死找到的进程
        Command::new("kill")
          .args(&["-9", &pid.to_string()])
          .output()?;
      }
    }
  }
  Ok(())
}

#[tauri::command]
pub fn getProjectNamesTree(app: AppHandle) -> Value {
  let blackList = vec!["$RECYCLE.BIN", "System Volume Information"];
  let blackStartWithChar = vec!["_", "$", ".", "-"];

  let val: Value = (getSetting(app));

  let settingData: SettingData = from_value(val).unwrap();
  dbg!(&settingData.projectPaths);

  let projectPaths: Vec<String> = settingData.projectPaths.unwrap();

  let nv: Vec<NamesTree> = projectPaths
    .into_iter()
    .filter(|item| {
      if let Ok(md) = metadata(Path::new(item)) {
        md.is_dir()
      } else {
        false
      }
    })
    .map(|item: String| {
      let name = item.replace(r"\", "/");
      let path = Path::new(item.as_str());

      let children = read_dir(path)
        .expect("msg")
        .into_iter()
        .map(|item| item.unwrap().file_name().to_string_lossy().to_string())
        .filter(|item| {
          !blackList.contains(&item.as_str())
            && !blackStartWithChar.iter().any(|char| item.starts_with(char))
        })
        .collect();

      let nt: NamesTree = NamesTree { name, children };

      return nt;
    })
    .collect();

  to_value(nv).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
struct NamesTree {
  name: String,
  children: Vec<String>,
}

#[tauri::command]
pub async fn openFolderEditor(
  app: tauri::AppHandle,
  projectPath: String,
  editorPath: String,
) -> Result<(), String> {
  dbg!(&projectPath);
  dbg!(&editorPath);

  // 执行外部命令示例
  let output = Command::new(editorPath)
    .arg(projectPath)
    .output()
    .expect("Failed to execute command");

  if output.status.success() {
    println!("打开成功");
  } else {
    println!("打开失败");
  }
  Ok(())
}

#[tauri::command]
pub async fn hideDirWindow(app: tauri::AppHandle, window: tauri::Window) -> Result<(), String> {
  let win = app
    .get_webview_window("openFolder")
    .expect("经济技术电饭锅");
  win.hide();
  Ok(())
}

#[tauri::command]
pub async fn setDirWindowSize(app: tauri::AppHandle, height: f64) -> Result<(), String> {
  let win = app
    .get_webview_window("openFolder")
    .expect("对方过后发过火");

  win.set_size(Size::Logical(LogicalSize {
    width: 800.0,
    height,
  }));
  Ok(())
}
