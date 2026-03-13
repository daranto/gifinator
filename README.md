# Gifinator

Portable Windows GIF-Picker mit Giphy-Integration. Per Hotkey ein Overlay aufrufen, GIFs suchen und direkt in die Zwischenablage kopieren.

## Features

- **Globaler Hotkey** (Standard: `Ctrl+Shift+G`) zum Öffnen des Overlays
- **Giphy-Suche** mit Echtzeit-Ergebnissen
- **GIF kopieren** als Datei in die Zwischenablage — funktioniert in Teams, Slack, Discord etc.
- **Verlauf** der zuletzt kopierten GIFs
- **System Tray** mit Menü (Öffnen, Einstellungen, Beenden)
- **Light/Dark Theme** folgt automatisch der Windows-Einstellung
- **Portable** — keine Installation nötig, einzelne `.exe`

## Download

Die aktuelle Version als portable `.exe` gibt es unter [Releases](https://github.com/daranto/gifinator/releases).

> Voraussetzung: Windows 10 (1803+) oder Windows 11 (WebView2 ist vorinstalliert).

## Einrichtung

1. `Gifinator.exe` herunterladen und starten
2. Rechtsklick auf das Tray-Icon → **Einstellungen**
3. Giphy API Key eingeben (kostenlos auf [developers.giphy.com](https://developers.giphy.com/))
4. Hotkey anpassen (optional)
5. `Ctrl+Shift+G` drücken und loslegen

## Entwicklung

### Voraussetzungen

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)

### Lokal starten

```bash
npm install
npm run tauri dev
```

### Bauen

```bash
npm run tauri build --no-bundle
```

Die `.exe` liegt dann unter `src-tauri/target/release/Gifinator.exe`.

## Tech-Stack

- **Backend:** [Tauri v2](https://v2.tauri.app/) (Rust)
- **Frontend:** [Svelte 5](https://svelte.dev/) + [Vite](https://vite.dev/)
- **API:** [Giphy API](https://developers.giphy.com/)

## Lizenz

[MIT](LICENSE)
