<script lang="ts">
    import Controller from "@assets/icons/UI/controller.svelte";
    import PlusSquare from "@assets/icons/UI/plus-square.svelte";
    import Logo from "@assets/Logo.svelte";
    import { currentTheme } from "@stores/theme";
    import { t } from "@stores/language";
    import { appStore } from "@stores/launcher";
    import { invoke } from "@tauri-apps/api/core";
    const current_theme = $derived($currentTheme);
</script>

<div class="launcher-container">
    <div class="launcher-content">
        <div class="logo-container">
            <Logo className="w-full h-full" />
        </div>

        <h1
            class="launcher-title"
            style="color: {current_theme?.text.primary};"
        >
            {$t("welcome.title")}
        </h1>

        <p
            class="launcher-description"
            style="color: {current_theme?.text.secondary};"
        >
            {$t("welcome.subtitle")}
        </p>

        <div class="buttons-grid">
            <button
                onclick={appStore.handle_new_instance_modal}
                class="btn-create"
                style="
                    background-color: {current_theme?.button.base};
                    color: {current_theme?.text.primary};
                    border-color: {current_theme?.border.default};
                "
            >
                <PlusSquare size="2.25rem" clickable={true} />
                <span>{$t("welcome.create_new_instance")}</span>
            </button>
            <button
                onclick={() => {
                    invoke("create_example_theme");
                }}>xd</button
            >
            <button
                class="btn-play"
                onclick={() => console.log("Play recent")}
                style="
                    background-color: {current_theme?.background};
                    color: {current_theme?.text.secondary};
                    border: 1px solid {current_theme?.border.default};
                "
            >
                <Controller />
                <span>{$t("welcome.play_recent")}</span>
            </button>
        </div>
    </div>
</div>

<style>
    .launcher-container {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100%;
        padding: 2rem;
    }

    .launcher-content {
        text-align: center;
        max-width: 32rem;
        width: 100%;
    }

    .logo-container {
        width: 6rem;
        height: 6rem;
        margin: 0 auto 1.5rem auto;
    }

    .launcher-title {
        font-size: 1.875rem;
        font-weight: 700;
        margin-bottom: 1rem;
        color: var(--text-primary, #1f2937);
    }

    .launcher-description {
        font-size: 1.125rem;
        margin-bottom: 2rem;
        color: var(--text-secondary, #6b7280);
    }

    .buttons-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1.5rem;
        margin-bottom: 2rem;
    }

    .btn-create {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0.5rem;
        gap: 0.5rem;
        border-radius: 0.75rem;
        cursor: pointer;
        transition: all 0.2s ease;
        border: 1px solid;
        font-size: 0.875rem;
        font-weight: 500;
        background-color: var(--accent-base, #3b82f6);
        color: var(--text-secondary, #6b7280);
        border-color: var(--border-default, #e5e7eb);
    }

    .btn-create:hover {
        opacity: 0.9;
        transform: translateY(-1px);
    }

    .btn-create:active {
        transform: translateY(0);
    }

    .btn-play {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0.75rem;
        gap: 0.5rem;
        background-color: #292524;
        border-radius: 0.75rem;
        border: 1px solid #57534e;
        cursor: pointer;
        transition: all 0.2s ease;
        font-size: 0.875rem;
        font-weight: 500;
    }

    .btn-play:hover {
        transform: translateY(-5px);
    }

    .btn-play:active {
        transform: translateY(0);
    }

    /* Responsive */
    @media (max-width: 640px) {
        .launcher-container {
            padding: 1rem;
        }

        .buttons-grid {
            grid-template-columns: 1fr;
            gap: 1rem;
        }

        .launcher-title {
            font-size: 1.5rem;
        }

        .launcher-description {
            font-size: 1rem;
        }
    }
</style>
