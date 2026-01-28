<script lang="ts">
    let { sequent } = $props();

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

    let selectedPath: number[] | null = null;

    function handleSelect(path: number[]) {
        selectedPath = path;
        console.log("Clicked path:", path);
    }
</script>

<div class="tree">
    {#each spans as span, index}
    <span
        onmouseover={(e) => onMouseOver(index)}
        onmouseout={(e) => onMouseOut(index)}
        class:span-hover={isMarked(index)}
    >
        {span.content}
    </span>
    {/each}
</div>

<style>
    .tree {
        background: #252525;
        padding: 10px;
        border-radius: 6px;
        height: 100%;
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

</style>
