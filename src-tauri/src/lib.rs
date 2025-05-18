#![allow(warnings)]

// inputbot 全局快捷键 的库

mod commands;
mod localStore;

use log::{info, trace, warn};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{webview, AppHandle, Emitter, LogicalSize, Manager, Window, WindowEvent};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};
use tauri_plugin_updater::UpdaterExt;
use tokio::time::{sleep, Duration};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(
      tauri_plugin_log::Builder::new()
        .timezone_strategy(TimezoneStrategy::UseLocal)
        .targets([
          Target::new(TargetKind::Stdout),
          Target::new(TargetKind::LogDir { file_name: None }),
          Target::new(TargetKind::Webview),
        ])
        .build(),
    )
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    // .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
    //   // let _ = show_window(app);
    // }))
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      commands::greet,
      commands::openWin,
      commands::importSetting,
      commands::exportSetting,
      commands::saveSetting,
      commands::getSetting,
      commands::clearStore,
      commands::getHistoryOpenedUrls,
      commands::clearHistoryOpenedUrls,
      commands::killPort,
      commands::getProjectNamesTree,
      commands::openFolderEditor,
      commands::hideDirWindow,
      commands::setDirWindowSize,
      commands::page_loaded,
      commands::hideWindow,
      commands::CopyAndPaste,
      commands::updateQuickInputWindowSize,
      commands::hideQuickInputWindow,
      commands::get_package_info,
      commands::checkUpdateRust,
      commands::downloadAndInstall
    ])
    .setup(|app| {
      info!("App setup 启动");
      // let handle = app.handle().clone();
      // tauri::async_runtime::spawn(async move {
      //   update(handle).await.unwrap();
      // });

      let ww = app.get_webview_window("openFolder").unwrap();
      ww.eval(
        r#"
          document.addEventListener('keydown', evt => {
            if (evt.code === 'Escape') {
              window.__TAURI_INTERNALS__.invoke('hideWindow')
            }
          })
      "#,
      );

      //
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
            app.exit(0);
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
        .on_tray_icon_event(|tray, evt| match evt {
          TrayIconEvent::Click {
            position,
            rect,
            button: MouseButton::Right,
            button_state: MouseButtonState::Up,
            ..
          } => {
            dbg!(&position);
          }
          _ => {}
        })
        .build(app)?;

      {
        use tauri_plugin_global_shortcut::{
          Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
        };

        let alt_space_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
        let alt_v_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyV);

        app.handle().plugin(
          tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, shortcut, event| {
              if shortcut == &alt_space_shortcut {
                match event.state() {
                  ShortcutState::Pressed => {
                    let openFolderWindows = _app.get_webview_window("openFolder").unwrap();

                    let isVisible = openFolderWindows.is_visible().unwrap_or_default();
                    if isVisible {
                      if openFolderWindows.is_focused().expect("is_focused msg") {
                        openFolderWindows.hide();
                      } else {
                        openFolderWindows.set_focus();
                      }
                    } else {
                      openFolderWindows.show();
                      openFolderWindows.set_focus();
                    }
                  }
                  ShortcutState::Released => {
                    // println!("Ctrl-N Released!");
                  }
                }
              }
              if shortcut == &alt_v_shortcut {
                match event.state() {
                  ShortcutState::Pressed => {
                    let ww = _app.get_webview_window("quickInput").unwrap();

                    if ww.is_visible().unwrap_or(false) {
                      ww.hide();
                    } else {
                      let pos = ww.cursor_position().unwrap();
                      ww.set_position(pos);
                      ww.show();
                      ww.set_focus();
                    }
                  }
                  ShortcutState::Released => {
                    // println!("Ctrl-N Released!");
                  }
                }
              }
            })
            .build(),
        )?;

        app.global_shortcut().register(alt_space_shortcut);
        app.global_shortcut().register(alt_v_shortcut);
      }
      return Ok(());
    })
    .on_window_event(|window, evt| match evt {
      WindowEvent::CloseRequested { api, .. } => match window.label() {
        "setting" | "openFolder" | "quickInput" => {
          api.prevent_close();
          window.hide();
        }
        _ => {}
      },
      WindowEvent::Focused(focused) => {
        if (window.label() == "openFolder") {
          if (!focused) {
            // let closure = || println!("异步任务");
            // let hand = tokio::spawn(async move {
            //   sleep(Duration::from_millis(1000)).await;
            //   closure();
            // });

            // window.hide();
          }
        }

        let app = window.app_handle();
        app.emit_to("openFolder", "focusChanged", focused).unwrap();
      }
      _ => {}
    })
    .on_webview_event(|window, event| {
      dbg!(&event);
    })
    .on_page_load(|ww, payload| {
      dbg!(&payload.event());
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn crateTray() {}

fn show_window(app: &AppHandle) {
  let windows: std::collections::HashMap<String, tauri::WebviewWindow> = app.webview_windows();

  windows
    .values()
    .next()
    .expect("Sorry, no window found")
    .set_focus()
    .expect("Can't Bring Window to Focus");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    let mut downloaded = 0;

    info!("rust -> 下载并安装");
    // alternatively we could also call update.download() and update.install() separately
    update
      .download_and_install(
        |chunk_length, content_length| {
          downloaded += chunk_length;
          println!("downloaded {downloaded} from {content_length:?}");
        },
        || {
          println!("download finished");
          info!("rust -> download finished");
        },
      )
      .await?;

    println!("update installed");
    info!("rust -> update installed");
    app.restart();
  }

  Ok(())
}
