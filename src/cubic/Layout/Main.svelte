<script lang="ts">
    import Sidebar from "@components/sidebar/Sidebar.svelte";
    import Titlebar from "./Titlebar.svelte";
    // import InstanceView from "../views/InstanceView.svelte";
    import NoInstanceView from "../views/Welcome.svelte";
    import "@css/main.scss";
    import { appStore } from "@stores/launcher";
    import {
        themeStore,
        selectedTheme,
        isDarkTheme,
        isThemeLoading,
    } from "@stores/theme";
    import { Themes } from "../types";
    import { onMount } from "svelte";
    import Welcome from "../views/Welcome.svelte";
    import CreateInstanceModal from "@components/modals/createInstance/createInstanceModal.svelte";

    $: currentInstance = $appStore.currentInstance;

    // async function setMokaTheme() {
    //     await themeStore.switchTheme(Themes.moka);
    // }

    // async function setMokaWhiteTheme() {
    //     await themeStore.switchTheme(Themes.moka_white);
    // }

    // Initialize theme on mount
    onMount(async () => {
        await themeStore.initializeTheme();
    });
</script>

<div class="app-layout">
    <!-- Sidebar -->
    <Sidebar />

    <!-- Main Section -->
    <div class="main-section">
        <!-- Titlebar -->
        <Titlebar />

        <!-- Main Content -->
        <div class="main-content">
            <CreateInstanceModal />
            <!-- {#if currentInstance}
                <InstanceView instance={currentInstance} />
            {:else}
                <NoInstanceView />
            {/if} -->
            <Welcome />
        </div>
    </div>
</div>
