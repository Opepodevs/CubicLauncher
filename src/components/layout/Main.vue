<template>
  <div class="flex h-screen">
    <!-- Sidebar - Hidden when in settings -->
    <Sidebar v-if="store.currentView !== 'settings'" />
    <!-- Main Section -->
    <div class="flex flex-col flex-1 overflow-hidden">
      <!-- Titlebar -->
      <Titlebar />

      <!-- Main Content -->
      <div class="flex-1 overflow-y-auto bg-stone-950">
        <AddInstanceModal />
        <SettingsView v-if="store.currentView === 'settings'" />
        <InstanceView v-else-if="store.currentView === 'instance' && store.CurrentInstance" :instance="store.CurrentInstance" />
        <WelcomeView v-else />
      </div>
    </div>
  </div>
</template>

<script setup>
import { useLauncherStore } from "../../stores/LauncherStore";
import AddInstanceModal from "../modals/AddInstanceModal.vue";
import Sidebar from "../navigation/Sidebar.vue";
import InstanceView from "../views/InstanceView.vue";
import SettingsView from "../views/SettingsView.vue";
import WelcomeView from "../views/WelcomeView.vue";
import Titlebar from "./Titlebar.vue";
const store = useLauncherStore();
</script>
