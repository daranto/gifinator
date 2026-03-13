import { fetch } from "@tauri-apps/plugin-http";
import type { GifData } from "../types";

const BASE_URL = "https://api.giphy.com/v1/gifs";

interface GiphyImage {
  url: string;
  width: string;
  height: string;
  mp4?: string;
}

interface GiphyItem {
  id: string;
  title: string;
  url: string;
  images: {
    original: GiphyImage;
    fixed_width: GiphyImage;
    fixed_width_small: GiphyImage;
  };
}

function mapGiphyItem(item: GiphyItem): GifData {
  const preview = item.images.fixed_width;
  return {
    id: item.id,
    title: item.title,
    url: item.url,
    gifUrl: item.images.original.url,
    previewUrl: preview.url,
    mp4Url: preview.mp4 ?? "",
    width: parseInt(preview.width, 10),
    height: parseInt(preview.height, 10),
    copiedAt: 0,
  };
}

export async function searchGifs(
  query: string,
  apiKey: string,
  limit = 25,
  offset = 0,
): Promise<GifData[]> {
  const params = new URLSearchParams({
    api_key: apiKey,
    q: query,
    limit: String(limit),
    offset: String(offset),
    rating: "g",
  });
  const res = await fetch(`${BASE_URL}/search?${params}`);
  if (!res.ok) throw new Error(`Giphy API error: ${res.status}`);
  const json = await res.json();
  return (json.data as GiphyItem[]).map(mapGiphyItem);
}

export async function trendingGifs(
  apiKey: string,
  limit = 25,
): Promise<GifData[]> {
  const params = new URLSearchParams({
    api_key: apiKey,
    limit: String(limit),
    rating: "g",
  });
  const res = await fetch(`${BASE_URL}/trending?${params}`);
  if (!res.ok) throw new Error(`Giphy API error: ${res.status}`);
  const json = await res.json();
  return (json.data as GiphyItem[]).map(mapGiphyItem);
}
