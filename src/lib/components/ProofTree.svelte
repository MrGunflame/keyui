<script lang="ts">
  import type { TreeNodeDesc } from "../../routes/api";

  let { appState } = $props();

  type Node =
    | { kind: "real"; node: TreeNodeDesc; depth: number }
    | { kind: "virtual"; label: string; depth: number };

  let nodes = $state<Node[]>([]);

  type CtxMenuState = {
    open: boolean;
    x: number;
    y: number;
    node: TreeNodeDesc | null;
  };

  let ctxMenu = $state<CtxMenuState>({
    open: false,
    x: 0,
    y: 0,
    node: null,
  });

  function openCtxMenu(e: MouseEvent, node: TreeNodeDesc) {
    e.preventDefault();
    ctxMenu = {
      open: true,
      x: e.clientX,
      y: e.clientY,
      node,
    };
  }

  function closeCtxMenu() {
    ctxMenu.open = false;
  }

  function statusFromName(name: string) {
    const up = name.toUpperCase();
    if (up.includes("OPEN")) return "open";
    if (up.includes("CLOSED")) return "closed";
    return "unknown";
  }

  function isLeaf(index: number) {
    const currentDepth = nodes[index]?.depth ?? 0;
    const nextDepth = nodes[index + 1]?.depth ?? -1;
    return nextDepth <= currentDepth;
  }

  function isActive(node: TreeNodeDesc) {
    return Number(appState.active_node?.nodeId) === Number(node.id.nodeId);
  }

  // DEV ONLY: keep FALSE for PR
  const DEMO_TREE = false;

  function makeDemoNodes(): Node[] {
    const fake = (nodeId: number, name: string) =>
      ({ id: { nodeId }, name } as unknown as TreeNodeDesc);

    return [
      { kind: "real", node: fake(1, "OPEN Root"), depth: 0 },
      { kind: "real", node: fake(2, "OPEN Step 1"), depth: 0 },
      { kind: "real", node: fake(3, "OPEN Step 2"), depth: 0 },
      { kind: "real", node: fake(4, "OPEN Step 3"), depth: 0 },

      { kind: "real", node: fake(5, "OPEN Branching Rule"), depth: 0 },
      { kind: "virtual", label: "5.1", depth: 1 },
      { kind: "real", node: fake(6, "CLOSED Left leaf"), depth: 2 },
      { kind: "virtual", label: "5.2", depth: 1 },
      { kind: "real", node: fake(7, "OPEN Right path"), depth: 2 },
      { kind: "real", node: fake(8, "CLOSED Right leaf"), depth: 2 },
    ];
  }

  async function loadTreeCollapsed(client: any, proof: any): Promise<Node[]> {
    const out: Node[] = [];

    const childrenCache = new Map<number, TreeNodeDesc[]>();

    const idOf = (n: TreeNodeDesc) => Number(n.id.nodeId);

    async function getChildren(node: TreeNodeDesc): Promise<TreeNodeDesc[]> {
      const id = idOf(node);
      if (childrenCache.has(id)) return childrenCache.get(id)!;
      const kids = await client.proofTreeChildren(proof, node.id);
      childrenCache.set(id, kids);
      return kids;
    }

    async function emit(node: TreeNodeDesc, depth: number): Promise<void> {
      const nodeId = idOf(node);

      out.push({ kind: "real", node, depth });

      const kids = await getChildren(node);

      // Linear chain => keep SAME depth
      if (kids.length === 1) {
        await emit(kids[0], depth);
        return;
      }

      // Branch => create virtual nodes n.1, n.2, ...
      if (kids.length >= 2) {
        for (let i = 0; i < kids.length; i++) {
          const label = `${nodeId}.${i + 1}`;
          out.push({ kind: "virtual", label, depth: depth + 1 });
          await emit(kids[i], depth + 2);
        }
      }
    }

    const root = await client.proofTreeRoot(proof);
    await emit(root, 0);

    return out;
  }

  let waker = appState.proofTreeChanged.subscribe();

  $effect(() => {
    $waker;

    if (appState.proof == null) {
      nodes = DEMO_TREE ? makeDemoNodes() : [];
      return;
    }

    loadTreeCollapsed(appState.client, appState.proof).then((n) => {
      nodes = n;
    });
  });
