import {
  register,
  unregister,
  isRegistered,
} from "@tauri-apps/plugin-global-shortcut";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

let currentHotkey: string | null = null;

export async function registerHotkey(shortcut: string): Promise<void> {
  const appWindow = getCurrentWebviewWindow();

  if (currentHotkey) {
    try {
      if (await isRegistered(currentHotkey)) {
        await unregister(currentHotkey);
      }
    } catch {
      // Ignore errors when unregistering old hotkey
    }
  }

  await register(shortcut, async (event) => {
    if (event.state === "Pressed") {
      const visible = await appWindow.isVisible();
      if (visible) {
        await appWindow.hide();
      } else {
        await appWindow.center();
        await appWindow.show();
        await appWindow.setFocus();
      }
    }
  });

  currentHotkey = shortcut;
}

export async function unregisterHotkey(): Promise<void> {
  if (currentHotkey) {
    try {
      if (await isRegistered(currentHotkey)) {
        await unregister(currentHotkey);
      }
    } catch {
      // Ignore
    }
    currentHotkey = null;
  }
}
