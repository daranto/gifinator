import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import type { GifData } from "../types";
import { addRecentGif } from "./store";

export async function copyGifToClipboard(gif: GifData): Promise<void> {
  await writeText(gif.gifUrl);
  await addRecentGif(gif);
}
