<script lang="ts">
    // Componentes
    import Sidebar from "@components/sidebar/Sidebar.svelte";
    import Titlebar from "./Titlebar.svelte";
    import InstanceView from "@views/Welcome.svelte";
    import Welcome from "../views/Welcome.svelte";
    import CreateInstanceModal from "@components/modals/createInstance/createInstanceModal.svelte";

    // Tienda de instancias
    import { appStore } from "@stores/launcher";

    // Tienda y tema
    import { themeStore } from "@stores/theme";
    import { onMount } from "svelte";
    import SettingsModal from "@components/modals/Settings/settingsModal.svelte";

    // Reactividad explícita: derivamos currentInstance del store
    let currentInstance = $derived($appStore.currentInstance);

    onMount(async () => {
        await themeStore.initializeTheme();
    });
</script>

<div class="app-layout">
    <!-- Sidebar -->
    <Sidebar />

    <!-- Sección principal -->
    <div class="main-section">
        <!-- Barra de título -->
        <Titlebar />

        <!-- Contenido -->
        <div class="main-content">
            <CreateInstanceModal />
            <SettingsModal />
            {#if currentInstance}
                <!-- <InstanceView {currentInstance} /> -->
            {:else}
                <Welcome />
            {/if}
        </div>
    </div>
</div>
