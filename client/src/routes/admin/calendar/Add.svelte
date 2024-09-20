<script>
	import { enhance } from '$app/forms';
	let date = '';
	export let form;
	let disabled = false;
</script>

{date}
<form action="?/add" method="post" use:enhance>
	<label class="label">
		Descripción
		<input type="text" name="todo" required placeholder="1, ..., n o student o professor" />
	</label>
	<label class="label">
		Hasta
		<input type="date" bind:value={date} name="end" required />
	</label>
	<label>
		Hora
		<div>
			<input value="23" type="number" name="hour" required min="0" max="23" placeholder="HH" />
			:
			<input value="59" type="number" name="minute" required min="0" max="59" placeholder="MM" />
		</div>
	</label>
	<label>
		<br />
		<button type="submit" {disabled}>Agregar</button>
	</label>
</form>

{#if form?.error}
	<p class="error">{form?.error}</p>
{/if}

<style>
	.error {
		color: red;
	}

	button:disabled {
		background: var(--color-400);
	}

	input[type='number'] {
		width: 3em;
		padding-left: 0.4em;
		border: none;
		outline: none;
		background: var(--bg);
	}
	input[type='number']::-webkit-outer-spin-button,
	input[type='number']::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	form {
		display: flex;
		flex-wrap: wrap;
		gap: 1em;
	}

	.label {
		display: flex;
		flex-direction: column;
	}

	form * {
		flex: none;
	}
</style>
