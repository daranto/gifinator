import { invoke } from "@tauri-apps/api/core";
import type { GifData } from "../types";
import { addRecentGif } from "./store";

export async function copyGifToClipboard(gif: GifData): Promise<void> {
  await invoke("copy_gif_to_clipboard", {
    url: gif.gifUrl,
    id: gif.id,
  });
  await addRecentGif(gif);
}
