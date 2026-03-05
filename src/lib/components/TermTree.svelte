<script lang="ts">
    import { asClassComponent } from "svelte/legacy";
    import {
        type TermActionDesc,
        type NodeTextSpan,
        type TermActionId,
        TermActionKind,
    } from "../../routes/api";
    import RuleList from "./sequent/RuleList.svelte";

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

    // --- Rust syntax highlighting ---

    const RUST_KEYWORDS = new Set([
        "as", "async", "await", "break", "const", "continue", "crate", "dyn",
        "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
        "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
        "self", "Self", "static", "struct", "super", "trait", "true", "type",
        "unsafe", "use", "where", "while",
    ]);

    const RUST_TYPES = new Set([
        "i8", "i16", "i32", "i64", "i128", "isize",
        "u8", "u16", "u32", "u64", "u128", "usize",
        "f32", "f64", "bool", "char", "str", "String",
        "Vec", "Option", "Result", "Box", "Rc", "Arc",
        "HashMap", "HashSet", "BTreeMap", "BTreeSet",
        "Cell", "RefCell", "Mutex", "RwLock",
    ]);

    type RustTokenKind =
        | "keyword"
        | "type"
        | "number"
        | "string"
        | "comment"
        | "lifetime"
        | "macro"
        | "operator"
        | "punctuation"
        | "plain";

    function rustTokenKind(token: string): RustTokenKind {
        const t = token.trim();
        if (t === "") return "plain";

        // Line comment
        if (t.startsWith("//")) return "comment";

        // String / char literal
        if (
            (t.startsWith('"') && t.endsWith('"')) ||
            (t.startsWith("'") && t.endsWith("'") && t.length > 2) ||
            (t.startsWith('b"') && t.endsWith('"')) ||
            t.startsWith('r#"')
        ) return "string";

        // Lifetime  e.g. 'a  'static
        if (/^'[a-z_][a-z0-9_]*$/.test(t)) return "lifetime";

        // Macro call  e.g. println!
        if (/^[a-z_][a-z0-9_]*!$/.test(t)) return "macro";

        // Keywords
        if (RUST_KEYWORDS.has(t)) return "keyword";

        // Built-in types
        if (RUST_TYPES.has(t)) return "type";

        // Numbers: integer, float, hex, binary, octal with optional suffix
        if (/^-?(?:0x[0-9a-fA-F_]+|0b[01_]+|0o[0-7_]+|[0-9][0-9_]*(?:\.[0-9_]+)?(?:[eE][+-]?[0-9_]+)?)(?:_?(?:i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize|f32|f64))?$/.test(t))
            return "number";

        // Operators
        if (/^(?:=>|->|::|\.\.=|\.\.|\+=|-=|\*=|\/=|%=|&&|\|\||[+\-*/%&|^!<>=?@~]+)$/.test(t))
            return "operator";

        // Punctuation
        if (/^[{}()[\];:,.]$/.test(t)) return "punctuation";

        return "plain";
    }
</script>

<div class="tree">
    {#each spans as span, index}
        <span
            onmouseover={(e) => onMouseOver(index)}
            onmouseout={(e) => onMouseOut(index)}
            onclick={(e) => onClick(e, index)}
            class:span-hover={isMarked(index)}
            class="rust-{rustTokenKind(span.content)}"
        >
            {span.content}
        </span>
    {/each}

    {#if contextMenuState.open}
        <div
            class="ctx-menu"
            style="top: {contextMenuState.y}px; left: {contextMenuState.x}px;"
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

    /* Rust syntax highlighting */
    .rust-keyword   { color: #cc99cd; font-weight: 600; }
    .rust-type      { color: #4ec9b0; }
    .rust-number    { color: #b5cea8; }
    .rust-string    { color: #ce9178; }
    .rust-comment   { color: #6a9955; font-style: italic; }
    .rust-lifetime  { color: #d7ba7d; }
    .rust-macro     { color: #dcdcaa; }
    .rust-operator  { color: #d4d4d4; }
    .rust-punctuation { color: #808080; }
    .rust-plain     { color: inherit; }

    .ctx-menu {
        position: absolute;
        background: #1f1f1f;
        padding: 5px;
        border: 1px solid #444;
        border-radius: 8px;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.4);
        z-index: 1000;
    }

    .action-list {
        display: flex;
        overflow: scroll;
    }
</style>
