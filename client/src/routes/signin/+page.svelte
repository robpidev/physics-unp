<<<<<<< HEAD
<h1> Hola mundo </h1>
=======
<script>
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { token, user } from '$lib/stores.js';
	import { onMount } from 'svelte';

	export let form;
	onMount(async () => {
		await fetch('/?/logout');
		token.update(() => null);
		user.update(() => null);
	});
</script>

<div class="content">
	<h1>Inicia Sesión</h1>
	<form
		method="POST"
		action="?/signin"
		use:enhance={() => {
			return async ({ result }) => {
				if (result.status === 200) {
					token.update(() => result.data.token);
					user.update(() => result.data.user);
					localStorage.setItem('user', JSON.stringify(result.data.user));
					localStorage.setItem('token', result.data.token);
					goto('/');
				}

				form = result.data;
			};
		}}
	>
		<label>
			<span>Código: </span>
			<input name="code" type="text" required autocomplete="on" />
		</label>
		<label>
			<span>Contraseña: </span>
			<input name="password" type="password" required />
		</label>
		<button type="submit">Iniciar Sesión</button>
	</form>

	{#if form?.error}
		<p class="error">{form.error}</p>
	{/if}

	<span
		>¿No tienes cuenta?

		<a href="/signup">Registrate</a>
	</span>
</div>

<style>
	.content {
		padding: 1em;
		display: flex;
		align-items: center;
		justify-content: center;
		display: flex;
		flex-direction: column;
		gap: 1em;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 0.8em;
		border: solid 1px var(--border);
		padding: 1.5em;
		border-radius: 12px;
		width: 300px;
	}

	form label {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	input {
		border-radius: 6px;
		border: solid 1px var(--border);
		height: 2.3em;
	}

	input:focus {
		outline: solid 2px var(--primary);
		/*outline: none;*/
	}

	button {
		margin: 1.5em 0 0.5em 0;
		background: var(--primary);
		color: white;
		padding: 0.5em 1em;
		border-radius: 6px;
		border: none;
		font-weight: 600;
	}

	a {
		color: var(--primary);
		text-decoration: none;
		font-weight: 600;
	}

	a:hover {
		color: var(--hover);
	}

	a:active {
		color: var(--active);
	}

	button:hover {
		background: var(--hover);
	}

	button:active {
		background: var(--active);
	}

	.error {
		color: red;
	}
</style>
>>>>>>> 99da2311c3e72a3fee7302281841e69e312a0b61
