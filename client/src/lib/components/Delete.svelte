<script>
	import { enhance } from '$app/forms';
	let { id } = $props();
	let disabled = $state(false);
</script>

<form
	method="post"
	action="?/remove"
	use:enhance={() => {
		disabled = true;
		return async ({ update }) => {
			await update();
			disabled = false;
		};
	}}
>
	<input type="hidden" name="id" value={id} />
	<button type="submit" {disabled}>Eliminar</button>
</form>

<style>
	button {
		background: none;
		color: #ff2323;
		font-weight: 500;
		user-select: none;
	}

	button:hover {
		color: ff0000;
		font-weight: 600;
	}

	button:active {
		color: #d70000;
		transform: scale(0.98);
	}

	button[disabled='true'] {
		color: blue;
	}

	button:disabled {
		color: #ffc0c0;
	}
</style>
