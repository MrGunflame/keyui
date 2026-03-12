<script lang="ts" module>
    import type { Client, NodeId, ProofId } from "./api";
    import type { ReactiveSignal } from "./reactive";

    export type AppState = {
        client: Client;
        // Current proof state (key file state).
        proof: ProofId | null;
        // Currently selected node in the proof tree.
        active_node: NodeId | null;
        // Subscriber called whenever the proof tree changes.
        proofTreeChanged: ReactiveSignal;
    };
</script>
