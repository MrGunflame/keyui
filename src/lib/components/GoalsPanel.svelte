<script lang="ts">
  import type { NodeDesc } from "../../routes/api";

  let { appState } = $props();

  let openGoals = $state<number>(0);
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function loadOpenGoals() {
    error = null;

    if (!appState.proof) {
      openGoals = 0;
      return;
    }

    loading = true;
    try {
      console.log("GoalsPanel:proof=",appState.proof);
      const goals: NodeDesc[] = await appState.client.proofGoals(appState.proof, true, true);// onlyOpened=true, onlyEnabled=true
      console.log("GoalsPanel:goals=", goals);
      openGoals = goals.length;
    } catch (e) {
      error = String(e);
      openGoals = 0;
    } finally {
      loading = false;
    }
  }

  // Reload whenever a new proof is loaded
  $effect(() => {
    void loadOpenGoals();
  });
</script>

<div class="panel">
  <h3>Goals</h3>

  {#if !appState.proof}
    <p>Open goals: 0</p>
  {:else if loading}
    <p style="opacity:0.7;">Loading open goals…</p>
  {:else if error}
    <p style="color:#ff6b6b;">Error: {error}</p>
  {:else}
    <p>Open goals: {openGoals}</p>
  {/if}
</div>


