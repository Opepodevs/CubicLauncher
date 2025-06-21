// stores/launcher.ts
import { derived, writable } from "svelte/store";
import { type Component } from "svelte";

interface Instance {
  id: string;
  name: string;
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
  isNewInstanceModalOpen: true,
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
