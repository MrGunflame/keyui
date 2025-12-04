<script lang="ts">
  import hljs from "highlight.js";
  import "highlight.js/styles/github-dark.css";
  import { onMount } from "svelte";

  export let code: string = "";
  export let language: string = "rust";

  let codeElement: HTMLElement;

  onMount(() => {
    if (codeElement) {
      hljs.highlightElement(codeElement);
    }
  });
</script>

<pre>
  <code bind:this={codeElement} class={"language-" + language}>
    {code}
  </code>
</pre>

<style>
pre {
  text-align: left;
  padding: 12px;
  border-radius: 8px;
  background-color: #0d1117;
  overflow-x: auto;
  max-width: 700px;
  margin: 0 auto;
  color: #c9d1d9;   
}

code {
  font-family: Consolas, "Courier New", monospace;
  font-size: 14px;
}
</style>

