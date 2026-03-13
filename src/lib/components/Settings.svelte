<script lang="ts">
  import { getSettings, saveSettings } from "../services/store";
  import { registerHotkey } from "../services/hotkey";
  import type { AppSettings } from "../types";
  import { DEFAULT_SETTINGS } from "../types";
  import { onMount } from "svelte";

  let { onBack }: { onBack: () => void } = $props();

  let apiKey = $state("");
  let hotkey = $state(DEFAULT_SETTINGS.hotkey);
  let recording = $state(false);
  let saved = $state(false);
  let error = $state("");
  let showApiKey = $state(false);

  onMount(async () => {
    const settings = await getSettings();
    apiKey = settings.giphyApiKey;
    hotkey = settings.hotkey;
  });

  function formatHotkey(shortcut: string): string {
    return shortcut
      .replace("Control", "Ctrl")
      .replace("Key", "")
      .replace("Digit", "")
      .replace(/\+/g, " + ");
  }

  function handleKeyRecord(e: KeyboardEvent) {
    e.preventDefault();
    e.stopPropagation();

    if (e.key === "Escape") {
      recording = false;
      return;
    }

    // Ignore standalone modifier keys
    if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) return;

    const parts: string[] = [];
    if (e.ctrlKey) parts.push("Control");
    if (e.shiftKey) parts.push("Shift");
    if (e.altKey) parts.push("Alt");
    if (e.metaKey) parts.push("Super");

    // Need at least one modifier
    if (parts.length === 0) return;

    parts.push(e.code);
    hotkey = parts.join("+");
    recording = false;
  }

  async function handleSave() {
    error = "";
    saved = false;

    try {
      const settings: AppSettings = {
        giphyApiKey: apiKey.trim(),
        hotkey,
      };
      await saveSettings(settings);
      await registerHotkey(hotkey);
      saved = true;
      setTimeout(() => { saved = false; }, 2000);
    } catch (e) {
      error = `Fehler beim Speichern: ${e}`;
    }
  }
</script>

<div class="settings">
  <div class="settings-header">
    <button class="back-btn" onclick={onBack}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M19 12H5M12 19l-7-7 7-7"/>
      </svg>
      Zurück
    </button>
    <h2>Einstellungen</h2>
  </div>

  <div class="settings-body">
    <div class="field">
      <label for="apikey">Giphy API Key</label>
      <p class="field-hint">
        Kostenlos erhältlich auf
        <a href="https://developers.giphy.com/" target="_blank" rel="noopener">developers.giphy.com</a>
      </p>
      <div class="input-row">
        <input
          id="apikey"
          type={showApiKey ? "text" : "password"}
          bind:value={apiKey}
          placeholder="API Key eingeben..."
          class="input"
        />
        <button
          class="toggle-btn"
          onclick={() => { showApiKey = !showApiKey; }}
          title={showApiKey ? "Verbergen" : "Anzeigen"}
        >
          {showApiKey ? "🙈" : "👁"}
        </button>
      </div>
    </div>

    <div class="field">
      <!-- svelte-ignore a11y_label_has_associated_control -->
      <label>Hotkey</label>
      <p class="field-hint">Tastenkombination zum Öffnen des Overlays</p>
      {#if recording}
        <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="hotkey-recorder"
          tabindex="0"
          role="textbox"
          onkeydown={handleKeyRecord}
          onblur={() => { recording = false; }}
        >
          Drücke eine Tastenkombination... (Escape zum Abbrechen)
        </div>
      {:else}
        <button class="hotkey-display" onclick={() => { recording = true; }}>
          {formatHotkey(hotkey)}
          <span class="change-hint">Klicken zum Ändern</span>
        </button>
      {/if}
    </div>

    {#if error}
      <div class="message error">{error}</div>
    {/if}

    <button class="save-btn" onclick={handleSave} class:saved>
      {saved ? "Gespeichert!" : "Speichern"}
    </button>
  </div>
</div>

<style>
  .settings {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 0 12px 12px;
  }

  .settings-header {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 0 16px;
  }

  .settings-header h2 {
    font-size: 16px;
    font-weight: 600;
  }

  .back-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    border: none;
    background: transparent;
    color: var(--accent);
    font-size: 13px;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 6px;
  }

  .back-btn:hover {
    background: var(--bg-hover);
  }

  .settings-body {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field label {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
  }

  .field-hint {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .field-hint a {
    color: var(--accent);
    text-decoration: none;
  }

  .field-hint a:hover {
    text-decoration: underline;
  }

  .input-row {
    display: flex;
    gap: 6px;
  }

  .input {
    flex: 1;
    height: 36px;
    padding: 0 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg-secondary);
    color: var(--text);
    font-size: 13px;
    outline: none;
  }

  .input:focus {
    border-color: var(--accent);
  }

  .toggle-btn {
    width: 36px;
    height: 36px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg-secondary);
    cursor: pointer;
    font-size: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .toggle-btn:hover {
    background: var(--bg-hover);
  }

  .hotkey-display {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 36px;
    padding: 0 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg-secondary);
    color: var(--text);
    font-size: 13px;
    font-family: monospace;
    cursor: pointer;
  }

  .hotkey-display:hover {
    border-color: var(--accent);
  }

  .change-hint {
    font-size: 11px;
    color: var(--text-secondary);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }

  .hotkey-recorder {
    height: 36px;
    padding: 0 12px;
    border: 2px solid var(--accent);
    border-radius: var(--radius-sm);
    background: var(--bg-secondary);
    color: var(--accent);
    font-size: 13px;
    display: flex;
    align-items: center;
    outline: none;
    animation: pulse 1.5s infinite;
  }

  @keyframes pulse {
    0%, 100% { border-color: var(--accent); }
    50% { border-color: var(--accent-hover); }
  }

  .save-btn {
    height: 40px;
    border: none;
    border-radius: var(--radius-sm);
    background: var(--accent);
    color: white;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s;
  }

  .save-btn:hover {
    background: var(--accent-hover);
  }

  .save-btn.saved {
    background: #38a169;
  }

  .message.error {
    color: #e53e3e;
    font-size: 13px;
    padding: 8px 12px;
    background: rgba(229, 62, 62, 0.1);
    border-radius: var(--radius-sm);
  }
</style>
