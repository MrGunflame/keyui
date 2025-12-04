<script>
    import ProofTree from '$lib/components/ProofTree.svelte';
    import CurrentGoal from '$lib/components/CurrentGoal.svelte';
    import GoalsPanel from '$lib/components/GoalsPanel.svelte';
</script>

<style>
    .layout {
        display: grid;
        grid-template-columns: 250px 1fr 250px;
        gap: 10px;
        height: 100vh;
        padding: 10px;
        background: #1e1e1e;
        color: white;
    }
</style>

<div class="layout">
    <ProofTree />
    <CurrentGoal />
    <GoalsPanel />
</div>
