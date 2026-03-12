<script lang="ts">
    import type { NodeId, NodeTextDesc, PrintOptions } from "$lib/api";
    import AppState from "$lib/AppState.svelte";

    type Props = {
        appState: AppState;
        // ID of the node, or null if this is a virtual node.
        nodeId: NodeId | null;
        nodeName: string | null;
    };

    let { appState, nodeId, nodeName }: Props = $props();

    type State = {
        loading: boolean;
        sequent: string | null;
        taclet: string | null;
        error: string | null;
        pruning: boolean;
        pruneError: string | null;
        pruneSuccess: boolean;
    };

    let state = $state<State>({
        loading: true,
        sequent: null,
        taclet: null,
        error: null,
        pruning: false,
        pruneError: null,
        pruneSuccess: false,
    });

    async function pruneTo(node: NodeId) {
        state.pruning = true;
        state.pruneError = null;
        state.pruneSuccess = false;

        try {
            await appState.client.proofPruneTo(node);
            state.pruneSuccess = true;
        } catch (err: any) {
            state.pruneError = err.toString();
        } finally {
            state.pruning = false;
        }

        appState.proofTreeChanged.notify();
    }

    if (nodeId != null) {
        const options: PrintOptions = {
            unicode: false,
            width: 120,
            indentation: 0,
            pure: false,
            termLabels: true,
        };

        appState.client
            .goalPrint(nodeId, options)
            .then((res: NodeTextDesc) => {
                state.loading = false;
                state.sequent = res.result;
                state.taclet = res.tacletApplicationInfo;
            })
            .catch((err: any) => {
                state.loading = false;
                state.error = err.toString();
            });
    }
</script>

{#if nodeName?.toLowerCase() === "closed goal"}
    <div class="ctx-simple">A closed goal</div>
{:else}
    <div class="ctx-title">Taclet info</div>

    <div class="ctx-content">
        <div class="ctx-row">
            <div class="ctx-label">Rule</div>
            <div class="ctx-value">
                {nodeName ?? "-"}
            </div>
        </div>

        <div class="ctx-sep"></div>

        <div class="ctx-label">Applied on</div>

        {#if state.loading}
            <div class="ctx-mono loading">Loading…</div>
        {:else if state.error}
            <div class="ctx-mono error">{state.error}</div>
        {:else}
            <div class="ctx-mono">
                {state.sequent ?? "-"}
            </div>
        {/if}
        <div class="ctx-sep"></div>

        {#if state.taclet != null}
            <div class="ctx-mono">
                {state.taclet}
            </div>
        {/if}

        <button
            class="ctx-prune-btn"
            disabled={state.pruning || state.pruneSuccess}
            onclick={() => nodeId && pruneTo(nodeId)}
        >
            {#if state.pruning}
                Pruning…
            {:else if state.pruneSuccess}
                ✓ Pruned
            {:else}
                ✂ Prune to here
            {/if}
        </button>

        {#if state.pruneError}
            <div class="ctx-mono error">
                {state.pruneError}
            </div>
        {/if}
    </div>
{/if}

<style>
    .ctx-simple {
        padding: 10px 12px;
        font-size: 13px;
        font-weight: 600;
        opacity: 0.95;
        white-space: nowrap;
    }

    .ctx-title {
        font-size: 12px;
        font-weight: 700;
        letter-spacing: 0.2px;
        opacity: 0.9;
        padding: 6px 2px 10px 2px;
        border-bottom: 1px solid rgba(255, 255, 255, 0.08);
        margin-bottom: 10px;
    }

    .ctx-content {
        display: grid;
        gap: 10px;
    }

    .ctx-row {
        display: grid;
        grid-template-columns: 92px 1fr;
        gap: 10px;
        align-items: baseline;
    }

    .ctx-label {
        font-size: 12px;
        opacity: 0.7;
    }

    .ctx-value {
        font-size: 13px;
        font-weight: 650;
    }

    .ctx-sep {
        height: 1px;
        background: rgba(255, 255, 255, 0.08);
        margin: 2px 0;
    }

    .ctx-mono {
        font-family:
            ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
            "Liberation Mono", "Courier New", monospace;
        font-size: 12px;
        line-height: 1.35;

        white-space: pre-wrap;
        word-break: break-word;

        padding: 8px 10px;
        border-radius: 10px;
        background: rgba(255, 255, 255, 0.06);
        border: 1px solid rgba(255, 255, 255, 0.08);
    }

    .ctx-mono.loading {
        opacity: 0.75;
    }

    .ctx-mono.error {
        border-color: rgba(255, 120, 120, 0.35);
        background: rgba(255, 120, 120, 0.1);
    }

    .ctx-prune-btn {
        width: 100%;
        padding: 8px 12px;
        border-radius: 8px;
        border: 1px solid rgba(255, 160, 80, 0.4);
        background: rgba(255, 160, 80, 0.1);
        color: rgba(255, 180, 100, 0.95);
        font-size: 13px;
        font-weight: 650;
        cursor: pointer;
        transition:
            background 120ms ease,
            border-color 120ms ease,
            opacity 120ms ease;
        text-align: center;
    }

    .ctx-prune-btn:hover:not(:disabled) {
        background: rgba(255, 160, 80, 0.2);
        border-color: rgba(255, 160, 80, 0.7);
    }

    .ctx-prune-btn:disabled {
        cursor: default;
        opacity: 0.65;
    }
</style>
