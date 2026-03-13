import { invoke } from "@tauri-apps/api/core";

export async function updateHotkey(
  oldHotkey: string,
  newHotkey: string,
): Promise<void> {
  await invoke("update_hotkey", {
    oldHotkey,
    newHotkey,
  });
}
