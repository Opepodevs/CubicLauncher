import type { Themes, FullTheme } from "../types";

export async function get_theme_content(theme: Themes): Promise<FullTheme> {
  try {
    // Importa dinámicamente el archivo JSON basado en el nombre del tema
    const themeModule = await import(`./themes/${theme}.json`);
    return themeModule.default;
  } catch (error) {
    throw new Error(`No se pudo cargar el tema: ${theme}. Error: ${error}`);
  }
}
