<script lang="ts">
    import type {
        TermActionDesc,
        NodeTextSpan,
        TermActionId,
    } from "../../routes/api";

    let { appState, sequent } = $props();

    type Span = {
        content: string;
        // All span indices that are associated with this span.
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
        actions: TermActionDesc[];
    };

    let contextMenuState = $state<ContextMenuState>({
        open: true,
        x: 0,
        y: 0,
        actions: [],
    });

    function onClick(event: MouseEvent, index: number) {
        const textStart = spans[index].textStart;

        appState.client
            .goalActions(sequent.id, textStart)
            .then((actions: TermActionDesc[]) => {
                contextMenuState = {
                    open: true,
                    x: event.pageX,
                    y: event.pageY,
                    actions,
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

    import hljs from "highlight.js";
    import "highlight.js/styles/github-dark.css";

    // Build a char-position → hljs class lookup from hljs HTML output.
    // hljs.highlight() returns HTML like:
    //   <span class="hljs-keyword">if</span> b <span class="hljs-number">3u32</span>
    // We strip the tags and record which class was active at each char index.
    function buildHljsColorMap(text: string, language: string): string[] {
        const result = hljs.highlight(text, { language, ignoreIllegals: true });
        const html = result.value;

        const colorMap: string[] = new Array(text.length).fill("");
        let charPos = 0;
        let currentClass = "";

        // Simple HTML parser — walks through the hljs output char by char.
        let i = 0;
        while (i < html.length) {
            if (html[i] === "<") {
                const tagEnd = html.indexOf(">", i);
                if (tagEnd === -1) break;
                const tag = html.slice(i + 1, tagEnd);

                if (tag.startsWith("/span")) {
                    currentClass = "";
                } else if (tag.startsWith("span")) {
                    // Extract class name from e.g. 'span class="hljs-keyword"'
                    const m = tag.match(/class="([^"]+)"/);
                    currentClass = m ? m[1] : "";
                }
                i = tagEnd + 1;
            } else {
                // Decode basic HTML entities that hljs emits
                let ch: string;
                if (html.startsWith("&amp;", i))  { ch = "&";  i += 5; }
                else if (html.startsWith("&lt;", i))   { ch = "<";  i += 4; }
                else if (html.startsWith("&gt;", i))   { ch = ">";  i += 4; }
                else if (html.startsWith("&quot;", i)) { ch = '"';  i += 6; }
                else                                    { ch = html[i]; i += 1; }

                if (charPos < colorMap.length) {
                    colorMap[charPos] = currentClass;
                    charPos++;
                }
            }
        }

        return colorMap;
    }

    const hljsColorMap = buildHljsColorMap(sequent.result, "rust");

    // Returns the hljs class for a given span based on its textStart position.
    // Uses the first non-whitespace character of the span for best accuracy.
    function hljsClassForSpan(span: Span): string {
        const text = span.content;
        for (let i = 0; i < text.length; i++) {
            if (text[i].trim() !== "") {
                const pos = span.textStart + i;
                return hljsColorMap[pos] ?? "";
            }
        }
        return "";
    }
</script>

<div class="tree">
    {#each spans as span, index}
        <span
            onmouseover={(e) => onMouseOver(index)}
            onmouseout={(e) => onMouseOut(index)}
            onclick={(e) => onClick(e, index)}
            class:span-hover={isMarked(index)}
            class={hljsClassForSpan(span)}
        >
            {span.content}
        </span>
    {/each}

    {#if contextMenuState.open}
        <div
            class="ctx-menu"
            style="top: {contextMenuState.y}px; left: {contextMenuState.x}px;"
        >
            <ul>
                {#each contextMenuState.actions as action}
                    <li>
                        <button onclick={() => applyAction(action.commandId)}
                            >{action.displayName}</button
                        >
                    </li>
                {/each}
            </ul>
        </div>
    {/if}
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

    .span-hover {
        background-color: gray;
    }

    .ctx-menu {
        position: absolute;
        background: #1f1f1f;
        padding: 5px;
        border: 1px solid #444;
        border-radius: 8px;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.4);
        z-index: 1000;
    }

    .ctx-menu ul {
        list-style-type: none;
        padding: 0;
        margin: 0;
    }
</style>
