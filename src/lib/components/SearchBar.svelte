<script lang="ts">
  let {
    onSearch,
    onClear,
  }: {
    onSearch: (query: string) => void;
    onClear: () => void;
  } = $props();

  let query = $state("");
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let inputEl: HTMLInputElement | undefined = $state();

  function handleInput() {
    if (debounceTimer) clearTimeout(debounceTimer);

    if (query.trim() === "") {
      onClear();
      return;
    }

    debounceTimer = setTimeout(() => {
      if (query.trim()) {
        onSearch(query.trim());
      }
    }, 300);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      if (query) {
        query = "";
        onClear();
      }
    }
  }

  export function focus() {
    inputEl?.focus();
  }

  export function clear() {
    query = "";
  }
</script>

<div class="search-container">
  <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
    <circle cx="11" cy="11" r="8"/>
    <path d="M21 21l-4.35-4.35"/>
  </svg>
  <input
    bind:this={inputEl}
    bind:value={query}
    oninput={handleInput}
    onkeydown={handleKeydown}
    type="text"
    placeholder="GIFs suchen..."
    class="search-input"
    spellcheck="false"
  />
  {#if query}
    <button class="clear-btn" onclick={() => { query = ""; onClear(); }} aria-label="Suche löschen">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M18 6L6 18M6 6l12 12"/>
      </svg>
    </button>
  {/if}
</div>

<style>
  .search-container {
    position: relative;
    display: flex;
    align-items: center;
    padding: 0 12px;
    flex-shrink: 0;
  }

  .search-icon {
    position: absolute;
    left: 24px;
    color: var(--text-secondary);
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    height: 40px;
    padding: 0 36px 0 40px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg-secondary);
    color: var(--text);
    font-size: 14px;
    outline: none;
    transition: border-color 0.15s;
  }

  .search-input::placeholder {
    color: var(--text-secondary);
  }

  .search-input:focus {
    border-color: var(--accent);
  }

  .clear-btn {
    position: absolute;
    right: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border: none;
    border-radius: 4px;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .clear-btn:hover {
    background: var(--bg-hover);
    color: var(--text);
  }
</style>
