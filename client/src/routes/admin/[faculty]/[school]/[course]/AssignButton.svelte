<script>
	import { enhance } from '$app/forms';

	export let user_id = '';
	export let role = '';
	let creating = false;
</script>

<form
	method="post"
	action="?/assign"
	use:enhance={() => {
		creating = true;
	}}
>
	<input type="text" name="user_id" hidden value={user_id} />
	<input type="text" name="role" hidden value={role} />
	<button disabled={creating} type="send"><slot /></button>
</form>

<style>
	button {
		height: min-content;
		background: inherit;
		color: var(--primary);
	}

	button:hover {
		text-decoration: underline;
		color: var(--color-600);
	}
</style>
