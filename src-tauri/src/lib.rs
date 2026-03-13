mod tray;

use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

const DEFAULT_HOTKEY: &str = "CmdOrCtrl+Shift+G";

struct HotkeyState(Mutex<String>);

fn toggle_overlay(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("overlay") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.center();
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

fn register_shortcut(app: &AppHandle, shortcut: &str) -> Result<(), String> {
    let parsed = shortcut
        .parse::<tauri_plugin_global_shortcut::Shortcut>()
        .map_err(|e| format!("Ungültiger Hotkey '{}': {}", shortcut, e))?;

    // Unregister if already registered
    let _ = app.global_shortcut().unregister(parsed);

    let app_handle = app.clone();
    app.global_shortcut()
        .on_shortcut(parsed, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                toggle_overlay(&app_handle);
            }
        })
        .map_err(|e| format!("Registrierung fehlgeschlagen: {}", e))?;

    Ok(())
}

fn read_hotkey_from_store(app: &AppHandle) -> String {
    let data_dir = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir());
    let store_path = data_dir.join("settings.json");

    if let Ok(content) = std::fs::read_to_string(&store_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(hotkey) = json.get("hotkey").and_then(|v| v.as_str()) {
                if !hotkey.is_empty() {
                    return hotkey.to_string();
                }
            }
        }
    }

    DEFAULT_HOTKEY.to_string()
}

#[tauri::command]
async fn update_hotkey(
    app: AppHandle,
    old_hotkey: String,
    new_hotkey: String,
) -> Result<(), String> {
    // Unregister old shortcut
    if !old_hotkey.is_empty() {
        if let Ok(parsed) = old_hotkey.parse::<tauri_plugin_global_shortcut::Shortcut>() {
            let _ = app.global_shortcut().unregister(parsed);
        }
    }

    // Register new shortcut
    register_shortcut(&app, &new_hotkey)?;

    // Update state
    if let Some(state) = app.try_state::<HotkeyState>() {
        *state.0.lock().unwrap() = new_hotkey;
    }

    Ok(())
}

#[tauri::command]
async fn copy_gif_to_clipboard(url: String, id: String) -> Result<(), String> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Download fehlgeschlagen: {}", e))?;
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Lesen fehlgeschlagen: {}", e))?;

    let temp_dir = std::env::temp_dir().join("gifinator");
    std::fs::create_dir_all(&temp_dir).map_err(|e| format!("Temp-Ordner: {}", e))?;
    let file_path = temp_dir.join(format!("{}.gif", id));
    std::fs::write(&file_path, &bytes).map_err(|e| format!("Schreiben: {}", e))?;

    clipboard::copy_file_to_clipboard(&file_path)
}

mod clipboard {
    use std::path::Path;

    #[cfg(target_os = "windows")]
    pub fn copy_file_to_clipboard(path: &Path) -> Result<(), String> {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let path_str = path.to_str().ok_or("Ungültiger Pfad")?;
        let script = format!(
            r#"Add-Type -Assembly System.Windows.Forms; $f = [System.Collections.Specialized.StringCollection]::new(); $f.Add('{}'); [System.Windows.Forms.Clipboard]::SetFileDropList($f)"#,
            path_str
        );
        let output = std::process::Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .output()
            .map_err(|e| format!("PowerShell: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "Clipboard-Fehler: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    #[cfg(target_os = "macos")]
    pub fn copy_file_to_clipboard(path: &Path) -> Result<(), String> {
        let path_str = path.to_str().ok_or("Ungültiger Pfad")?;
        let script = format!(r#"set the clipboard to POSIX file "{}""#, path_str);
        let output = std::process::Command::new("osascript")
            .args(["-e", &script])
            .output()
            .map_err(|e| format!("osascript: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "Clipboard-Fehler: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    pub fn copy_file_to_clipboard(_path: &Path) -> Result<(), String> {
        Err("Datei-Clipboard wird auf dieser Plattform nicht unterstützt".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(HotkeyState(Mutex::new(DEFAULT_HOTKEY.to_string())))
        .invoke_handler(tauri::generate_handler![
            copy_gif_to_clipboard,
            update_hotkey
        ])
        .setup(|app| {
            // Setup system tray
            tray::setup_tray(app.handle())?;

            // Register global shortcut from stored settings
            let hotkey = read_hotkey_from_store(app.handle());
            if let Err(e) = register_shortcut(app.handle(), &hotkey) {
                eprintln!("Hotkey-Registrierung fehlgeschlagen: {}", e);
                // Fallback to default
                if hotkey != DEFAULT_HOTKEY {
                    let _ = register_shortcut(app.handle(), DEFAULT_HOTKEY);
                }
            }
            *app.state::<HotkeyState>().0.lock().unwrap() = hotkey;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
