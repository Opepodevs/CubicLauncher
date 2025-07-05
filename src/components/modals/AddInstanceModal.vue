<template>
  <BaseModal v-model="store.isAddInstanceModalOpen" :title="languageStore.getTranslation('Launcher.addInstance.title')">
    <div class="space-y-4">
      <!-- Instance Name -->
      <div class="space-y-2">
        <label class="block text-sm font-medium text-stone-300">
          {{ languageStore.getTranslation('Launcher.addInstance.name') }}
        </label>
        <input v-model="formData.name" type="text"
          class="w-full px-3 py-2 bg-stone-700 border border-stone-600 rounded-md text-stone-200 focus:outline-none focus:border-stone-500"
          :placeholder="languageStore.getTranslation('Launcher.addInstance.namePlaceholder')">
      </div>

      <!-- Version Selection -->
      <div class="space-y-2">
        <label class="block text-sm font-medium text-stone-300">
          {{ languageStore.getTranslation('Launcher.addInstance.version') }}
        </label>
        <select v-model="formData.version"
          class="w-full px-3 py-2 bg-stone-700 border border-stone-600 rounded-md text-stone-200 focus:outline-none focus:border-stone-500">
          <option value="" disabled>{{ languageStore.getTranslation('Launcher.addInstance.selectVersion') }}</option>
          <option v-for="version in versions" :key="version" :value="version">
            {{ version }}
          </option>
          <option>
            1.16.5
          </option>
        </select>
      </div>

      <!-- Loader Selection -->
      <div class="space-y-2">
        <label class="block text-sm font-medium text-stone-300">
          {{ languageStore.getTranslation('Launcher.addInstance.loader') }}
        </label>
        <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
          <button v-for="loader in loaders" :key="loader.id" @click="formData.loader = loader.id"
            class="flex flex-col items-center p-4 border rounded-lg transition-all duration-200 hover:scale-105" :class="[
              formData.loader === loader.id
                ? 'bg-stone-700 border-stone-500 text-stone-200 shadow-lg'
                : 'border-stone-600 text-stone-400 hover:bg-stone-700/50 hover:border-stone-500'
            ]">
            <component :is="loader.icon" class="w-10 h-10 mb-3" />
            <span class="text-sm font-medium">{{ loader.name }}</span>
          </button>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex justify-end gap-3 pt-4">
        <button @click="handleCreate"
          class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="!isFormValid">
          {{ languageStore.getTranslation('Launcher.addInstance.create') }}
        </button>
      </div>
    </div>
  </BaseModal>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import Vanilla from "../../assets/icons/minecraft/vanilla.vue";
import Fabric from "../../assets/icons/minecraft/fabric.vue";
import Quilt from "../../assets/icons/minecraft/Quilt.vue";
import { useLanguageStore } from "../../stores/LanguageStore";
import { useLauncherStore } from "../../stores/LauncherStore";
import BaseModal from "./BaseModal.vue";
import { Loaders } from "../../types";
import { toast } from 'vue-sonner'

const store = useLauncherStore();
const languageStore = useLanguageStore();
const versions = ref<string[]>([]);

versions.value.push("1.16.5", "1.12.2")

const loaders = [
  {
    id: Loaders.Vanilla,
    name: "Vanilla",
    icon: Vanilla,
    EnumValue: Loaders.Vanilla,
  },
];

const formData = ref<{
  name: string;
  version: string;
  loader: Loaders;
}>({
  name: "",
  version: "",
  loader: Loaders.Vanilla,
});

const isFormValid = computed(() => {
  return formData.value.name && formData.value.version;
});

const handleCreate = async () => {
  if (!isFormValid.value) return;

  // Create new instance
  let result = await store.addInstance({
    name: formData.value.name,
    loader: formData.value.loader,
    version: formData.value.version,
    custom_args: [],
    downloaded: false
  });

  if (result.success) {
    store.toggleAddInstanceModal();
    formData.value = {
      name: "",
      version: "",
      loader: Loaders.Vanilla,
    };
  } else if (result.error) {
    toast.error(result.error.error_message ?? "Error desconocido");
  }
};
</script>