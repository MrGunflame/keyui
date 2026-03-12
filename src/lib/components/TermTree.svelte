<script lang="ts">
    import {
        type TermActionDesc,
        type NodeTextSpan,
        type TermActionId,
        TermActionKind,
    } from "$lib/api";
    import RuleList from "./sequent/RuleList.svelte";
    import ContextMenu from "./ContextMenu.svelte";

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

    function expandTerms(
        seq: string,
        terms: NodeTextSpan[],
        startIndex: number,
        textStart: number,
    ): Span[] {
        terms.sort((a, b) => a.start - b.start);

        let output: Span[] = [];
        let outerSpans = [];

        let pos = 0;
        terms.forEach((term) => {
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
                let subterms = expandTerms(
                    s,
                    term.children,
                    startIndex + output.length,
                    textStart + term.start,
                );
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
        outerSpans.forEach((index) => {
            for (let i = startIndex; i < endIndex; i++) {
                output[index].spans.push(i);
            }
        });

        return output;
    }

    const spans = expandTerms(sequent.result, sequent.terms, 0, 0);
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

    type Actions = {
        taclets: TermActionDesc[];
        macros: TermActionDesc[];
        other: TermActionDesc[];
    };

    type ContextMenuState = {
        open: boolean;
        x: number;
        y: number;
        actions: Actions;
    };

    let contextMenuState = $state<ContextMenuState>({
        open: false,
        x: 0,
        y: 0,
        actions: {
            taclets: [],
            macros: [],
            other: [],
        },
    });

    function onClick(event: MouseEvent, index: number) {
        const textStart = spans[index].textStart;

        appState.client
            .goalActions(sequent.id, textStart)
            .then((actions: TermActionDesc[]) => {
                const taclets = actions.filter(
                    (a) => a.kind === TermActionKind.Taclet,
                );
                const macros = actions.filter(
                    (a) => a.kind === TermActionKind.Macro,
                );
                const other = actions.filter(
                    (a) =>
                        a.kind != TermActionKind.Taclet &&
                        a.kind != TermActionKind.Macro,
                );

                contextMenuState = {
                    open: true,
                    x: event.pageX,
                    y: event.pageY,
                    actions: {
                        taclets,
                        macros,
                        other,
                    },
                };
            });
    }

    function applyAction(id: TermActionId) {
        contextMenuState.open = false;

        appState.client
            .applyAction(id)
            .then((ok: boolean) => {
                if (ok) {
                    appState.proofTreeChanged.notify();
                } else {
                    console.error("failed to apply rule");
                }
            })
            .catch((err: Error) => {
                // TODO: Post error to error widget.
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

    <ContextMenu
        open={contextMenuState.open}
        onClose={() => (contextMenuState.open = false)}
        positionX={contextMenuState.x}
        positionY={contextMenuState.y}
    >
        <div class="action-list">
            <RuleList
                name={"Taclet"}
                actions={contextMenuState.actions.taclets}
                onApply={(action) => applyAction(action.commandId)}
            />
            <RuleList
                name={"Macros"}
                actions={contextMenuState.actions.macros}
                onApply={(action) => applyAction(action.commandId)}
            />
            <RuleList
                name={"Other"}
                actions={contextMenuState.actions.other}
                onApply={(action) => applyAction(action.commandId)}
            />
        </div>
    </ContextMenu>

    {#if contextMenuState.open}{/if}
</div>

<style>
    .tree {
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

    .span-hover {
        background-color: gray;
    }

    .action-list {
        display: flex;
        overflow: scroll;
    }
</style>
