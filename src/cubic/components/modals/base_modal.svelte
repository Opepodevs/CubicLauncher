<script lang="ts">
    import Close from "@assets/icons/UI/close.svelte";
    import { currentTheme } from "@stores/theme";
    // Props - including children snippet
    let { isOpen = $bindable(), title, children } = $props();

    // Function to close modal
    function closeModal() {
        isOpen = false;
    }

    // Función personalizada para la transición
    function modalTransition(node: Element, { duration = 300 }) {
        return {
            duration,
            css: (t: number, u: number) => {
                // t va de 0 a 1 (entrada), u va de 1 a 0 (salida)
                const opacity = t;
                const translateY = u * -300; // -300px cuando u=1 (inicio salida), 0 cuando u=0 (fin entrada)

                return `
                    opacity: ${opacity};
                    transform: translateY(${translateY}px);
                `;
            },
        };
    }
</script>

<div class="flex justify-center">
    {#if isOpen}
        <div
            class="modal"
            style="
            background-color: {$currentTheme?.background};
            color: {$currentTheme?.text.primary};
            border: 2px solid {$currentTheme?.border.default};
            "
            transition:modalTransition={{ duration: 400 }}
        >
            <header class="modal-header">
                <h2 class="modal-title">{title}</h2>
                <button onclick={closeModal}>
                    <Close size="2rem" />
                </button>
            </header>
            <hr />
            <div>
                {@render children?.()}
            </div>
        </div>
    {/if}
</div>

<style lang="scss">
    .modal {
        position: absolute;
        top: 35px;
        padding: 1rem 2rem 2rem 2rem;
        border-radius: 0 0 10px 10px;
        max-width: 960px;
        width: 100%;
        color: white;
        box-shadow:
            0 2px 4px rgba(0, 0, 0, 0.1),
            2px 6px 10px rgba(0, 0, 0, 0.15),
            4px 12px 20px rgba(0, 0, 0, 0.2);
        background-color: #2d2d2d;
        display: flex;
        flex-direction: column;
        text-align: center;
        z-index: 40;
    }

    /* Header */
    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .modal-title {
        font-size: 1.5rem;
        font-weight: 600;
    }

    .modal-header .close-btn:hover {
        color: #ff5f5f;
    }

    /* Separador */
    hr {
        border: none;
        border-top: 1px solid #555;
        margin: 0.5rem 0 1rem;
    }

    /* Botón Cerrar */
    .close-btn {
        margin-top: 1rem;
        background-color: #ff5f5f;
        color: white;
        border: none;
        padding: 0.5rem 1.2rem;
        border-radius: 6px;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .close-btn:hover {
        background-color: #e14e4e;
    }
</style>
