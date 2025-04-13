#![allow(warnings)]

mod commands;

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{webview, AppHandle, LogicalSize, Manager, WindowEvent};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
      let _ = show_window(app);
    }))
    .plugin(
      tauri_plugin_global_shortcut::Builder::new()
        .with_shortcuts(["ctrl+space"])
        .unwrap()
        .with_handler(|app, shortcut, event| {
          if event.state == ShortcutState::Pressed {
            if shortcut.matches(Modifiers::CONTROL, Code::Space) {
              let openFolderWindows = app.get_webview_window("openFolder").unwrap();

              if openFolderWindows.is_visible().unwrap() {
                openFolderWindows.hide();
              } else {
                openFolderWindows.show();
                openFolderWindows.set_focus();
              }
            }
          }
        })
        .build(),
    )
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![commands::greet])
    .invoke_handler(tauri::generate_handler![commands::openWin])
    .invoke_handler(tauri::generate_handler![commands::importSetting])
    .invoke_handler(tauri::generate_handler![commands::saveSetting])
    .setup(|app| {
      let m2 = MenuItem::with_id(app, "setting", "设置", true, None::<&str>)?;
      let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
      let separator = &PredefinedMenuItem::separator(app).unwrap();

      let menu = Menu::with_items(app, &[&m2, separator, &quit_i])?;
      let tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| match event.id.as_ref() {
          "quit" => {
            println!("click quit")
          }
          "setting" => {
            let settingWindow: tauri::WebviewWindow = app.get_webview_window("setting").unwrap();

            if settingWindow.is_minimized().unwrap_or(false) {
              settingWindow.unminimize();
            }

            settingWindow.show();
            settingWindow.set_focus();
          }
          _ => {
            println!("未匹配 {:?}", event.id)
          }
        })
        // .on_tray_icon_event(|tray, evt| match evt {
        //   TrayIconEvent::Click {
        //     button: MouseButton::Left,
        //     button_state: MouseButtonState::Up,
        //   } => {
        //     println!("left click pressed and released");
        //   }
        //   _ => {
        //     println!("{:?}", evt)
        //   }
        // })
        .build(app)?;
      return Ok(());
    })
    .on_window_event(|window, evt| match evt {
      WindowEvent::CloseRequested { api, .. } => match window.label() {
        "setting" | "openFolder" | "quickInput" => {
          api.prevent_close();
          window.hide().unwrap();
        }
        _ => {}
      },
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn crateTray() {}

fn show_window(app: &AppHandle) {
  let windows = app.webview_windows();

  windows
    .values()
    .next()
    .expect("Sorry, no window found")
    .set_focus()
    .expect("Can't Bring Window to Focus");
}
