<script lang="ts">
    import Controller from "@assets/icons/UI/controller.svelte";
    import PlusSquare from "@assets/icons/UI/plus-square.svelte";
    import Logo from "@assets/Logo.svelte";
    import BaseModal from "@components/modals/base_modal.svelte";
    import { currentTheme } from "@stores/theme";

    const text_color = $derived($currentTheme?.text);
    let isOpen = $state(false);

    const handle_close = () => {
        isOpen = false;
    };
    const handle_modal = () => {
        isOpen = !isOpen;
    };
</script>

<div class="flex items-center justify-center h-full">
    <div class="text-center max-w-2xl">
        <div class="w-24 h-24 mx-auto mb-6">
            <Logo className="w-full h-full" />
        </div>
        <h1
            class="text-3xl font-bold mb-4"
            style="color: {text_color?.primary};"
        >
            Welcome to Launcher
        </h1>
        <p class="text-lg mb-8" style="color: {text_color?.secondary};">
            Create a new instance or play your most recent game
        </p>
        <div class="grid grid-cols-2 gap-6">
            <button
                onclick={() => handle_modal()}
                class="flex items-center justify-center p-2 gap-2 bg-stone-800 rounded-xl border border-stone-600 cursor-pointer hover:bg-stone-700 transition-all"
            >
                <PlusSquare />
                <span style="color: {text_color?.secondary};">Create new</span>
            </button>
            <button
                class="flex items-center justify-center p-2 gap-2 bg-stone-800 rounded-xl border border-stone-600 cursor-pointer hover:bg-stone-700 transition-all"
                onclick={() => console.log("Play recent")}
            >
                <Controller />
                <span style="color: {text_color?.secondary};">Play Recent</span>
            </button>
        </div>

        <BaseModal {isOpen} title="asd" on:close={handle_close}>
            {#snippet children()}
                <h1>xd</h1>
            {/snippet}
        </BaseModal>
    </div>
</div>
