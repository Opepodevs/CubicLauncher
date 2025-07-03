<script setup lang="ts">
import { onMounted } from "vue";
import Main from "./components/layout/Main.vue";
import { useLauncherStore } from "./stores/LauncherStore";
import { Toaster } from 'vue-sonner'
import 'vue-sonner/style.css'

const store = useLauncherStore();

onMounted(async () => {
    await store.loadInstances();
});
</script>

<template>
    <Main />
    <Toaster
  :rich-colors="true"
  :toast-options="{
    style: {
      background: '#27272a',       // bg-stone-800
      border: '1px dotted ##57534e', // border-stone-600
      color: '#e7e5e4'             // text-stone-300
    }
  }"
/>

</template>