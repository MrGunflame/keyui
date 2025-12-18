<script lang="ts">
  import Header from './Header.svelte';
  import Api from './Api.svelte';
  import { Client } from './api';
  import CodeBlock from '$lib/CodeBlock.svelte';
  import ProofTree from '$lib/components/ProofTree.svelte';
  import GoalsPanel from '$lib/components/GoalsPanel.svelte';
  import Sequent from '$lib/panel/Sequent.svelte';
  import Panel from '$lib/panel/Panel.svelte';
  import type{ProofId,NodeId} from './api';

  type ProofInfo= {proofId: string};

  type AppState = {
    client: Client,
    // Current proof state (key file state).
    proof: ProofId | null,
    // Currently selected node in the proof tree.
    active_node: NodeId | null,
  };

  let appState: AppState = $state({
    client: new Client(),
    proof: null,
    active_node:null,
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


  
  <div class="layout">
    <Panel>
      <ProofTree appState= {appState} />
    </Panel>
    <Panel>
      <Sequent appState={appState} />
    </Panel>
    <Panel>
      <GoalsPanel appState={appState} />
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
