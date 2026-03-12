<script lang="ts">
    import type { TreeNodeDesc } from "$lib/api";
    import ContextMenu from "$lib/components/ContextMenu.svelte";
    import NodeInfo from "$lib/components/proof_tree/NodeInfo.svelte";

    let { appState } = $props();

    type Node =
        | { kind: "real"; node: TreeNodeDesc; depth: number }
        | { kind: "virtual"; label: string; depth: number };

    let nodes = $state<Node[]>([]);
    let searchQuery = $state("");
    let collapsedNodes = $state<Set<string>>(new Set());

    //State for the context menu
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

    //Opens the context menu when user right-clicks a node
    function openCtxMenu(e: MouseEvent, node: TreeNodeDesc) {
        e.preventDefault();
        ctxMenu = {
            open: true,
            x: e.clientX,
            y: e.clientY,
            node,
        };
    }

    function statusFromName(name: string) {
        const up = name.toUpperCase();
        if (up.includes("OPEN")) return "open";
        if (up.includes("CLOSED")) return "closed";
        return "unknown";
    }

    // Returns the status of a virtual node's branch based on its descendants
    function virtualStatus(index: number): "open" | "closed" | "mixed" {
        let hasOpen = false;
        let hasClosed = false;
        const currentDepth = nodes[index]?.depth ?? 0;

        for (let i = index + 1; i < nodes.length; i++) {
            const item = nodes[i];
            if (item.depth <= currentDepth) break;
            if (item.kind !== "real") continue;

            const s = statusFromName(item.node.name);
            if (s === "open") hasOpen = true;
            if (s === "closed") hasClosed = true;
            if (hasOpen && hasClosed) return "mixed";
        }

        if (hasOpen) return "open";
        if (hasClosed) return "closed";
        return "open"; // fallback: treat unknown as open
    }

    function isLeaf(index: number) {
        const currentDepth = nodes[index]?.depth ?? 0;
        const nextDepth = nodes[index + 1]?.depth ?? -1;
        return nextDepth <= currentDepth;
    }

    function isActive(node: TreeNodeDesc) {
        return Number(appState.active_node?.nodeId) === Number(node.id.nodeId);
    }

    function toggleCollapse(nodeId: string) {
        if (collapsedNodes.has(nodeId)) {
            collapsedNodes.delete(nodeId);
        } else {
            collapsedNodes.add(nodeId);
        }
        collapsedNodes = new Set(collapsedNodes);
    }

    function isCollapsed(nodeId: string) {
        return collapsedNodes.has(nodeId);
    }

    function isHiddenByCollapse(index: number): boolean {
        const currentNode = nodes[index];
        if (!currentNode) return false;

        for (let i = index - 1; i >= 0; i--) {
            const ancestor = nodes[i];

            if (ancestor.depth < currentNode.depth) {
                if (ancestor.kind === "virtual") {
                    continue;
                }

                const ancestorNodeId = ancestor.node.id.nodeId;
                if (collapsedNodes.has(ancestorNodeId)) {
                    return true;
                }
            }
        }

        return false;
    }

    function matchesSearch(node: TreeNodeDesc): boolean {
        if (!searchQuery.trim()) return true;

        const query = searchQuery.toLowerCase();
        const nodeName = node.name.toLowerCase();
        const nodeId = node.id.nodeId.toString();

        return nodeName.includes(query) || nodeId.includes(query);
    }

    function hasMatchingDescendant(index: number): boolean {
        if (!searchQuery.trim()) return false;

        const currentDepth = nodes[index]?.depth;

        for (let i = index + 1; i < nodes.length; i++) {
            const node = nodes[i];

            if (node.depth <= currentDepth) break;

            if (node.kind === "virtual") return true;

            if (matchesSearch(node.node)) return true;
        }
        return false;
    }

    function shouldShowNode(index: number): boolean {
        const node = nodes[index];
        if (!node) return false;

        if (isHiddenByCollapse(index)) return false;

        if (!searchQuery.trim()) return true;

        if (node.kind === "virtual") return true;

        return matchesSearch(node.node) || hasMatchingDescendant(index);
    }

    async function loadTreeCollapsed(client: any, proof: any): Promise<Node[]> {
        const out: Node[] = [];

        const childrenCache = new Map<number, TreeNodeDesc[]>();

        const idOf = (n: TreeNodeDesc) => Number(n.id.nodeId);

        async function getChildren(
            node: TreeNodeDesc,
        ): Promise<TreeNodeDesc[]> {
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
            return;
        }

        loadTreeCollapsed(appState.client, appState.proof).then((n) => {
            nodes = n;
        });
    });
</script>

