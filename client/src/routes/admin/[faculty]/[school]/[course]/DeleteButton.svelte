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
		border: solid 1px;
		border-color: red;
		color: red;
	}
</style>
