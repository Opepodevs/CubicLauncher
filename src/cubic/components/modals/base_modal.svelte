<script lang="ts">
    import Close from "@assets/icons/UI/close.svelte";
    import { currentTheme } from "@stores/theme";

    let { isOpen = $bindable(), title, children } = $props();

    const closeModal = () => (isOpen = false);

    const modalTransition = (node: Element, { duration = 300 } = {}) => ({
        duration,
        css: (t: number) => `
            opacity: ${t};
            transform: translateY(${(1 - t) * -300}px);
        `,
    });
</script>

{#if isOpen}
    <div class="modal-container">
        <div
            class="modal"
            style:background-color={$currentTheme?.background}
            style:color={$currentTheme?.text.primary}
            style:border="2px solid {$currentTheme?.border.default}"
            transition:modalTransition={{ duration: 400 }}
        >
            <header class="modal-header">
                <h2 class="modal-title">{title}</h2>
                <button class="close-btn" onclick={closeModal}>
                    <Close size="2rem" />
                </button>
            </header>
            <hr style:border-top="1px solid {$currentTheme?.border.default}" />
            <div class="modal-content">
                {@render children?.()}
            </div>
        </div>
    </div>
{/if}

<style lang="scss">
    .modal-container {
        display: flex;
        justify-content: center;
    }

    .modal {
        position: absolute;
        top: 35px;
        padding: 1rem 2rem 2rem;
        border-radius: 0 0 10px 10px;
        max-width: 960px;
        width: 100%;
        box-shadow:
            0 2px 4px rgba(0, 0, 0, 0.1),
            2px 6px 10px rgba(0, 0, 0, 0.15),
            4px 12px 20px rgba(0, 0, 0, 0.2);
        display: flex;
        flex-direction: column;
        z-index: 40;
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;

        .modal-title {
            font-size: 1.5rem;
            font-weight: 600;
            margin: 0;
        }

        .close-btn {
            background: none;
            border: none;
            cursor: pointer;
            padding: 0;

            &:hover {
                color: #ff5f5f;
            }
        }
    }

    hr {
        border: none;
        margin: 0.5rem 0 1rem;
    }

    .modal-content {
        flex: 1;
    }
</style>
