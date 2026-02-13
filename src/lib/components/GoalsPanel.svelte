<script lang="ts">
    import type { NodeDesc } from "../../routes/api";

    let { appState } = $props();

    let goals = $state<NodeDesc[]>([]);

    async function loadOpenGoals(client: any, proof: any) {
        const goals = await client.proofGoals(proof, true, true);
        return goals;
    }

    // Reload whenever a new proof is loaded
    $effect(() => {
        if (appState.proof == null) {
            return;
        }

        loadOpenGoals(appState.client, appState.proof).then((res) => {
            goals = res;
        });
    });
</script>

<div class="panel">
    <h3>Goals</h3>

    <div>Open goals: {goals.length}</div>
    <ul>
        {#each goals as goal}
            <li>{goal.description}</li>
        {/each}
    </ul>
</div>
<style>
.panel {
  background: var(--c-panel);
  color: var(--c-text);
  border: 1px solid var(--c-border);
  border-radius: 8px;
  padding: 10px;
  height: 100%;
}

.panel h3 {
  margin-top: 0;
}

.panel ul {
  margin: 0;
  padding-left: 18px;
}

.panel li {
  margin: 4px 0;
}
</style>
