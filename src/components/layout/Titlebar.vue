<script setup lang="ts">
import { useLauncherStore } from "../../stores/LauncherStore";
const store = useLauncherStore();
import { getCurrentWindow } from "@tauri-apps/api/window";
import logo from "../../assets/logo.vue";
import { getIcon } from "../../lib/utils";
const appWindow = getCurrentWindow();

const handleMaximize = async () => {
    if (await appWindow.isMaximized()) {
        appWindow.unmaximize();
    } else {
        appWindow.maximize();
    }
};

const startDrag = async (event: MouseEvent) => {
    // Solo iniciar drag con botón izquierdo
    if (event.button !== 0) return;

    // Prevenir drag si se hace clic en botones
    const target = event.target as HTMLElement;
    if (target.closest(".titlebar-buttons") || target.closest("button")) {
        return;
    }

    try {
        await appWindow.startDragging();
    } catch (error) {
        console.error("Error starting drag:", error);
    }
};
</script>

<template>
    <div
        class="flex items-center justify-between h-9 bg-stone-800 border-b border-stone-600 px-4 titlebar z-1000"
        @mousedown="startDrag"
    >
        <!-- Left spacer for balance -->
        <div class="w-24"></div>
        <div
            class="flex items-center gap-2.5 px-3 bg-stone-750/30 rounded-md border border-stone-600/30"
        >
            <component
                v-if="store.CurrentInstance"
                :is="getIcon(store.CurrentInstance.loader)"
                class="w-6 h-6"
            />
            <logo v-else class="w-6 h-6" />
            <span class="text-sm text-stone-200 select-none">
                {{
                    store.currentView === "settings"
                        ? "Settings — Cubic"
                        : store.CurrentInstance
                          ? `${store.CurrentInstance.name} — Cubic`
                          : "CubicLauncher"
                }}
            </span>
        </div>
        <!-- Window controls -->
        <div class="flex items-center gap-0.5 titlebar-buttons">
            <button
                @click="appWindow.minimize"
                class="w-7 h-7 flex items-center justify-center text-stone-400 hover:text-stone-200 hover:bg-stone-700 rounded-sm transition-all duration-75 no-drag"
                aria-label="Minimize"
            >
                <svg width="12" height="12" viewBox="0 0 12 12">
                    <path
                        d="M3 6h6"
                        stroke="currentColor"
                        stroke-width="1.5"
                        stroke-linecap="round"
                    />
                </svg>
            </button>
            <button
                @click="handleMaximize"
                class="w-7 h-7 flex items-center justify-center text-stone-400 hover:text-stone-200 hover:bg-stone-700 rounded-sm transition-all duration-75 no-drag"
                aria-label="Maximize"
            >
                <svg width="12" height="12" viewBox="0 0 12 12">
                    <rect
                        x="3"
                        y="3"
                        width="6"
                        height="6"
                        stroke="currentColor"
                        stroke-width="1.5"
                        fill="none"
                        rx="1"
                    />
                </svg>
            </button>
            <button
                @click="appWindow.close"
                class="w-7 h-7 flex items-center justify-center text-stone-400 hover:text-stone-200 hover:bg-stone-600 rounded-sm transition-all duration-75 no-drag"
                aria-label="Close"
            >
                <svg width="12" height="12" viewBox="0 0 12 12">
                    <path
                        d="M3 3l6 6M9 3l-6 6"
                        stroke="currentColor"
                        stroke-width="1.5"
                        stroke-linecap="round"
                    />
                </svg>
            </button>
        </div>
    </div>
</template>

<style scoped>
.titlebar {
    -webkit-app-region: drag;
    user-select: none;
    cursor: default;
}

.titlebar-buttons,
.no-drag {
    -webkit-app-region: no-drag;
}

/* Mejor feedback visual */
.titlebar:hover {
    cursor: move;
}

.titlebar-buttons:hover {
    cursor: default;
}

/* Asegurar que los botones tengan el cursor correcto */
.titlebar-buttons button {
    cursor: pointer;
}
</style>
