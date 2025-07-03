<template>
  <div class="flex h-full">
    <!-- Vertical Tabs -->
    <div class="w-56 border-r border-[#272727ff] bg-stone-900">
      <div class="p-4 border-b border-[#272727ff]">
        <div class="flex items-center justify-between mb-2">
          <button 
            @click="goBack"
            class="flex items-center gap-2 text-[#d6d2d2ff]/60 hover:text-[#d6d2d2ff] transition-colors"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
            </svg>
            {{ launcherStore.previousView === 'instance' ? 'Back to Instance' : 'Back to Home' }}
          </button>
        </div>
        <h1 class="text-xl font-semibold text-[#d6d2d2ff]">
          {{ languageStore.getTranslation('Launcher.settings.title') }}
        </h1>
      </div>
      <nav class="flex flex-col space-y-0.5 p-2" aria-label="Tabs">
        <button 
          v-for="tab in tabs" 
          :key="tab.id" 
          @click="activeTab = tab.id" 
          :class="[
            activeTab === tab.id
              ? 'bg-[#272727ff] text-[#d6d2d2ff] border-l-2 border-[#78716c]'
              : 'text-[#d6d2d2ff]/60 hover:text-[#d6d2d2ff] hover:bg-[#272727ff]',
            'w-full text-left px-4 py-2.5 text-sm font-medium transition-colors flex items-center gap-2'
          ]"
        >
          <!-- Vue Icon Component -->
          <component 
            :is="tab.icon" 
            class="w-5 h-5"
          />
          
          {{ tab.name }}
        </button>
      </nav>
    </div>

    <!-- Tab Content -->
    <div class="flex-1 p-6 bg-stone-950">
      <!-- Dynamic Tab Content -->
      <component 
        :is="getCurrentTabContent()" 
        v-if="getCurrentTabContent()"
        :key="activeTab"
      />
      <div v-else class="flex items-center justify-center h-64 text-stone-400">
        No content available for this tab
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useLanguageStore } from "../../stores/LanguageStore";
import { useLauncherStore } from "../../stores/LauncherStore";

// Import tab content components
import LauncherSettings from "./Settings/Tabs/LauncherSettings.vue";
import AccountSettings from "./Settings/Tabs/AccountSettings.vue";
import GeneralSettings from "./Settings/Tabs/GeneralSettings.vue";

// Import icons (SVG components)
import controller from "../../assets/icons/UI/controller.vue";

const languageStore = useLanguageStore();
const launcherStore = useLauncherStore();

const activeTab = ref("general");

// Function to go back to previous view
const goBack = () => {
  launcherStore.goBack();
};

// Refactored tabs array with Vue icon components
const tabs = [
	{
		id: "general",
		name: "General",
		icon: controller,
		content: GeneralSettings,
	},
	{
		id: "game",
		name: "Game",
		icon: controller,
		content: LauncherSettings,
	},
	{
		id: "launcher",
		name: "Launcher",
		icon: controller,
		content: LauncherSettings,
	},
  {
    id: "account",
    name: "Account",
    icon: controller,
    content: AccountSettings,
  },
];

// Method to get current tab content component
const getCurrentTabContent = () => {
	const currentTab = tabs.find((tab) => tab.id === activeTab.value);
	return currentTab?.content || null;
};
</script> 