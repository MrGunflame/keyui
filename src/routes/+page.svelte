<script lang="ts">
  import Header from './Header.svelte';
  import Api from './Api.svelte';
  import { Client } from './api';
  import CodeBlock from '$lib/CodeBlock.svelte';
  import ProofTree from '$lib/components/ProofTree.svelte';
  import CurrentGoal from '$lib/components/CurrentGoal.svelte';
  import GoalsPanel from '$lib/components/GoalsPanel.svelte';
  import TermTree from '$lib/components/TermTree.svelte';

  type ProofId = {
    proofId: string;
  };

  type AppState = {
    client: Client,
    proofs: ProofId[],
  };

  let appState: AppState = $state({
    client: new Client(),
    proofs: [],
  });

  const rustExample = `
fn main() {
    println!("Hello from Rust + Tauri!");
}
`;

</script>

<main class="container">
  <Header {appState} />
  <!-- <Api /> -->

  {#each appState.proofs as proof}
    <span>{proof.proofId}</span>
  {/each}
  
 <div class="layout">
    <ProofTree />
    <CurrentGoal />
    <TermTree />    
    <GoalsPanel />
</div>

  
  <section class="code-section">
    <h2>Rust example</h2>
    <CodeBlock language="rust" code={rustExample} />
  </section>
</main>

<style>
  .layout {
    display: grid;
    grid-template-columns: 250px 1fr 250px;
    gap: 10px;
    min-height: calc(100vh - 60px); /* subtract header height */
}
</style>
