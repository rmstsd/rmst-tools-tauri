// #![allow(warnings)]

// inputbot 全局快捷键 的库

mod commands;

use log::{info, trace, warn};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{webview, AppHandle, Emitter, LogicalSize, Manager, Window, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};
use tauri_plugin_updater::UpdaterExt;
// use tokio::time::{sleep, Duration};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let builder = tauri::Builder::default();

  builder
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_autostart::init(
      MacosLauncher::LaunchAgent,
      Some(vec![]),
    ))
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
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
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
      commands::downloadAndInstall,
      commands::saveCommands,
      commands::getCommands,
      commands::execCommand,
      commands::check_update
    ])
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
    .setup(|app| {
      // #[cfg(desktop)]
      // app
      //   .handle()
      //   .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
      //     let ww = app.get_webview_window("setting");
      //     if let Some(ww) = ww {
      //       ww.show();
      //       ww.set_focus();
      //     }
      //   }));

      info!("App setup 启动了");
      // let handle = app.handle().clone();
      // tauri::async_runtime::spawn(async move {
      //   update(handle).await.unwrap();
      // });

      // 获取自动启动管理器
      let autostart_manager = app.autolaunch();
      let isEnabled = autostart_manager.is_enabled();

      dbg!(&isEnabled);
      if (isEnabled.unwrap_or_default()) {
        // autostart_manager.disable();
      } else {
        autostart_manager.enable();
      }

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
      let restart = MenuItem::with_id(app, "restart", "重启", true, None::<&str>)?;
      let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
      let separator = &PredefinedMenuItem::separator(app).unwrap();
      let menu = Menu::with_items(app, &[&m2, separator, &restart, &quit_i])?;
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
          "restart" => {
            let app_handle = app.app_handle();
            tauri::process::restart(&app_handle.env());
            // app.restart();
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
        let alt_r_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyR);

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
              if shortcut == &alt_r_shortcut {
                dbg!(&"alt + r");
                match event.state() {
                  ShortcutState::Pressed => {
                    let ww = _app.get_webview_window("setting").unwrap();
                    ww.unminimize();
                    ww.show();
                    ww.set_focus();

                    let clipboard_text = _app.clipboard().read_text().unwrap_or_default();
                    let text = clipboard_text.trim().to_string();

                    _app.emit_to("setting", "showQrCode", text);
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
        app.global_shortcut().register(alt_r_shortcut);
      }
      return Ok(());
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
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
