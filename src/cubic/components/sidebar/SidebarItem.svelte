<script lang="ts">
    import { type Component } from "svelte";
    import SidebarTooltip from "./SidebarTooltip.svelte";
    export let name: string | undefined = undefined;
    export let icon: Component | string;
    export let onClick: (() => void) | undefined = undefined;
    let tooltipShow = false;
    let tooltipX = 0;
    let tooltipY = 0;
    const showTooltip = (event: MouseEvent) => {
        const rect = (
            event.currentTarget as HTMLElement
        ).getBoundingClientRect();
        tooltipX = rect.right + 8;
        tooltipY = rect.top + rect.height / 2;
        tooltipShow = true;
    };
    const hideTooltip = () => {
        tooltipShow = false;
    };
    // Helper to check if it's a string (image URL)
    const isImageUrl = (icon: Component | string): icon is string => {
        return typeof icon === "string";
    };
    const handleClick = () => {
        if (onClick) {
            onClick();
        }
    };
    const handleKeydown = (event: KeyboardEvent) => {
        if (event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            handleClick();
        }
    };
</script>

<div
    class="sidebar-item"
    on:mouseenter={showTooltip}
    on:mouseleave={hideTooltip}
    on:click={handleClick}
    on:keydown={handleKeydown}
    role="button"
    tabindex="0"
>
    <!-- Render image if string -->
    {#if isImageUrl(icon)}
        <img src={icon} alt="Icon" class="icon" />
    {:else}
        <!-- Render component if not string -->
        <svelte:component this={icon} class="icon" />
    {/if}

    <!-- Tooltip -->
    {#if tooltipShow}
        <div
            class="fixed z-50 pointer-events-none"
            style="left: {tooltipX}px; top: {tooltipY}px; transform: translateY(-50%);"
        >
            <SidebarTooltip text={name ?? "Instance"} />
        </div>
    {/if}
</div>

<style>
    .sidebar-item {
        background: var(--color-background);
        padding: 0.25rem;
        border-radius: 0.75rem;
        border: 1px solid var(--color-border-default);
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        flex-shrink: 0;
        z-index: 60;
    }

    .sidebar-item:hover {
        background: var(--color-accent-base);
        border-color: var(--color-accent-hover);
        transform: translateY(-1px);
    }

    .sidebar-item:active {
        transform: translateY(0);
        background: var(--color-accent-active);
    }

    .icon {
        width: 2.5rem;
        height: 2.5rem;
        border-radius: 0.75rem;
    }
</style>
