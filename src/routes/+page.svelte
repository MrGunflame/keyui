<script lang="ts">
    import Header from "$lib/components/header/Header.svelte";
    import { Client } from "$lib/api";
    import CodeBlock from "$lib/components/CodeBlock.svelte";
    import ProofTree from "$lib/main/ProofTree.svelte";
    import GoalsPanel from "$lib/main/GoalsPanel.svelte";
    import Sequent from "$lib/main/Sequent.svelte";
    import Panel from "$lib/main/Panel.svelte";
    import type { ProofId, NodeId } from "../lib/api";
    import Modal from "$lib/Modal.svelte";

    import { ReactiveSignal } from "$lib/reactive";
    import AutoProofButton from "$lib/AutoProofButton.svelte";

    type AppState = {
        client: Client;
        // Current proof state (key file state).
        proof: ProofId | null;
        // Currently selected node in the proof tree.
        active_node: NodeId | null;
        // Subscriber called whenever the proof tree changes.
        proofTreeChanged: ReactiveSignal;
    };

    let appState: AppState = $state({
        client: new Client(),
        proof: null,
        active_node: null,
        proofTreeChanged: new ReactiveSignal(),
    });

    let errorState: string | null = $state(null);
</script>

<main class="main">
    <div class="header">
        <Header {appState} onError={(error: any) => (errorState = error)} />

        <div class="actions">
            <AutoProofButton {appState} onError={(err) => (errorState = err)} />
        </div>
    </div>

    {#if errorState}
        <Modal open={true} on:close={() => (errorState = null)}>
            <h2>Error</h2>
            <pre class="error-state-pre">
<code>{errorState}</code>
</pre>
        </Modal>
    {/if}

    <div class="main-section">
        <div class="flex-1">
            <Panel>
                <ProofTree {appState} />
            </Panel>
        </div>
        <div class="flex-10">
            <Panel>
                <Sequent {appState} />
            </Panel>
        </div>
        <div class="flex-1">
            <Panel>
                <GoalsPanel {appState} />
            </Panel>
        </div>
    </div>
</main>

<style>
    .main {
        width: 100vw;
        height: 100vh;
        display: flex;
        flex-flow: column;

        background: var(--c-main-background);
        color: var(--c-text);
    }

    .header {
        flex-grow: 0;
        flex-shrink: 1;
        flex-basis: auto;
    }

    .main-section {
        flex-grow: 1;
        flex-shrink: 1;
        flex-basis: auto;

        display: flex;

        gap: 10px;
        margin: 10px;
        padding: 10px;

        min-width: 0;
        min-height: 0;
    }

    .actions {
        padding: 10px;
        display: flex;
        gap: 10px;
        align-items: center;
    }

    .flex-1 {
        flex-grow: 1;
    }

    .flex-10 {
        flex-grow: 10;
    }

    /* Don't let the <code> exceed <pre>. */
    .error-state-pre {
        display: flex;
        overflow: scroll;
    }
</style>
