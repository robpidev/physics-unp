<script>
	let groups = Array.from({ length: 40 }, (v, i) => i + 1);
	export let selected = [];
</script>

<section>
	<p>Seleccione los grupos que tiene ocupados:</p>
	<div class="groups">
		{#each groups as group}
			{#if !selected.includes(group)}
				<button on:click={() => (selected = [...selected, group])}>
					{group}
				</button>
			{/if}
		{/each}
	</div>
	<div class="selected">
		Grupos ocupados:
		{#if selected.length != 0}
			{#each selected as group}
				<button on:click|preventDefault={() => (selected = selected.filter((g) => g !== group))}>
					{group}
				</button>
			{/each}
			<!-- content here -->
		{:else}
			<!-- else content here -->
			<span class="error">Sin grupos ocupados</span>
		{/if}
	</div>
</section>

<style>
	.groups,
	.selected {
		display: flex;
		flex-wrap: wrap;
		gap: 5px;
		padding: 5px;
		border-radius: 5px;
	}

	.groups {
		border: solid 1px var(--border);
	}

	.selected {
		align-items: center;
	}

	button {
		width: 40px;
		color: var(--text);
		background: var(--bg);
	}

	button:hover {
		color: var(--primary);
		background: var(--color-200);
	}

	.error {
		color: red;
	}
</style>
