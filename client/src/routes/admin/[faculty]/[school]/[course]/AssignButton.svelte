<script>
	import { enhance } from '$app/forms';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let user_id = '';
	export let role = '';
	let creating = false;
</script>

<form
	method="post"
	action="?/assign"
	use:enhance={() => {
		creating = true;

		return async ({ update, result }) => {
			await update();
			creating = false;

			if (result.status === 200) {
				dispatch('assigned', {
					role,
					user_id
				});
			}
		};
	}}
>
	<input type="text" name="user_id" hidden value={user_id} />
	<input type="text" name="role" hidden value={role} />
	<button disabled={creating} type="send"><slot /></button>
</form>
