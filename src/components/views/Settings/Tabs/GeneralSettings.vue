<template>
    <div class="space-y-6">
        <!-- Language Section -->
        <div class="bg-stone-800 border border-stone-600 rounded-lg">
            <div class="px-6 py-4 border-b border-stone-600">
                <h3 class="text-lg font-semibold text-stone-200 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-stone-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129"></path>
                    </svg>
                    Idioma
                </h3>
            </div>
            <div class="p-6">
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-medium text-stone-200">Idioma de la aplicación</p>
                        <p class="text-sm text-stone-400">Selecciona el idioma en el que se mostrará la interfaz</p>
                    </div>
                    <select class="bg-stone-700 border border-stone-600 text-stone-200 text-sm rounded-lg focus:ring-stone-500 focus:border-stone-500 block px-3 py-2">
                        <option value="es">Español</option>
                        <option value="en">English</option>
                        <option value="fr">Français</option>
                        <option value="de">Deutsch</option>
                    </select>
                </div>
            </div>
        </div>

        <!-- Theme Section -->
        <div class="bg-stone-800 border border-stone-600 rounded-lg">
            <div class="px-6 py-4 border-b border-stone-600">
                <h3 class="text-lg font-semibold text-stone-200 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-stone-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"></path>
                    </svg>
                    Tema
                </h3>
            </div>
            <div class="p-6">
                <p class="text-sm font-medium text-stone-200 mb-4">Modo de apariencia</p>
                <div class="flex flex-col gap-2">
                    <label
                        v-for="option in themeOptions"
                        :key="option.value"
                        class="flex items-center justify-between cursor-pointer rounded-lg border transition-colors duration-200 px-4 py-3 group"
                        :class="[
                            selectedTheme === option.value
                                ? 'border-stone-400 bg-stone-700'
                                : 'border-stone-600 bg-stone-800 hover:bg-stone-700',
                        ]"
                    >
                        <div class="flex items-center gap-4">
                            <span :class="['w-7 h-7 rounded-full border border-stone-500', option.preview]" />
                            <div>
                                <div class="text-stone-200 font-medium">{{ option.label }}</div>
                                <div class="text-xs text-stone-400">{{ option.desc }}</div>
                            </div>
                        </div>
                        <input
                            type="radio"
                            name="theme"
                            :value="option.value"
                            v-model="selectedTheme"
                            class="sr-only"
                        />
                        <span v-if="selectedTheme === option.value" class="flex items-center justify-center w-6 h-6 rounded-full bg-stone-600">
                            <svg class="w-4 h-4 text-stone-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                            </svg>
                        </span>
                    </label>
                </div>
            </div>
        </div>

        <!-- Information Section -->
        <div class="bg-stone-800 border border-stone-600 rounded-lg">
            <div class="px-6 py-4 border-b border-stone-600">
                <h3 class="text-lg font-semibold text-stone-200 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-stone-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    Información
                </h3>
            </div>
            <div class="p-6">
                <div class="space-y-6">
                    <!-- Version Info -->
                    <div class="flex items-center justify-between p-4 bg-stone-700 rounded-lg">
                        <div class="flex items-center">
                            <div class="w-10 h-10 bg-stone-600 rounded-lg flex items-center justify-center">
                                <svg class="w-5 h-5 text-stone-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                            </div>
                            <div class="ml-4">
                                <p class="text-sm font-medium text-stone-200">Versión</p>
                                <p class="text-sm text-stone-400">CubicLauncher v1.0.0</p>
                            </div>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-stone-600 text-stone-300">
                                Actualizado
                            </span>
                            <button 
                                @click="checkForUpdates"
                                class="inline-flex items-center px-3 py-1.5 rounded-md text-xs font-medium bg-stone-600 text-stone-300 hover:bg-stone-500 hover:text-stone-200 transition-colors"
                            >
                                <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                                </svg>
                                Buscar actualización
                            </button>
                        </div>
                    </div>

                    <!-- System Info -->
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div class="p-4 bg-stone-700 rounded-lg">
                            <div class="flex items-center">
                                <div class="w-8 h-8 bg-stone-600 rounded-lg flex items-center justify-center">
                                    <svg class="w-4 h-4 text-stone-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"></path>
                                    </svg>
                                </div>
                                <div class="ml-3">
                                    <p class="text-sm font-medium text-stone-200">Sistema Operativo</p>
                                    <p class="text-sm text-stone-400">Windows 10</p>
                                </div>
                            </div>
                        </div>

                        <div class="p-4 bg-stone-700 rounded-lg">
                            <div class="flex items-center">
                                <div class="w-8 h-8 bg-stone-600 rounded-lg flex items-center justify-center">
                                    <svg class="w-4 h-4 text-stone-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                </div>
                                <div class="ml-3">
                                    <p class="text-sm font-medium text-stone-200">Build</p>
                                    <p class="text-sm text-stone-400">v1.0.0</p>
                                </div>
                            </div>
                        </div>

                        <div class="p-4 bg-stone-700 rounded-lg">
                            <div class="flex items-center justify-between">
                                <div class="flex items-center">
                                    <div class="w-8 h-8 bg-stone-600 rounded-lg flex items-center justify-center">
                                        <svg class="w-4 h-4 text-stone-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path>
                                        </svg>
                                    </div>
                                    <div class="ml-3">
                                        <p class="text-sm font-medium text-stone-200">Código Fuente</p>
                                        <p class="text-sm text-stone-400">GitHub</p>
                                    </div>
                                </div>
                                <a 
                                    href="https://github.com/CubicLauncher/CubicLauncher"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="inline-flex items-center px-3 py-1.5 rounded-md text-xs font-medium bg-stone-600 text-stone-300 hover:bg-stone-500 hover:text-stone-200 transition-colors"
                                >
                                    <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
                                    </svg>
                                    Ver
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const selectedTheme = ref('light')

const themeOptions = [
    {
        value: 'light',
        label: 'Claro',
        desc: 'Fondo claro, ideal para el día',
        preview: 'bg-stone-100',
    },
    {
        value: 'dark',
        label: 'Oscuro',
        desc: 'Fondo oscuro, ideal para la noche',
        preview: 'bg-stone-900',
    },
    {
        value: 'auto',
        label: 'Automático',
        desc: 'Se adapta a tu sistema',
        preview: 'bg-gradient-to-r from-stone-100 to-stone-900',
    }
]
</script>

<style scoped>
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.2s;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
</style> 
