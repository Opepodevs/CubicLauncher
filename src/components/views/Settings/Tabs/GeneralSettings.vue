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
                <div class="grid grid-cols-3 gap-4">
                    <div class="relative">
                        <input type="radio" id="light" name="theme" value="light" class="sr-only" checked>
                        <label for="light" class="flex flex-col items-center p-4 bg-stone-700 border-2 border-stone-600 rounded-lg cursor-pointer hover:bg-stone-600 transition-colors">
                            <div class="w-8 h-8 bg-stone-400 rounded-full mb-2 flex items-center justify-center">
                                <svg class="w-4 h-4 text-stone-800" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd"></path>
                                </svg>
                            </div>
                            <span class="text-sm font-medium text-stone-200">Claro</span>
                        </label>
                    </div>
                    <div class="relative">
                        <input type="radio" id="dark" name="theme" value="dark" class="sr-only">
                        <label for="dark" class="flex flex-col items-center p-4 bg-stone-800 border-2 border-stone-600 rounded-lg cursor-pointer hover:bg-stone-700 transition-colors">
                            <div class="w-8 h-8 bg-stone-600 rounded-full mb-2 flex items-center justify-center">
                                <svg class="w-4 h-4 text-stone-200" fill="currentColor" viewBox="0 0 20 20">
                                    <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path>
                                </svg>
                            </div>
                            <span class="text-sm font-medium text-stone-200">Oscuro</span>
                        </label>
                    </div>
                    <div class="relative">
                        <input type="radio" id="auto" name="theme" value="auto" class="sr-only">
                        <label for="auto" class="flex flex-col items-center p-4 bg-stone-700 border-2 border-stone-600 rounded-lg cursor-pointer hover:bg-stone-600 transition-colors">
                            <div class="w-8 h-8 bg-gradient-to-r from-stone-400 to-stone-600 rounded-full mb-2 flex items-center justify-center">
                                <svg class="w-4 h-4 text-stone-800" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd"></path>
                                </svg>
                            </div>
                            <span class="text-sm font-medium text-stone-200">Automático</span>
                        </label>
                    </div>
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
// Component logic can be added here later
</script> 