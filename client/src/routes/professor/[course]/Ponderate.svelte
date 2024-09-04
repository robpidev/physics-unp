<script>
	import { enhance } from '$app/forms';
	export let tests;
	let updating = false;
</script>

<form
	method="post"
	action="?/updateponderation"
	use:enhance={() => {
		updating = true;
		return async ({ update }) => {
			await update();
			updating = false;
		};
	}}
>
	<span class="title">Ponderados: </span>
	{#each tests as test}
		<label>
			<span>{test.name}:</span>
			<input type="number" name={test.name} bind:value={test.weight} /> %
		</label>
	{/each}
	<button disabled={updating} type="submit">Actualizar</button>
</form>

<style>
	form {
		display: flex;
		gap: 1em;
		margin: 10px;
		align-items: center;
		flex-wrap: wrap;
	}
	span {
		margin-right: 0.5em;
	}
	input {
		width: 2.5em;
		border: none;
		background: var(--bg);
		height: min-content;
	}

	input[type='number']::-webkit-inner-spin-button,
	input[type='number']::-webkit-outer-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}
</style>
