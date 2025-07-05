import type { Component } from "vue";
import Vanilla from "../../assets/icons/minecraft/vanilla.vue";
import { useLanguageStore } from "../../stores/LanguageStore";

export interface INoInstance {
	message: string;
	description: string;
	icon: Component;
}

export const getNoInstanceMessages = (): Record<number, INoInstance> => {
	const languageStore = useLanguageStore();
	const t = languageStore.getTranslation;

	return {
		1: {
			message: t("Launcher.noInstance.1.message"),
			description: t("Launcher.noInstance.1.description"),
			icon: Vanilla,
		},
		2: {
			message: t("Launcher.noInstance.2.message"),
			description: t("Launcher.noInstance.2.description"),
			icon: Vanilla,
		},
		3: {
			message: t("Launcher.noInstance.3.message"),
			description: t("Launcher.noInstance.3.description"),
			icon: Vanilla,
		},
		4: {
			message: t("Launcher.noInstance.4.message"),
			description: t("Launcher.noInstance.4.description"),
			icon: Vanilla,
		},
	};
};
