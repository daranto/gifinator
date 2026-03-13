<script lang="ts">
  import type { GifData } from "../types";
  import { copyGifToClipboard } from "../services/clipboard";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  let { gif, onCopied }: { gif: GifData; onCopied?: () => void } = $props();

  let copied = $state(false);

  async function handleClick() {
    await copyGifToClipboard(gif);
    copied = true;

    if (onCopied) onCopied();

    setTimeout(async () => {
      const appWindow = getCurrentWebviewWindow();
      await appWindow.hide();
    }, 150);
  }
</script>

<button class="gif-item" onclick={handleClick} title={gif.title || "GIF kopieren"}>
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
  {#if copied}
    <div class="copied-overlay">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="3">
        <path d="M20 6L9 17l-5-5"/>
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

  .gif-media {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .copied-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.5);
    animation: fadeIn 0.15s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
