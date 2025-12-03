<script lang="ts">
  import Header from './Header.svelte';

  import { invoke } from "@tauri-apps/api/core";
  import CodeBlock from "$lib/CodeBlock.svelte";

  const rustExample = `
fn main() {
    println!("Hello from Rust + Tauri!");
}
`;

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<<<<<<< HEAD
=======
<div class="menu-bar">
  <div class="menu-item">
    File
    <div class="submenu">
      <div class="submenu-item">New</div>
      <div class="submenu-item">Open</div>
      <div class="submenu-item">Save</div>
      <div class="submenu-item">Exit</div>
    </div>
  </div>

  <div class="menu-item">
    Edit
    <div class="submenu">
      <div class="submenu-item">Undo</div>
      <div class="submenu-item">Redo</div>
      <div class="submenu-item">Cut</div>
      <div class="submenu-item">Copy</div>
      <div class="submenu-item">Paste</div>
    </div>
  </div>

  <div class="menu-item">
    View
    <div class="submenu">
      <div class="submenu-item">Zoom In</div>
      <div class="submenu-item">Zoom Out</div>
    </div>
  </div>

  <div class="menu-item">
    Help
    <div class="submenu">
      <div class="submenu-item">Documentation</div>
      <div class="submenu-item">About</div>
    </div>
  </div>
</div>




>>>>>>> 76ff4a9 (Add Rust syntax highlighting component)
<main class="container">
  <Header />
  
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>
  <section class="code-section">
    <h2>Rust example</h2>
    <CodeBlock language="rust" code={rustExample} />
  </section>

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</main>

<style>

<<<<<<< HEAD
=======

>>>>>>> 76ff4a9 (Add Rust syntax highlighting component)
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  /* Old light style
  color: #0f0f0f;
  background-color: #f6f6f6;
  */
  
  color: #f6f6f6;
  background-color: #2f2f2f;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
.menu-bar {
  display: flex;
  gap: 20px;
  padding: 8px 15px;
  background-color: #222;
  color: white;
  border-bottom: 2px solid #444;
}

.menu-item {
  position: relative;
  padding: 4px 10px;
  cursor: pointer;
}

.submenu {
  display: none;
  position: absolute;
  top: 100%;
  left: 0;
  background-color: #333;
  border: 1px solid #444;
  padding: 5px 0;
  min-width: 120px;
  z-index: 9999;
}

.submenu-item {
  padding: 6px 12px;
  cursor: pointer;
}

.submenu-item:hover {
  background-color: #555;
}

.menu-item:hover .submenu {
  display: block;
}
.code-section {
  margin-top: 40px;
  padding: 20px;
}

.code-section h2 {
  margin-bottom: 10px;
  color: white;
}


</style>
