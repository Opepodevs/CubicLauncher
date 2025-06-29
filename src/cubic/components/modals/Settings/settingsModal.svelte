<script lang="ts">
    import { appStore } from "@stores/launcher";
    import BaseModal from "../base_modal.svelte";
    import Forge from "@components/modals/createInstance/tabs/forge.svelte";
    import About from "@components/modals/Settings/tabs/About.svelte";
    import controller from "@assets/icons/UI/controller.svelte";
    let activeTab = $state("appearance");
    let CurrentTabContent = $state<typeof tabs[number]["content"] | null>(null);
    const tabs = [
        {
            id: "appearance",
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
            id: "about",
            name: "About",
            icon: controller,
            content: About,
        },
    ];
    $effect(() => {
        const found = tabs.find((tab) => tab.id === activeTab);
        CurrentTabContent = found?.content ?? null;
    });

    // Handler para click tabs: evitar función inline en template
    function selectTab(id: string) {
        activeTab = id;
        console.log(activeTab);
    }
</script>

<BaseModal isOpen={$appStore.isSettingsModalOpen} title="Settings">
    <div class="flex h-full">
        <nav
            class="w-56 border-r h-full flex flex-col space-y-0.5"
            style="border-color: var(--color-border-default)"
            aria-label="Tabs"
        >
            {#each tabs as { id, name, icon: Icon }}
                <button
                    type="button"
                    onclick={() => selectTab(id)}
                    class={`w-full text-left px-4 py-2.5 text-sm font-medium transition-colors flex items-center gap-2
            ${activeTab === id ? "border-l-2" : "hover:bg-opacity-50"}`}
                    style={activeTab === id
                        ? "background-color: var(--color-surface); color: var(--color-text-primary); border-left-color: var(--color-accent-base);"
                        : "color: var(--color-text-secondary);"}
                    onmouseenter={(e) => {
                        if (activeTab !== id) {
                            e.currentTarget.style.color =
                                "var(--color-text-primary)";
                            e.currentTarget.style.backgroundColor =
                                "var(--color-surface)";
                        }
                    }}
                    onmouseleave={(e) => {
                        if (activeTab !== id) {
                            e.currentTarget.style.color =
                                "var(--color-text-secondary)";
                            e.currentTarget.style.backgroundColor =
                                "transparent";
                        }
                    }}
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
            {#if CurrentTabContent}
                <CurrentTabContent />
            {:else}
                <div
                    class="flex items-center justify-center h-full"
                    style="color: var(--color-text-disabled)"
                >
                    No content available for this tab
                </div>
            {/if}
        </section>
    </div>
</BaseModal>
