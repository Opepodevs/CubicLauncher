import { Loaders } from "../types";
import Quilt from "../assets/icons/minecraft/Quilt.vue";
import Fabric from "../assets/icons/minecraft/fabric.vue";
import Vanilla from "../assets/icons/minecraft/vanilla.vue";

const iconMap = {
	[Loaders.Vanilla]: Vanilla,
	[Loaders.Fabric]: Fabric,
	[Loaders.Forge]: Quilt, // Temporal - cambiar por Forge cuando tengas el icono
	[Loaders.Quilt]: Quilt,
	[Loaders.NeoForge]: Quilt, // Temporal - cambiar por NeoForge cuando tengas el icono
} as const;

export function getIcon(loader: Loaders | string | undefined) {
	if (!loader) {
		return Vanilla;
	}
	
	const icon = iconMap[loader as Loaders];
	return icon || Vanilla;
}