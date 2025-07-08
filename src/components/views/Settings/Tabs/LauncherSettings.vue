<template>
  <div class="space-y-6">
    <!-- Instancias y Java -->
    <div class="bg-stone-800 border border-stone-600 rounded-lg">
      <div class="px-6 py-4 border-b border-stone-600">
        <h3 class="text-lg font-semibold text-stone-200 flex items-center gap-2">
          <svg class="w-5 h-5 text-stone-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v4a1 1 0 001 1h3m10-5h2a2 2 0 012 2v12a2 2 0 01-2 2H7a2 2 0 01-2-2V7a2 2 0 012-2h2"></path>
          </svg>
          Instancias y Java
        </h3>
      </div>
      <div class="p-6 space-y-6">
        <!-- Carpeta de Instancias -->
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-stone-200">Carpeta de instancias</p>
            <p class="text-sm text-stone-400">Selecciona la carpeta donde se guardarán las instancias de Minecraft</p>
          </div>
          <div class="flex items-center gap-2">
            <input type="text" class="bg-stone-700 border border-stone-600 text-stone-200 text-sm rounded-lg px-3 py-2 w-64" placeholder="C:/Users/Usuario/MinecraftInstances" disabled />
            <button class="px-3 py-2 rounded-md bg-stone-600 text-stone-200 hover:bg-stone-500 transition-colors text-sm">Cambiar</button>
          </div>
        </div>
        <!-- Ruta de Java -->
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-stone-200">Ruta de Java</p>
            <p class="text-sm text-stone-400">Selecciona la ruta del ejecutable de Java</p>
          </div>
          <div class="flex items-center gap-2">
            <input type="text" class="bg-stone-700 border border-stone-600 text-stone-200 text-sm rounded-lg px-3 py-2 w-64" placeholder="C:/Program Files/Java/bin/javaw.exe" disabled />
            <button class="px-3 py-2 rounded-md bg-stone-600 text-stone-200 hover:bg-stone-500 transition-colors text-sm">Buscar</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Rendimiento -->
    <div class="bg-stone-800 border border-stone-600 rounded-lg">
      <div class="px-6 py-4 border-b border-stone-600">
        <h3 class="text-lg font-semibold text-stone-200 flex items-center gap-2">
          <svg class="w-5 h-5 text-stone-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16"></path>
          </svg>
          Rendimiento
        </h3>
      </div>
      <div class="p-6 space-y-6">
        <!-- Memoria RAM -->
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-stone-200">Memoria RAM asignada</p>
            <p class="text-sm text-stone-400">Elige cuánta memoria RAM se asignará a Minecraft</p>
          </div>
          <div class="flex items-center gap-2">
            <input type="range" min="2" max="16" value="4" class="w-40 accent-stone-600" disabled />
            <span class="text-stone-200 text-sm">4 GB</span>
          </div>
        </div>
        <!-- Parámetros JVM -->
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-stone-200">Parámetros JVM adicionales</p>
            <p class="text-sm text-stone-400">Agrega parámetros personalizados para Java</p>
          </div>
          <input type="text" class="bg-stone-700 border border-stone-600 text-stone-200 text-sm rounded-lg px-3 py-2 w-96" placeholder="-Xmx4G -XX:+UnlockExperimentalVMOptions" disabled />
        </div>
      </div>
    </div>

    <!-- Ventana y comportamiento -->
    <div class="bg-stone-800 border border-stone-600 rounded-lg">
      <div class="px-6 py-4 border-b border-stone-600">
        <h3 class="text-lg font-semibold text-stone-200 flex items-center gap-2">
          <svg class="w-5 h-5 text-stone-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <rect width="20" height="14" x="2" y="5" rx="2" ry="2" stroke-width="2" stroke="currentColor" fill="none" />
          </svg>
          Ventana y comportamiento
        </h3>
      </div>
      <div class="p-6 space-y-6">
        <!-- Opciones adicionales -->
        <div class="flex flex-col gap-4">
          <label class="flex items-center gap-3">
            <input type="checkbox" class="accent-stone-600" disabled />
            <span class="text-stone-200 text-sm">Cerrar launcher al iniciar el juego</span>
          </label>
          <label class="flex items-center gap-3">
            <input type="checkbox" class="accent-stone-600" disabled />
            <span class="text-stone-200 text-sm">Mostrar logs avanzados</span>
          </label>
        </div>
      </div>
    </div>
  </div>
</template>

<!-- No script ni lógica, solo diseño -->

<style scoped>
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.2s;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
</style> 