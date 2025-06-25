// main.ts (o main.js)
import App from "./App.svelte";
import { initI18n } from "./cubic/stores/language";
import { mount } from "svelte";
async function bootstrap() {
  await initI18n(); // Esto reemplaza tu load()

  mount(App, {
    target: document.getElementById("app")!,
  });
}

bootstrap();
