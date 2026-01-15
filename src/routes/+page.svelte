<script lang="ts">
  import Header from './Header.svelte';
  import Api from './Api.svelte';
  import { Client } from './api';
  import CodeBlock from '$lib/CodeBlock.svelte';
  import ProofTree from '$lib/components/ProofTree.svelte';
  import GoalsPanel from '$lib/components/GoalsPanel.svelte';
  import Sequent from '$lib/panel/Sequent.svelte';
  import Panel from '$lib/panel/Panel.svelte';
  import type { ProofId, NodeId } from './api';
  import Modal from './Modal.svelte';

  import { ReactiveSignal } from '$lib/reactive.ts';
  import { writable, type Writable } from "svelte/store";

  type AppState = {
    client: Client,
    // Current proof state (key file state).
    proof: ProofId | null,
    // Currently selected node in the proof tree.
    active_node: NodeId | null,
    // Subscriber called whenever the proof tree changes.
    proofTreeChanged: ReactiveSignal, 
  };

  let appState: AppState = $state({
    client: new Client(),
    proof: null,
    active_node:null,
    proofTreeChanged: new ReactiveSignal(),
  });

  let errorState: string | null = $state(null);

  const rustExample = `
fn main() {
    println!("Hello from Rust + Tauri!");
}
`;

</script>

<main class="container">
  <Header {appState} onError={(error) => (errorState = error)} />
  <!-- <Api /> -->
   {#if errorState}
    <Modal open={true} on:close={() => (errorState = null)}>
      <h2>Error</h2>
      <pre>
        <code>{errorState}</code>
      </pre>
    </Modal>
  {/if}
  
  <div class="layout">
    <Panel>
      <ProofTree {appState} />
    </Panel>
    <Panel>
      <Sequent {appState} />
    </Panel>
    <Panel>
      <GoalsPanel {appState} />
    </Panel>
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
    height: 100vh;
    padding: 10px;
    background: #1e1e1e;
    color: white;
  }
</style>
