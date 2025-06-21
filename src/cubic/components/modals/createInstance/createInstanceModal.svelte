<script lang="ts">
    import { appStore } from "@stores/launcher";
    import BaseModal from "../base_modal.svelte";
    import Forge from "@components/modals/createInstance/tabs/forge.svelte";
    import controller from "@assets/icons/UI/controller.svelte";
    import Logo from "@assets/Logo.svelte";

    let activeTab = $state("general");

    // Predefine las tabs con contenido y iconos
    const tabs = [
        {
            id: "general",
            name: "General",
            icon: controller,
            content: Forge,
        },
        {
            id: "launcher",
            name: "Launcher",
            icon: controller,
            content: Forge,
        },
        {
            id: "accounts",
            name: "Luis roscasel",
            icon: controller,
            content: Forge,
        },
    ];

    // Reactivo para contenido actual de tab (evita función en template)
    let currentTabContent = $state(
        tabs.find((tab) => tab.id === activeTab)?.content ?? null,
    );

    // Handler para click tabs: evitar función inline en template
    function selectTab(id: string) {
        activeTab = id;
    }
</script>

<BaseModal isOpen={$appStore.isNewInstanceModalOpen} title="xd">
    <div class="flex h-full">
        <nav
            class="w-56 border-r border-[#272727ff] h-full flex flex-col space-y-0.5"
            aria-label="Tabs"
        >
            {#each tabs as { id, name, icon: Icon }}
                <button
                    type="button"
                    onclick={() => selectTab(id)}
                    class={`w-full text-left px-4 py-2.5 text-sm font-medium transition-colors flex items-center gap-2
            ${
                activeTab === id
                    ? "bg-[#272727ff] text-[#d6d2d2ff] border-l-2 border-[#78716c]"
                    : "text-[#d6d2d2ff]/60 hover:text-[#d6d2d2ff] hover:bg-[#272727ff]"
            }`}
                    aria-selected={activeTab === id}
                    role="tab"
                    tabindex={activeTab === id ? 0 : -1}
                >
                    <Icon size="24" />
                    {name}
                </button>
            {/each}
        </nav>

        <section class="flex-1 h-full p-2 flex flex-col" role="tabpanel">
            {#if currentTabContent}
                <currentTabContent />
            {:else}
                <div
                    class="flex items-center justify-center h-full text-stone-400"
                >
                    No content available for this tab
                </div>
            {/if}
        </section>
    </div>
</BaseModal>
