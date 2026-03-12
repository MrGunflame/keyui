<script lang="ts">
    import type { StrategyOptions } from "./api";

    type Props = {
        // TODO: How can we name this type?
        appState: any;
        onError: (err: string) => void;
    };

    let { appState, onError }: Props = $props();

    async function autoProof() {
        if (!appState.proof) {
            return;
        }

        const options: StrategyOptions = {
            method: null,
            dep: null,
            query: null,
            nonLinArith: null,
            maxSteps: 1_000_000,
            stopMode: null,
        };

        try {
            const status = await appState.client.proofAuto(
                appState.proof,
                options,
            );

            console.debug(status);

            // Proof tree nodes have changed; reload the tree.
            appState.proofTreeChanged.notify();
        } catch (err: any) {
            onError(err.toString());
        }
    }
</script>

<button class="play" onclick={autoProof} disabled={!appState.proof}>
    ▶ Auto Proof
</button>

<style>
    .play {
        padding: 8px 12px;
        border: none;
        cursor: pointer;
        border-radius: 6px;
        background-color: white;
        color: black;
    }

    .play:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
</style>
