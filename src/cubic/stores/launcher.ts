// stores/launcher.ts
import { derived, writable } from "svelte/store";
import { type Component } from "svelte";

interface Instance {
  id: string;
  name: string;
  version: string;
  loader: "vanilla" | "forge" | "fabric" | "quilt";
  icon: Component;
}

interface AppState {
  currentInstance: Instance | null;
  instances: Instance[];
  isLoading: boolean;
  isSettingsModalOpen: boolean;
  isNewInstanceModalOpen: boolean;
}

const initialState: AppState = {
  currentInstance: null,
  instances: [],
  isLoading: false,
  isSettingsModalOpen: false,
  isNewInstanceModalOpen: false,
};
// Store principal
const appState = writable<AppState>(initialState);

// Función para crear el store con acciones
function createAppStore() {
  const { subscribe, set, update } = appState;

  return {
    subscribe,

    // Actions
    setCurrentInstance(instance: Instance | null) {
      update((state) => ({
        ...state,
        currentInstance: instance,
      }));
    },

    addInstance(instance: Instance) {
      update((state) => ({
        ...state,
        instances: [...state.instances, instance],
      }));
    },

    removeInstance(instanceId: string) {
      update((state) => ({
        ...state,
        instances: state.instances.filter((i) => i.id !== instanceId),
        currentInstance:
          state.currentInstance?.id === instanceId
            ? null
            : state.currentInstance,
      }));
    },

    setInstances(instances: Instance[]) {
      update((state) => ({
        ...state,
        instances,
      }));
    },

    setLoading(loading: boolean) {
      update((state) => ({
        ...state,
        isLoading: loading,
      }));
    },
    handle_new_instance_modal() {
      update((state) => ({
        ...state,
        isNewInstanceModalOpen: !state.isNewInstanceModalOpen,
      }));
    },
    handle_settings_modal() {
      update((state) => ({
        ...state,
        isSettingsModalOpen: !state.isSettingsModalOpen,
      }));
    },
    reset() {
      set(initialState);
    },
  };
}

// Exportar el store
export const appStore = createAppStore();

// Stores derivados para acceso fácil
export const currentInstance = derived(
  appState,
  ($state) => $state.currentInstance,
);
export const instances = derived(appState, ($state) => $state.instances);
export const isAppLoading = derived(appState, ($state) => $state.isLoading);

export interface MinecraftVersion {
  id: string;
  type: "release" | "snapshot" | "old_beta" | "old_alpha";
  releaseTime: string;
}

const initialVersions: MinecraftVersion[] = [
  { id: "1.20.4", type: "release", releaseTime: "2023-12-01" },
  { id: "1.20.3", type: "release", releaseTime: "2023-10-01" },
  { id: "1.20", type: "release", releaseTime: "2023-06-01" },
  { id: "1.19.4", type: "release", releaseTime: "2023-03-01" },
  { id: "1.18.2", type: "release", releaseTime: "2022-02-28" },
  // ... puedes agregar más o cargar desde una API
];

export const versions = writable<MinecraftVersion[]>(initialVersions);

// Si quieres cargar desde una API, puedes agregar una función asíncrona aquí.
