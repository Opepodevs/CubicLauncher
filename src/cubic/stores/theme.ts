// stores/theme.ts
import { writable, derived, get } from "svelte/store";
import { type FullTheme, Themes } from "../types";
import { get_theme_content } from "../themes/themes";

// Tipos para el estado del store
interface ThemeState {
  currentTheme: FullTheme | null;
  availableThemes: Themes[];
  selectedTheme: Themes | null;
  isLoading: boolean;
  error: string | null;
}

// Estado inicial
const initialState: ThemeState = {
  currentTheme: null,
  availableThemes: Object.values(Themes),
  selectedTheme: null,
  isLoading: false,
  error: null,
};

// Store principal
const themeState = writable<ThemeState>(initialState);

// Función para crear el store con acciones
function createThemeStore() {
  const { subscribe, set, update } = themeState;

  return {
    subscribe,

    // Actions
    async loadTheme(theme: Themes) {
      update((state) => ({ ...state, isLoading: true, error: null }));

      try {
        const themeContent = await get_theme_content(theme);
        update((state) => ({
          ...state,
          currentTheme: themeContent,
          selectedTheme: theme,
          isLoading: false,
          error: null,
        }));

        // Guardar en localStorage
        if (typeof localStorage !== "undefined") {
          localStorage.setItem("selected-theme", theme);
        }

        // Aplicar tema al documento
        applyThemeToDocument(themeContent);
      } catch (error) {
        update((state) => ({
          ...state,
          isLoading: false,
          error: error instanceof Error ? error.message : "Error desconocido",
        }));
      }
    },

    // Cambiar tema
    async switchTheme(theme: Themes) {
      const currentState = get(themeState);
      if (currentState.selectedTheme === theme) return;

      await this.loadTheme(theme);
    },

    // Obtener tema desde localStorage
    async initializeTheme() {
      let savedTheme: Themes | null = null;

      if (typeof localStorage !== "undefined") {
        const saved = localStorage.getItem("selected-theme");
        if (saved && Object.values(Themes).includes(saved as Themes)) {
          savedTheme = saved as Themes;
        }
      }

      // Cargar tema guardado o usar por defecto
      const themeToLoad = savedTheme || Themes.moka;
      await this.loadTheme(themeToLoad);
    },

    // Alternar entre temas
    async toggleTheme() {
      const currentState = get(themeState);
      const currentTheme = currentState.selectedTheme;

      if (currentTheme === Themes.moka) {
        await this.loadTheme(Themes.moka_white);
      } else {
        await this.loadTheme(Themes.moka);
      }
    },

    // Limpiar errores
    clearError() {
      update((state) => ({ ...state, error: null }));
    },

    // Reset completo
    reset() {
      set(initialState);
      if (typeof localStorage !== "undefined") {
        localStorage.removeItem("selected-theme");
      }
    },
  };
}

// Función para aplicar el tema al documento
function applyThemeToDocument(theme: FullTheme) {
  if (typeof document === "undefined") return;

  const root = document.documentElement;

  // Aplicar variables CSS
  root.style.setProperty("--color-background", theme.background);
  root.style.setProperty("--color-surface", theme.surface);
  root.style.setProperty("--color-text-primary", theme.text.primary);
  root.style.setProperty("--color-text-secondary", theme.text.secondary);
  root.style.setProperty("--color-text-disabled", theme.text.disabled);
  root.style.setProperty("--color-accent-base", theme.accent.base);
  root.style.setProperty("--color-accent-hover", theme.accent.hover);
  root.style.setProperty("--color-accent-active", theme.accent.active);
  root.style.setProperty("--color-border-default", theme.border.default);
  root.style.setProperty("--color-border-subtle", theme.border.subtle);
  root.style.setProperty("--color-state-error", theme.state.error);
  root.style.setProperty("--color-state-warning", theme.state.warning);
  root.style.setProperty("--color-state-success", theme.state.success);
  root.style.setProperty("--color-state-info", theme.state.info);

  // Agregar clase para el tono
  root.classList.remove("theme-dark", "theme-light");
  root.classList.add(`theme-${theme.meta.tone}`);
}

// Exportar el store
export const themeStore = createThemeStore();

// Stores derivados para acceso fácil
export const currentTheme = derived(
  themeState,
  ($state) => $state.currentTheme,
);
export const selectedTheme = derived(
  themeState,
  ($state) => $state.selectedTheme,
);
export const isThemeLoading = derived(themeState, ($state) => $state.isLoading);
export const themeError = derived(themeState, ($state) => $state.error);
export const availableThemes = derived(
  themeState,
  ($state) => $state.availableThemes,
);

// Helper para verificar si es tema oscuro
export const isDarkTheme = derived(
  currentTheme,
  ($theme) => $theme?.meta.tone === "dark",
);

// Helper para obtener colores específicos
export const themeColors = derived(currentTheme, ($theme) => {
  if (!$theme) return null;

  return {
    background: $theme.background,
    surface: $theme.surface,
    text: $theme.text,
    accent: $theme.accent,
    border: $theme.border,
    state: $theme.state,
  };
});
