<script setup>
import { useLauncherStore } from "../../stores/LauncherStore";
import AddInstanceModal from "../modals/AddInstanceModal.vue";
import Sidebar from "../navigation/Sidebar.vue";
import Titlebar from "./Titlebar.vue";

import { defineAsyncComponent } from "vue";

const store = useLauncherStore();

// Import dinámico para views pesadas o poco usadas
const SettingsView = defineAsyncComponent(
    () => import("../views/SettingsView.vue"),
);
const InstanceView = defineAsyncComponent(
    () => import("../views/InstanceView.vue"),
);
const WelcomeView = defineAsyncComponent(
    () => import("../views/WelcomeView.vue"),
);
</script>

<template>
    <div class="flex h-screen">
        <Sidebar v-if="store.currentView !== 'settings'" />
        <div class="flex flex-col flex-1 overflow-hidden">
            <Titlebar />
            <div class="flex-1 overflow-y-auto bg-stone-950">
                <AddInstanceModal />
                <SettingsView v-if="store.currentView === 'settings'" />
                <InstanceView
                    v-else-if="
                        store.currentView === 'instance' &&
                        store.CurrentInstance
                    "
                    :instance="store.CurrentInstance"
                />
                <WelcomeView v-else />
            </div>
        </div>
    </div>
</template>
