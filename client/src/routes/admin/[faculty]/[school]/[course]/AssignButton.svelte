<script>
	import { enhance } from '$app/forms';

	/**
	 * @typedef {Object} Props
	 * @property {string} [user_id]
	 * @property {string} [role]
	 * @property {import('svelte').Snippet} [children]
	 */

	/** @type {Props} */
	let { user_id = '', role = '', children } = $props();
	let creating = $state(false);
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
	<button disabled={creating} type="send">{@render children?.()}</button>
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
