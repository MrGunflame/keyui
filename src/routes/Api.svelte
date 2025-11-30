<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let method = $state("");
    let body = $state("");
    let resp = $state("");

    async function submit(e: Event) {
        e.preventDefault();

        try {
            let payload = JSON.parse(body);

            console.log(method);

            let resp_json = invoke("send_msg", {
                "method": method,
                "params": payload,
            });

            resp = JSON.stringify(resp_json);
        } catch (e) {}
    }
</script>

<div>
    <input bind:value={method} />
    <input bind:value={body} />
    <button onclick={submit}>Submit</button>
    <span>{resp}</span>
</div>
