import { LazyStore } from "@tauri-apps/plugin-store";
import { DEFAULT_SETTINGS, type AppSettings, type GifData } from "../types";

const settingsStore = new LazyStore("settings.json");
const historyStore = new LazyStore("history.json");

const MAX_HISTORY = 50;

export async function getSettings(): Promise<AppSettings> {
  const apiKey =
    (await settingsStore.get<string>("giphyApiKey")) ??
    DEFAULT_SETTINGS.giphyApiKey;
  const hotkey =
    (await settingsStore.get<string>("hotkey")) ?? DEFAULT_SETTINGS.hotkey;
  return { giphyApiKey: apiKey, hotkey };
}

export async function saveSettings(settings: AppSettings): Promise<void> {
  await settingsStore.set("giphyApiKey", settings.giphyApiKey);
  await settingsStore.set("hotkey", settings.hotkey);
  await settingsStore.save();
}

export async function getRecentGifs(): Promise<GifData[]> {
  const gifs = (await historyStore.get<GifData[]>("recentGifs")) ?? [];
  return gifs.sort((a, b) => b.copiedAt - a.copiedAt);
}

export async function addRecentGif(gif: GifData): Promise<void> {
  let gifs = (await historyStore.get<GifData[]>("recentGifs")) ?? [];
  gifs = gifs.filter((g) => g.id !== gif.id);
  gifs.unshift({ ...gif, copiedAt: Date.now() });
  if (gifs.length > MAX_HISTORY) {
    gifs = gifs.slice(0, MAX_HISTORY);
  }
  await historyStore.set("recentGifs", gifs);
  await historyStore.save();
}
