<script lang="ts">
  import type { NodeDesc } from "../../routes/api";

  let { appState } = $props();

  let goals = $state<[]>([]);

  async function loadOpenGoals(client, proof) {
    const goals = await client.proofGoals(proof, true, true);
    return goals;
  }

  // Reload whenever a new proof is loaded
  $effect(() => {
    if (appState.proof == null) {
      return;
    }
    
    loadOpenGoals(appState.client, appState.proof).then(res => {
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
