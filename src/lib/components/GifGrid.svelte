<script lang="ts">
  import type { GifData } from "../types";
  import GifItem from "./GifItem.svelte";

  let {
    gifs,
    loading = false,
    error = "",
    onCopied,
  }: {
    gifs: GifData[];
    loading?: boolean;
    error?: string;
    onCopied?: () => void;
  } = $props();
</script>

<div class="grid-container">
  {#if loading}
    <div class="state-message">
      <div class="spinner"></div>
      <span>Suche...</span>
    </div>
  {:else if error}
    <div class="state-message error">
      <span>{error}</span>
    </div>
  {:else if gifs.length === 0}
    <div class="state-message">
      <span>Keine GIFs gefunden</span>
    </div>
  {:else}
    <div class="grid">
      {#each gifs as gif (gif.id)}
        <GifItem {gif} {onCopied} />
      {/each}
    </div>
  {/if}

  <div class="attribution">
    Powered by GIPHY
  </div>
</div>

<style>
  .grid-container {
    flex: 1;
    overflow-y: auto;
    padding: 8px 12px;
    display: flex;
    flex-direction: column;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(110px, 1fr));
    gap: 8px;
    flex: 1;
  }

  .state-message {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 40px;
    color: var(--text-secondary);
    font-size: 14px;
    flex: 1;
  }

  .error {
    color: #e53e3e;
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 3px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .attribution {
    text-align: center;
    padding: 8px;
    font-size: 11px;
    color: var(--text-secondary);
    flex-shrink: 0;
  }
</style>
