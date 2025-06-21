import { register, init as svelteI18nInit, getLocaleFromNavigator, locale, t } from 'svelte-i18n';
import { derived, writable } from 'svelte/store';

const modules = import.meta.glob("../locales/*.json", { eager: true });

const languages: Record<string, any> = {};
Object.entries(modules).forEach(([path, mod]: any) => {
	const code = path.match(/([a-zA-Z0-9_-]+)\.json$/)?.[1] || '';
	languages[code] = mod.default;
	// The library expects a function that returns a promise
	register(code, () => Promise.resolve(mod.default));
});

export const isLoaded = writable(false);

export async function initI18n() {
	const savedLanguage = (typeof localStorage !== 'undefined' && localStorage.getItem("selectedLanguage")) || getLocaleFromNavigator() || 'en';

	await svelteI18nInit({
		fallbackLocale: 'en',
		initialLocale: savedLanguage,
	});
	
	isLoaded.set(true);
}

locale.subscribe(value => {
	if (typeof localStorage !== 'undefined' && value) {
		localStorage.setItem("selectedLanguage", value);
	}
});

export const availableLanguages = Object.entries(languages)
	.map(([code, data]) => ({
		code,
		name: data?.language?.name || code,
	}))
	.sort((a, b) => a.name.localeCompare(b.name));

export const currentLanguage = locale;

export const locales = derived(locale, ($locale) => {
	return languages[$locale || 'en'];
});

export { t };