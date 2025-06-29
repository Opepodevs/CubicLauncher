<script lang="ts">
    import { appStore } from "@stores/launcher";
    import BaseModal from "../base_modal.svelte";
    import vanillaIcon from "@assets/icons/minecraft/vanilla.svelte";
    import fabricIcon from "@assets/icons/minecraft/fabric.svelte";
    import QuiltIcon from "@assets/icons/minecraft/Quilt.svelte";
    import { versions, type MinecraftVersion } from "@stores/launcher";

    let name = "";
    let selectedVersion = "";
    let loader: "vanilla" | "forge" | "fabric" | "quilt" = "vanilla";
    $: versionList = $versions;

    // Icono según loader
    $: icon = loader === "vanilla" ? vanillaIcon : loader === "fabric" ? fabricIcon : loader === "quilt" ? QuiltIcon : vanillaIcon;

    function uuidv4() {
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
            var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    }

    function createInstance() {
        if (!name.trim()) return;
        appStore.addInstance({
            id: uuidv4(),
            name,
            version: selectedVersion,
            loader,
            icon
        });
        appStore.handleModal('newInstance'); // Cierra el modal
        name = "";
        selectedVersion = "";
        loader = "vanilla";
    }
</script>

<BaseModal isOpen={$appStore.isNewInstanceModalOpen} title="Crear nueva instancia">
    <form class="create-instance-form" on:submit|preventDefault={createInstance}>
        <label>
            Nombre:
            <input type="text" bind:value={name} placeholder="Nombre de la instancia" required />
        </label>
        <label>
            Versión:
            <select bind:value={selectedVersion} required>
                <option value="" disabled selected>Selecciona una versión</option>
                {#each versionList as v}
                    <option value={v.id}>{v.id} ({v.type})</option>
                {/each}
            </select>
        </label>
        <label>
            Loader:
            <select bind:value={loader}>
                <option value="vanilla">Vanilla</option>
                <option value="fabric">Fabric</option>
                <option value="quilt">Quilt</option>
            </select>
        </label>
        <button type="submit">Crear instancia</button>
    </form>
</BaseModal>

<style lang="scss">
.create-instance-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1.5rem;
    min-width: 300px;
    label {
        display: flex;
        flex-direction: column;
        font-weight: 500;
        color: var(--color-text-primary);
        input, select {
            margin-top: 0.25rem;
            padding: 0.5rem;
            border-radius: 0.5rem;
            border: 1px solid var(--color-border-default);
            background: var(--color-surface);
            color: var(--color-text-primary);
            font-size: 1rem;
        }
    }
    button[type="submit"] {
        margin-top: 1rem;
        padding: 0.75rem;
        border-radius: 0.5rem;
        background: var(--color-accent-base);
        color: #fff;
        border: none;
        font-size: 1.1rem;
        font-weight: bold;
        cursor: pointer;
        transition: background 0.2s;
        &:hover {
            background: var(--color-accent-hover);
        }
        &:active {
            background: var(--color-accent-active);
        }
    }
}
</style>
