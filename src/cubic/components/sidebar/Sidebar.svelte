<script lang="ts">
    import SidebarItem from "./SidebarItem.svelte";
    import SidebarTooltip from "./SidebarTooltip.svelte";
    import Logo from "@assets/Logo.svelte";
    import settings from "@assets/icons/UI/settings.svelte";
    import { appStore, instances } from "../../stores/launcher";
    import { isDarkTheme } from "../../stores/theme";

    $: items = $instances.map((instance) => {
        return {
            name: instance.name,
            icon: instance.icon,
            onClick: () => appStore.setCurrentInstance(instance),
        };
    });

    // Tooltip state
    let tooltipShow = false;
    let tooltipX = 0;
    let tooltipY = 0;
    let tooltipText = "";

    const showTooltip = (event: MouseEvent, text: string) => {
        const rect = (
            event.currentTarget as HTMLElement
        ).getBoundingClientRect();
        tooltipX = rect.right + 8;
        tooltipY = rect.top + rect.height / 2;
        tooltipText = text;
        tooltipShow = true;
    };

    const hideTooltip = () => {
        tooltipShow = false;
    };

    function handleLogoClick() {
        appStore.setCurrentInstance(null);
    }

    function handleSettingsClick() {
        // Nota: toggleSettingsModel no existe en el store proporcionado
        // Necesitarás agregarlo al store o cambiar esta funcionalidad
        console.log(
            "Settings clicked - implement toggleSettingsModel in store",
        );
    }
</script>

<div class="sidebar" class:dark={$isDarkTheme}>
    <div
        class="logo-container"
        on:click={handleLogoClick}
        on:mouseenter={(e) => showTooltip(e, "Home")}
        on:mouseleave={hideTooltip}
        role="button"
        tabindex="0"
        on:keydown={(e) => e.key === "Enter" && handleLogoClick()}
    >
        <Logo width="2rem" height="2rem" />
    </div>

    <div class="instances-container">
        <div class="instances-scroll">
            {#each items as item (item.name)}
                <SidebarItem Icon={item.icon} onClick={item.onClick} />
            {/each}
        </div>
    </div>

    <div class="settings-container">
        <div class="separator"></div>
        <div
            on:click={handleSettingsClick}
            on:mouseenter={(e) => showTooltip(e, "Settings")}
            on:mouseleave={hideTooltip}
            role="button"
            tabindex="0"
            on:keydown={(e) => e.key === "Enter" && handleSettingsClick()}
        >
            <SidebarItem
                Icon={settings}
                onClick={() => {
                    console.log("xd");
                }}
            />
        </div>
    </div>

    <!-- Global Tooltip -->
    {#if tooltipShow}
        <div
            class="fixed z-60 pointer-events-none"
            style="left: {tooltipX}px; top: {tooltipY}px; transform: translateY(-50%);"
        >
            <SidebarTooltip text={tooltipText} />
        </div>
    {/if}
</div>

<style>
    .sidebar {
        display: flex;
        flex-direction: column;
        align-items: center;
        height: 100%;
        padding: 0.5rem;
        border-right: 1px solid var(--color-border-default);
        background: var(--color-surface);
        transition: all 0.2s ease;
    }

    .logo-container {
        flex-shrink: 0;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background: var(--color-background);
        border-radius: 0.75rem;
        padding: 0.5rem;
        border: 1px solid var(--color-border-default);
        margin-bottom: 1rem;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .logo-container:hover {
        background: var(--color-accent-base);
        border-color: var(--color-accent-hover);
        transform: translateY(-1px);
    }

    .logo-container:active {
        transform: translateY(0);
        background: var(--color-accent-active);
    }

    .instances-container {
        flex: 1;
        width: 100%;
        position: relative;
        overflow: hidden;
    }

    .instances-scroll {
        height: 100%;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        padding: 0.5rem 0;
        scrollbar-width: none; /* Firefox */
        -ms-overflow-style: none; /* IE and Edge */
    }

    .instances-scroll::-webkit-scrollbar {
        display: none; /* Chrome, Safari and Opera */
    }

    .separator {
        margin: 0.5rem 1rem;
        width: calc(100% - 2rem);
        height: 1px;
        background: var(--color-border-subtle);
    }

    /* Dark theme specific adjustments */
    .sidebar.dark {
        box-shadow: 2px 0 8px rgba(0, 0, 0, 0.3);
    }

    .sidebar:not(.dark) {
        box-shadow: 2px 0 8px rgba(0, 0, 0, 0.1);
    }
</style>
