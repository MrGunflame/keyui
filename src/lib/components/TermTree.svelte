<script lang="ts">
    import type { TermActionDesc } from "../../routes/api";
    
    let { appState, sequent } = $props();

    type Span = {
        content: string;
        // All span indices that are associated with this span.
        // This includes all children.
        // These should be marked when the span is hovered.
        spans: number[];
        // The character index where the text for this span starts.
        textStart: number;
    };

    function expandTerms(seq: string, terms: NodeTextSpan[], startIndex: number, textStart: number): Span[] {
        terms.sort((a, b) => (a.start - b.start));

        let output = [];
        let outerSpans = [];

        let pos = 0;
        terms.forEach(term => {
            // Mark section between subterms.
            if (pos != term.start) {
                let s = seq.slice(pos, term.start);
                outerSpans.push(output.length);
                output.push({
                    content: s,
                    spans: [],
                    textStart: textStart + pos,
                });
            }
            
            let s = seq.slice(term.start, term.end);
            if (s.length != 0) {
                let subterms = expandTerms(s, term.children, startIndex + output.length, textStart + term.start);
                output = output.concat(subterms);
            }

            pos = term.end;
        });

        // Trailing section.
        if (pos < seq.length) {
            let s = seq.slice(pos, seq.length);
            outerSpans.push(output.length);
            output.push({
                content: s,
                spans: [],
                textStart: textStart + pos,
            });
        }

        let endIndex = startIndex + output.length;
        outerSpans.forEach(index => {
            for (let i = startIndex; i < endIndex; i++) {
                output[index].spans.push(i);
            }
        });

        return output;
    }

    let spans = expandTerms(sequent.result, sequent.terms, 0, 0);
    let hoveredElement = $state<number | null>(null);

    function onMouseOver(index: number) {
        hoveredElement = index;
    }

    function onMouseOut(index: number) {
        if (hoveredElement == index) {
            hoveredElement = null;
        }
    }

    function isMarked(index: number) {
        if (!hoveredElement) {
            return false;
        }

        return spans[hoveredElement].spans.includes(index);
    }

    type ContextMenuState = {
        open: boolean;
        x: number;
        y: number;
        actions: TermActionDesc[],
    };

    let contextMenuState = $state<ContextMenuState>({
        open: true,
        x: 0,
        y: 0,
        actions: [],
    });

    function onClick(event: MouseEvent, index: number) {
        const textStart = spans[index].textStart;

        appState.client.goalActions(sequent.id, textStart).then(actions => {
            contextMenuState = {
                open: true,
                x: event.pageX,
                y: event.pageY,
                actions,
            };
        });
    }
</script>

<div class="tree">
    {#each spans as span, index}
    <span
        onmouseover={(e) => onMouseOver(index)}
        onmouseout={(e) => onMouseOut(index)}
        onclick={(e) => onClick(e, index)}
        class:span-hover={isMarked(index)}
    >
        {span.content}
    </span>
    {/each}

    {#if contextMenuState.open}
    <div class="ctx-menu" style="top: {contextMenuState.y}px; left: {contextMenuState.x}px;">
        <ul>
            {#each contextMenuState.actions as action}
                <li><button>{action.displayName}</button></li>
            {/each}
        </ul>
    </div>
    {/if}
</div>

<style>
    .tree {
        background: #252525;
        padding: 10px;
        border-radius: 6px;
        height: 100%;
        overflow-x: hidden;
        overflow-y: auto;
        word-wrap: break-word;
        overflow-wrap: break-word;
    }
    .tree span {
        display: inline;
        white-space: pre-wrap; 
        word-break: break-word; 
    }
    .display {
        margin-top: 8px;
        background: #333;
        padding: 6px;
        border-radius: 4px;
    }

    .span-hover {
        background-color: gray;
    }

    .ctx-menu {
        position: absolute;
        background: #1f1f1f;
        padding: 5px;
        border: 1px solid #444;
        border-radius: 8px;
        box-shadow: 0 10px 25px rgba(0,0,0,0.4);
        z-index: 1000;
   }

    .ctx-menu ul {
        list-style-type: none;
        padding: 0;
        margin: 0;
    }
</style>