</script>

<div>
  <h3>Proof Tree</h3>
  <ul class="node-list">
    {#each nodes as item, index}
      <li style="margin-left: {item.depth * 14}px;">
        {#if item.kind === "real"}
          <button
            class="node {statusFromName(item.node.name)} {isActive(item.node) ? "active" : ""} {isLeaf(index) ? "leaf" : "internal"}"
            onclick={() => (appState.active_node = item.node.id)}
            oncontextmenu={(e) => openCtxMenu(e, item.node)}
          >
            {Number(item.node.id.nodeId)}: {item.node.name}
          </button>
        {:else}
          <div class="virtual">{item.label}</div>
        {/if}
      </li>
    {/each}
  </ul>

  {#if ctxMenu.open}
    <div class="ctx-backdrop" onclick={closeCtxMenu}>
      <div
        class="ctx-menu"
        style="left: {ctxMenu.x}px; top: {ctxMenu.y}px;"
        onclick={(e) => e.stopPropagation()}
      >
        <div class="ctx-title">Node {ctxMenu.node?.id.nodeId}</div>
        <button class="ctx-item" disabled>Action A</button>
        <button class="ctx-item" disabled>Action B</button>
        <button class="ctx-item" disabled>Action C</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .node-list {
    height: 100%;
    overflow-y: auto;
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .node {
    width: 100%;
    text-align: left;

    color: white;
    border: 1px solid rgba(255, 255, 255, 0.08);
    padding: 8px 10px;
    margin: 6px 0;
    border-radius: 8px;

    background: #2b2b2b;
    font-weight: 600;
    cursor: pointer;
  }

  .node:hover {
    border-color: rgba(255, 255, 255, 0.22);
  }

  .virtual {
    width: 100%;
    padding: 6px 10px;
    margin: 6px 0;
    border-radius: 8px;
    border: 1px dashed rgba(255, 255, 255, 0.20);
    color: rgba(255, 255, 255, 0.8);
    background: rgba(255, 255, 255, 0.04);
    font-weight: 600;
  }

  .open { background: #662222; }
  .closed { background: #225522; }
  .unknown { background: #333; }

  .ctx-backdrop {
    position: fixed;
    inset: 0;
    z-index: 999;
  }

  .ctx-menu {
    position: fixed;
    z-index: 1000;
    min-width: 220px;
    background: #1f1f1f;
    border: 1px solid #444;
    border-radius: 8px;
    padding: 8px;
    box-shadow: 0 10px 25px rgba(0,0,0,0.4);
  }

  .ctx-title {
    font-size: 12px;
    opacity: 0.8;
    padding: 6px 8px;
    border-bottom: 1px solid #333;
    margin-bottom: 6px;
  }

  .ctx-item {
    width: 100%;
    text-align: left;
    padding: 8px;
    border: 0;
    background: transparent;
    color: white;
    cursor: not-allowed;
    border-radius: 6px;
    opacity: 0.7;
  }

  .ctx-item:hover {
    background: #2b2b2b;
  }

  .node.active {
    outline: 2px solid rgba(80, 200, 120, 0.95);
    outline-offset: 2px;
  }

  .node.open {
    background: #6a2525;
  }

  .node.closed {
    background: #1f4f2a;
    opacity: 0.55;
  }

  .node.unknown {
    background: #333;
  }

  .node.leaf {
    border-left: 6px solid rgba(255, 255, 255, 0.16);
  }

  .node.internal {
    border-left: 6px solid rgba(255, 255, 255, 0.34);
  }

  .node.closed.active {
    opacity: 0.9;
  }
</style>
