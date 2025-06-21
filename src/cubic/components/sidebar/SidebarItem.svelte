<script lang="ts">
    import { type Component } from "svelte";

    interface Props {
        name: string;
        Icon: Component;
        onClick: () => void;
    }

    let { name, Icon, onClick }: Props = $props();

    const handleClick = () => {
        if (onClick) {
            onClick();
        }
    };
    const handleKeydown = (event: KeyboardEvent) => {
        if (event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            handleClick();
        }
    };
</script>

<div
    class="sidebar-item"
    onclick={handleClick}
    onkeydown={handleKeydown}
    role="button"
    tabindex="0"
>
    <Icon />
</div>

<style>
    .sidebar-item {
        background: var(--color-background);
        padding: 0.5rem;
        border-radius: 0.75rem;
        border: 1px solid var(--color-border-default);
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        flex-shrink: 0;
        z-index: 60;
    }

    .sidebar-item:hover {
        background: var(--color-accent-base);
        border-color: var(--color-accent-hover);
        transform: translateY(-1px);
    }

    .sidebar-item:active {
        transform: translateY(0);
        background: var(--color-accent-active);
    }

    .icon {
        width: 2.5rem;
        height: 2.5rem;
        border-radius: 0.75rem;
    }
</style>
