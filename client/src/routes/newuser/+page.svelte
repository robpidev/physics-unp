<script>
	import { user } from '$lib/stores.js';
	import { onDestroy, onMount } from 'svelte';
	import { goto } from '$app/navigation';
	let usr;
	let user_sus = user.subscribe((u) => {
		usr = u;
	});
	onDestroy(user_sus);
	onMount(() => {
		if (usr === null) goto('/signin');
	});
</script>

<div class="containder">
	<span class="title">Usuario registrado</span>

	<div class="user">
		<div class="row">
			<span class="label">codigo: </span>
			<span class="value">{usr?.id}</span>
		</div>
		<div class="row">
			<span class="label">Nombres: </span>
			<span class="value">{usr?.names}</span>
		</div>
		<div class="row">
			<span class="label">Apellidos: </span>
			<span class="valu">{usr?.last_name1 + ' ' + usr?.last_name2}</span>
		</div>
	</div>
	<a href="/signin">Iniciar sesión</a>
</div>

<style>
	.containder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}
	a {
		font-weight: 600;
	}

	.title {
		margin-top: 1em;
		font-size: 1.5em;
	}

	.user {
		display: flex;
		flex-direction: column;
		border: solid 1px var(--border);
		border-radius: 12px;
		padding: 1em;
		margin: 1em;
		gap: 0.5em;
		box-shadow:
			0 4px 6px -1px rgb(0 0 0 / 0.1),
			0 2px 4px -2px rgb(0 0 0 / 0.1);
	}

	.label {
		color: var(--primary);
		font-weight: 600;
	}
</style>
