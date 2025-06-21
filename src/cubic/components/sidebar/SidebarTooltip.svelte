<script lang="ts">
    import { isDarkTheme } from "../../stores/theme";
    export let text: string | null;
</script>

<div class="tooltip" class:dark={$isDarkTheme}>
    <div class="tooltip-content">
        <span>{text ?? "How did u got here?"}</span>
    </div>
    <!-- Arrow -->
    <div class="tooltip-arrow"></div>
</div>

<style>
    .tooltip {
        position: relative;
        margin-left: 0.5rem;
        background: var(--color-surface);
        color: var(--color-text-primary);
        font-size: 0.75rem;
        padding: 0.5rem 0.75rem;
        border-radius: 0.375rem;
        border: 1px solid var(--color-border-default);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        white-space: nowrap;
        backdrop-filter: blur(8px);
        transition: all 0.2s ease;
        z-index: 60;
    }

    .tooltip.dark {
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    }

    .tooltip-content {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .tooltip-arrow {
        position: absolute;
        left: -6px;
        top: 50%;
        transform: translateY(-50%);
        width: 0;
        height: 0;
        border-top: 6px solid transparent;
        border-bottom: 6px solid transparent;
        border-right: 6px solid var(--color-surface);
    }

    .tooltip-arrow::before {
        content: "";
        position: absolute;
        left: 1px;
        top: -6px;
        width: 0;
        height: 0;
        border-top: 6px solid transparent;
        border-bottom: 6px solid transparent;
        border-right: 6px solid var(--color-border-default);
    }
</style>