<div class="proof-tree-container">
    <h3>Proof Tree</h3>

    <div class="search-container">
        <input
            type="text"
            class="search-input"
            placeholder="Suche nach Name oder ID..."
            bind:value={searchQuery}
        />
        {#if searchQuery}
            <button class="clear-btn" onclick={() => (searchQuery = "")}
                >✕</button
            >
        {/if}
    </div>

    <ul class="node-list">
        {#each nodes as item, index}
            {#if shouldShowNode(index)}
                <li style="margin-left: {item.depth * 14}px;">
                    {#if item.kind === "real"}
                        <button
                            class="node {statusFromName(
                                item.node.name,
                            )} {isActive(item.node) ? 'active' : ''} {isLeaf(
                                index,
                            )
                                ? 'leaf'
                                : 'internal'}"
                            onclick={() =>
                                (appState.active_node = item.node.id)}
                            oncontextmenu={(e) => openCtxMenu(e, item.node)}
                        >
                            {#if !isLeaf(index)}
                                <!-- svelte-ignore a11y_click_events_have_key_events -->
                                <!-- svelte-ignore a11y_no_static_element_interactions -->
                                <span
                                    class="collapse-icon"
                                    onclick={(e) => {
                                        e.stopPropagation();
                                        toggleCollapse(item.node.id.nodeId);
                                    }}
                                >
                                    {isCollapsed(item.node.id.nodeId)
                                        ? "▶"
                                        : "▼"}
                                </span>
                            {/if}

                            {Number(item.node.id.nodeId)}: {item.node.name}
                        </button>
                    {:else}
                        <div class="virtual {virtualStatus(index)}">
                            {item.label}
                        </div>
                    {/if}
                </li>
            {/if}
        {/each}
    </ul>

    <ContextMenu
        open={ctxMenu.open}
        positionX={ctxMenu.x}
        positionY={ctxMenu.y}
        onClose={() => (ctxMenu.open = false)}
    >
        <div class="ctx-menu">
            <NodeInfo
                {appState}
                nodeId={ctxMenu.node!.id}
                nodeName={ctxMenu.node!.name}
            />
        </div>
    </ContextMenu>
</div>

<style>
    .proof-tree-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        overflow: hidden;
    }
    .proof-tree-container h3 {
        margin: 0 0 10px 0;
        flex-shrink: 0;
    }
    .proof-tree-container {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 0 10px;
    }
    .search-container {
        position: relative;
        margin: 10px 0;
    }
    .search-input {
        width: 100%;
        padding: 10px 35px 10px 12px;
        background: var(--c-hover-bg);
        border: 1px solid rgba(255, 255, 255, 0.12);
        border-radius: 8px;
        font-size: 14px;
        box-sizing: border-box;
    }
    .search-input:focus {
        outline: none;
        border-color: rgba(80, 200, 120, 0.5);
    }

    .search-input::placeholder {
        /* color: rgba(255, 255, 255, 0.4); */
        color: var(--c-text);
        opacity: 0.4;
    }

    .clear-btn {
        position: absolute;
        right: 8px;
        top: 50%;
        transform: translateY(-50%);
        background: transparent;
        border: none;
        color: var(--c-text);
        cursor: pointer;
        padding: 4px 8px;
        font-size: 16px;
        opacity: 0.5;
    }

    .clear-btn:hover {
        color: var(--c-text);
        opacity: 1;
    }

    .node-list {
        flex: 1;
        overflow-y: auto;
        overflow-x: hidden;
        list-style: none;
        padding: 0;
        margin: 0;
    }

    .node {
        width: 100%;
        text-align: left;
        word-wrap: break-word;
        white-space: normal;
        display: flex;
        align-items: center;
        gap: 8px;

        color: var(--c-text);
        border: 1px solid rgba(255, 255, 255, 0.08);
        padding: 8px 10px;
        margin: 6px 0;
        border-radius: 8px;

        background: var(--c-hover-bg);
        font-weight: 600;
        cursor: pointer;
        transition:
            border-color 120ms ease,
            transform 120ms ease;
    }

    .node:hover {
        border-color: rgba(255, 255, 255, 0.22);
        transform: translateY(-1px);
    }

    .virtual {
        width: 100%;
        padding: 6px 10px;
        margin: 6px 0;
        border-radius: 8px;
        border: 1px dashed rgba(255, 255, 255, 0.2);
        opacity: 0.8;
        background: rgba(255, 255, 255, 0.04);
        font-weight: 600;
    }

    .virtual.open {
        border-color: rgba(255, 100, 100, 0.45);
        background: rgba(255, 80, 80, 0.1);
    }

    .virtual.closed {
        border-color: rgba(80, 200, 120, 0.45);
        background: rgba(80, 200, 120, 0.1);
    }

    .virtual.mixed {
        border-color: rgba(255, 180, 60, 0.45);
        background: rgba(255, 180, 60, 0.08);
    }

    .collapse-icon {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 16px;
        height: 16px;
        font-size: 10px;
        opacity: 0.7;
        transition: opacity 0.2s;
    }

    .collapse-icon:hover {
        opacity: 1;
    }

    .node-content {
        flex: 1;
    }

    .open {
        background: var(--c-node-open);
    }
    .closed {
        background: var(--c-node-closed);
    }
    .unknown {
        background: var(--c-node-unknown);
    }

    .ctx-menu {
        min-width: 260px;
        max-width: 420px;
        padding: 10px 12px;
    }

    .node.active {
        outline: 2px solid rgba(80, 200, 120, 0.95);
        outline-offset: 2px;
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
