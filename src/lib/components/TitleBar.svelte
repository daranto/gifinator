<script lang="ts">
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  let { onSettings }: { onSettings?: () => void } = $props();

  const appWindow = getCurrentWebviewWindow();

  function handleClose() {
    appWindow.hide();
  }
</script>

<div class="titlebar" data-tauri-drag-region>
  <span class="title" data-tauri-drag-region>Gifinator</span>
  <div class="actions">
    {#if onSettings}
      <button class="titlebar-btn settings-btn" onclick={onSettings} title="Einstellungen">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"/>
          <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
        </svg>
      </button>
    {/if}
    <button class="titlebar-btn close-btn" onclick={handleClose} title="Schließen">
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <path d="M18 6L6 18M6 6l12 12"/>
      </svg>
    </button>
  </div>
</div>

<style>
  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 36px;
    padding: 0 8px 0 14px;
    user-select: none;
    -webkit-user-select: none;
    flex-shrink: 0;
  }

  .title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    pointer-events: none;
  }

  .actions {
    display: flex;
    gap: 4px;
  }

  .titlebar-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s;
  }

  .titlebar-btn:hover {
    background: var(--bg-hover);
    color: var(--text);
  }

  .close-btn:hover {
    background: #e53e3e;
    color: white;
  }
</style>
