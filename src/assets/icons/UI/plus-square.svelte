<script lang="ts">
    import { currentTheme } from "@stores/theme";
    interface Props {
        size: string;
        clickable: boolean;
    }
    let current_theme = $derived($currentTheme);
    let { size, clickable = true }: Props = $props();
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    fill={current_theme?.text.secondary || "#a9a8a9"}
    viewBox="0 0 256 256"
    class="plus-square-icon"
    class:clickable
>
    <path d="M224,128a96,96,0,1,1-96-96A96,96,0,0,1,224,128Z" opacity="0.2"
    ></path>
    <path
        d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm48-88a8,8,0,0,1-8,8H136v32a8,8,0,0,1-16,0V136H88a8,8,0,0,1,0-16h32V88a8,8,0,0,1,16,0v32h32A8,8,0,0,1,176,128Z"
    ></path>
</svg>

<style>
    .plus-square-icon {
        transition:
            fill 0.2s ease,
            transform 0.1s ease;
    }

    .plus-square-icon.clickable {
        cursor: pointer;
    }

    .plus-square-icon.clickable:hover {
        fill: var(--color-accent-hover, #6366f1);
        transform: scale(1.05);
    }

    .plus-square-icon.clickable:active {
        transform: scale(0.95);
    }
</style>
