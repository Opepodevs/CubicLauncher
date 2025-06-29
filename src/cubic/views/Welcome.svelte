<script lang="ts">
    import Controller from "@assets/icons/UI/controller.svelte";
    import PlusSquare from "@assets/icons/UI/plus-square.svelte";
    import Logo from "@assets/Logo.svelte";
    import { currentTheme } from "@stores/theme";
    import { t } from "@stores/language";
    import { appStore } from "@stores/launcher";

    // Usando $derived para crear valores computados reactivos
    const theme = $derived($currentTheme);
    const welcomeTitle = $derived($t("welcome.title"));
    const welcomeSubtitle = $derived($t("welcome.subtitle"));
    const createButtonText = $derived($t("welcome.create_new_instance"));
    const playButtonText = $derived($t("welcome.play_recent"));

    // Estilos computados usando $derived
    const titleStyle = $derived(`color: ${theme?.text.primary};`);
    const descriptionStyle = $derived(`color: ${theme?.text.secondary};`);
    const createButtonStyle = $derived(`
        background-color: ${theme?.button.base};
        color: ${theme?.text.primary};
        border-color: ${theme?.border.default};
    `);
    const playButtonStyle = $derived(`
        background-color: ${theme?.background};
        color: ${theme?.text.secondary};
        border: 1px solid ${theme?.border.default};
    `);

    // Funciones de manejo de eventos
    const handleNewInstance = () => {
        appStore.handleModal('newInstance');
    };

    const handlePlayRecent = () => {
        // todo: Terminar logica de botones
    };
</script>

<div class="launcher-container">
    <div class="launcher-content">
        <div class="logo-container">
            <Logo className="w-full h-full" />
        </div>

        <h1 class="launcher-title" style={titleStyle}>
            {welcomeTitle}
        </h1>

        <p class="launcher-description" style={descriptionStyle}>
            {welcomeSubtitle}
        </p>

        <div class="buttons-grid">
            <button
                onclick={handleNewInstance}
                class="btn-create"
                style={createButtonStyle}
            >
                <PlusSquare size="2.25rem" clickable={true} />
                <span>{createButtonText}</span>
            </button>

            <button
                onclick={handlePlayRecent}
                class="btn-play"
                style={playButtonStyle}
            >
                <Controller />
                <span>{playButtonText}</span>
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
    }

    .launcher-description {
        font-size: 1.125rem;
        margin-bottom: 2rem;
    }

    .buttons-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1.5rem;
        margin-bottom: 2rem;
    }

    .btn-create,
    .btn-play {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0.75rem;
        gap: 0.5rem;
        border-radius: 0.75rem;
        cursor: pointer;
        transition: all 0.2s ease;
        border: 1px solid;
        font-size: 0.875rem;
        font-weight: 500;
    }

    .btn-create {
        padding: 0.5rem;
    }

    .btn-create:hover,
    .btn-play:hover {
        opacity: 0.9;
        transform: translateY(-1px);
    }

    .btn-play:hover {
        transform: translateY(-5px);
    }

    .btn-create:active,
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
