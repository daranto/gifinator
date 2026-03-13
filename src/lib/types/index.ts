export interface GifData {
  id: string;
  title: string;
  url: string;
  gifUrl: string;
  previewUrl: string;
  mp4Url: string;
  width: number;
  height: number;
  copiedAt: number;
}

export interface AppSettings {
  giphyApiKey: string;
  hotkey: string;
}

export const DEFAULT_SETTINGS: AppSettings = {
  giphyApiKey: "",
  hotkey: "CmdOrCtrl+Shift+G",
};
