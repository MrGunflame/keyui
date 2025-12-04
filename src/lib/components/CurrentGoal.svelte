<script>
    export let sequent = "==> (p -> q) -> !q -> !p";
</script>

<style>
    .goal {
        background: #252525;
        padding: 10px;
        border-radius: 6px;
        height: 100%;
    }
</style>

<div class="goal">
    <h3>Current Goal</h3>
    <p>{sequent}</p>
</div>
