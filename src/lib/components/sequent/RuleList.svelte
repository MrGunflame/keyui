<script lang="ts">
    import type { TermActionDesc } from "../../../routes/api";

    type Props = {
        name: string;
        actions: TermActionDesc[];
        onApply: (action: TermActionDesc) => void;
    };

    const { name, actions, onApply }: Props = $props();
</script>

{#if actions.length > 0}
    <div class="rule-list">
        <h4>{name}</h4>
        <ul>
            {#each actions as action}
                <li>
                    <button onclick={() => onApply(action)}
                        >{action.displayName}</button
                    >
                </li>
            {/each}
        </ul>
    </div>
{/if}

<style>
    .rule-list {
        padding: 5px;
    }

    h4 {
        padding: 0;
        margin: 0;
    }

    ul {
        list-style: none;
        margin: 0;
        padding: 0;
    }

    li {
        padding: 1px;
    }
</style>
