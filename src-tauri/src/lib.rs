mod tray;

#[tauri::command]
async fn copy_gif_to_clipboard(url: String, id: String) -> Result<(), String> {
    // Download GIF
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Download fehlgeschlagen: {}", e))?;
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Lesen fehlgeschlagen: {}", e))?;

    // Save to temp directory
    let temp_dir = std::env::temp_dir().join("gifinator");
    std::fs::create_dir_all(&temp_dir).map_err(|e| format!("Temp-Ordner: {}", e))?;
    let file_path = temp_dir.join(format!("{}.gif", id));
    std::fs::write(&file_path, &bytes).map_err(|e| format!("Schreiben: {}", e))?;

    // Copy file to clipboard (platform-specific)
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
        let script = format!(
            r#"set the clipboard to POSIX file "{}""#,
            path_str
        );
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
        .invoke_handler(tauri::generate_handler![copy_gif_to_clipboard])
        .setup(|app| {
            tray::setup_tray(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
