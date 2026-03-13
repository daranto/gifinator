<script lang="ts">
  import type { GifData } from "../types";
  import { copyGifToClipboard } from "../services/clipboard";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  let { gif, onCopied }: { gif: GifData; onCopied?: () => void } = $props();

  let state: "idle" | "loading" | "copied" | "error" = $state("idle");

  async function handleClick() {
    if (state === "loading") return;
    state = "loading";

    try {
      await copyGifToClipboard(gif);
      state = "copied";
      if (onCopied) onCopied();

      setTimeout(async () => {
        const appWindow = getCurrentWebviewWindow();
        await appWindow.hide();
      }, 300);
    } catch (e) {
      console.error("GIF copy failed:", e);
      state = "error";
      setTimeout(() => { state = "idle"; }, 2000);
    }
  }
</script>

<button class="gif-item" onclick={handleClick} title={gif.title || "GIF kopieren"} disabled={state === "loading"}>
  {#if gif.mp4Url}
    <video
      src={gif.mp4Url}
      autoplay
      loop
      muted
      playsinline
      class="gif-media"
    ></video>
  {:else}
    <img src={gif.previewUrl} alt={gif.title} class="gif-media" loading="lazy" />
  {/if}
  {#if state === "loading"}
    <div class="overlay loading-overlay">
      <div class="spinner"></div>
    </div>
  {:else if state === "copied"}
    <div class="overlay copied-overlay">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="3">
        <path d="M20 6L9 17l-5-5"/>
      </svg>
    </div>
  {:else if state === "error"}
    <div class="overlay error-overlay">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="3">
        <path d="M18 6L6 18M6 6l12 12"/>
      </svg>
    </div>
  {/if}
</button>

<style>
  .gif-item {
    position: relative;
    border: none;
    padding: 0;
    background: var(--bg-secondary);
    border-radius: var(--radius-sm);
    overflow: hidden;
    cursor: pointer;
    aspect-ratio: 1;
    transition: transform 0.15s, box-shadow 0.15s;
  }

  .gif-item:hover {
    transform: scale(1.05);
    box-shadow: 0 4px 12px var(--shadow);
    z-index: 1;
  }

  .gif-item:disabled {
    cursor: wait;
  }

  .gif-media {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: fadeIn 0.15s ease;
  }

  .loading-overlay {
    background: rgba(0, 0, 0, 0.4);
  }

  .copied-overlay {
    background: rgba(0, 0, 0, 0.5);
  }

  .error-overlay {
    background: rgba(229, 62, 62, 0.6);
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 3px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
