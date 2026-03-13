<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { listen } from "@tauri-apps/api/event";
  import TitleBar from "./lib/components/TitleBar.svelte";
  import SearchBar from "./lib/components/SearchBar.svelte";
  import GifGrid from "./lib/components/GifGrid.svelte";
  import Settings from "./lib/components/Settings.svelte";
  import { searchGifs, trendingGifs } from "./lib/services/giphy";
  import { getSettings, getRecentGifs } from "./lib/services/store";
  import type { GifData } from "./lib/types";

  let view: "overlay" | "settings" = $state("overlay");
  let gifs: GifData[] = $state([]);
  let loading = $state(false);
  let error = $state("");
  let searchBarComponent: SearchBar | undefined = $state();
  let noApiKey = $state(false);

  const appWindow = getCurrentWebviewWindow();

  onMount(async () => {
    // Hotkey is registered by Rust on startup — no JS registration needed

    // Load initial GIFs
    await loadRecentOrTrending();

    // Auto-hide on blur
    appWindow.onFocusChanged(({ payload: focused }) => {
      if (!focused && view === "overlay") {
        appWindow.hide();
      }
    });

    // Listen for show-settings event from tray
    listen("show-settings", () => {
      view = "settings";
    });

    // Listen for show-overlay event from tray
    listen("show-overlay", async () => {
      view = "overlay";
      await loadRecentOrTrending();
    });

    // Focus search on window show
    listen("tauri://focus", () => {
      if (view === "overlay") {
        setTimeout(() => {
          searchBarComponent?.clear();
          searchBarComponent?.focus();
        }, 50);
      }
    });

    // Escape to hide
    document.addEventListener("keydown", (e) => {
      if (e.key === "Escape") {
        appWindow.hide();
      }
    });
  });

  async function loadRecentOrTrending() {
    error = "";
    const recent = await getRecentGifs();
    if (recent.length > 0) {
      gifs = recent;
      return;
    }

    // No recent GIFs — try trending
    const settings = await getSettings();
    if (!settings.giphyApiKey) {
      noApiKey = true;
      gifs = [];
      return;
    }

    noApiKey = false;
    loading = true;
    try {
      gifs = await trendingGifs(settings.giphyApiKey);
    } catch (e) {
      error = `Fehler beim Laden: ${e}`;
    } finally {
      loading = false;
    }
  }

  async function handleSearch(query: string) {
    const settings = await getSettings();
    if (!settings.giphyApiKey) {
      noApiKey = true;
      return;
    }

    noApiKey = false;
    error = "";
    loading = true;
    try {
      gifs = await searchGifs(query, settings.giphyApiKey);
    } catch (e) {
      error = `Suche fehlgeschlagen: ${e}`;
      gifs = [];
    } finally {
      loading = false;
    }
  }

  function handleClear() {
    loadRecentOrTrending();
  }

  function handleCopied() {
    // GIF was copied — overlay will auto-hide via GifItem
  }
</script>

<div class="container">
  <TitleBar onSettings={view === "overlay" ? () => { view = "settings"; } : undefined} />

  {#if view === "settings"}
    <Settings onBack={() => { view = "overlay"; loadRecentOrTrending(); }} />
  {:else}
    <SearchBar
      bind:this={searchBarComponent}
      onSearch={handleSearch}
      onClear={handleClear}
    />

    {#if noApiKey}
      <div class="no-api-key">
        <div class="no-api-key-icon">🔑</div>
        <p><strong>Kein Giphy API Key</strong></p>
        <p>Öffne die Einstellungen und gib deinen Giphy API Key ein.</p>
        <p class="hint">
          Kostenlos erhältlich auf
          <a href="https://developers.giphy.com/" target="_blank" rel="noopener">developers.giphy.com</a>
        </p>
        <button class="settings-link" onclick={() => { view = "settings"; }}>
          Einstellungen öffnen
        </button>
      </div>
    {:else}
      <GifGrid {gifs} {loading} {error} onCopied={handleCopied} />
    {/if}
  {/if}
</div>

<style>
  .container {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg);
    border-radius: var(--radius);
    border: 1px solid var(--border);
    overflow: hidden;
  }

  .no-api-key {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 40px 20px;
    text-align: center;
    flex: 1;
    color: var(--text-secondary);
    font-size: 14px;
  }

  .no-api-key-icon {
    font-size: 32px;
    margin-bottom: 8px;
  }

  .no-api-key strong {
    color: var(--text);
  }

  .hint {
    font-size: 12px;
  }

  .hint a {
    color: var(--accent);
    text-decoration: none;
  }

  .hint a:hover {
    text-decoration: underline;
  }

  .settings-link {
    margin-top: 8px;
    padding: 8px 20px;
    border: none;
    border-radius: var(--radius-sm);
    background: var(--accent);
    color: white;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
  }

  .settings-link:hover {
    background: var(--accent-hover);
  }
</style>
