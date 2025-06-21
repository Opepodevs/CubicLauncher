<!-- Titlebar.svelte -->
<script lang="ts">
    import { appStore } from "@stores/launcher";
    import { currentTheme } from "@stores/theme";
    import Logo from "@assets/Logo.svelte";
    import "../../css/titlebar.scss";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import {
        hideLauncher,
        closeLauncher,
        maximizeLauncher,
    } from "../libraries/backend";

    function startDrag() {
        getCurrentWindow().startDragging();
    }

    // Prevent dragging when clicking controls
    function stopPropagation(event: Event) {
        event.stopPropagation();
    }

    const background = $derived($currentTheme?.background);
    const currentInstance = $derived($appStore.currentInstance);
</script>

<div
    class="titlebar bg-[{background}]"
    onmousedown={startDrag}
    role="button"
    tabindex="-1"
>
    <!-- Left spacer for balance -->
    <div class="spacer"></div>
    <!-- Center content -->
    <div class="center-content">
        <Logo width="1.5rem" height="1.5rem" />
        {#if currentInstance}
            <span class="title-text">{`${currentInstance.name} - Cubic`}</span>
        {:else}
            <span class="title-text">Cubic</span>
        {/if}
    </div>
    <!-- Window controls -->
    <div
        class="window-controls"
        onmousedown={stopPropagation}
        role="button"
        tabindex="-1"
    >
        <button
            onclick={hideLauncher}
            class="control-button"
            aria-label="Minimize"
            onmousedown={stopPropagation}
        >
            <svg width="12" height="12" viewBox="0 0 12 12">
                <path
                    d="M3 6h6"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                />
            </svg>
        </button>
        <button
            onclick={maximizeLauncher}
            class="control-button"
            aria-label="Maximize"
            onmousedown={stopPropagation}
        >
            <svg width="12" height="12" viewBox="0 0 12 12">
                <rect
                    x="3"
                    y="3"
                    width="6"
                    height="6"
                    stroke="currentColor"
                    stroke-width="1.5"
                    fill="none"
                    rx="1"
                />
            </svg>
        </button>
        <button
            onclick={closeLauncher}
            class="control-button close-button"
            aria-label="Close"
            onmousedown={stopPropagation}
        >
            <svg width="12" height="12" viewBox="0 0 12 12">
                <path
                    d="M3 3l6 6M9 3l-6 6"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                />
            </svg>
        </button>
    </div>
</div>
