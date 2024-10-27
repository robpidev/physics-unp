<script>
	import { enhance } from '$app/forms';
	export let user_id;
</script>

<form method="post" action="?/unassign" use:enhance>
	<input type="text" name="user_id" value={user_id} hidden />
	<button>Quitar</button>
</form>

<style>
	button {
		background: initial;
		border-color: red;
		color: red;
	}

	button:hover {
		color: #e60000;
		text-decoration: underline;
	}
</style>
