<script>
	let { form } = $props();
	import { enhance } from '$app/forms';
</script>

<form action="?/add" method="post" use:enhance>
	<input name="note" type="text" placeholder="Agrega un nuevo aviso" />
	<button>Agregar</button>
</form>

{#if form?.error}
	<p class="error">{form?.error}</p>
{/if}

<style>
	form {
		width: 100%;
		display: flex;
		gap: 6px;
	}

	input {
		flex-grow: 1;
	}
	.error {
		color: red;
	}
</style>
