<script>
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { user } from '$lib/stores.js';
	import { onMount } from 'svelte';

	export let form;
	onMount(async () => {
		await fetch('/?/logout');
		user.set(null);
		localStorage.removeItem('user');
	});

	let loading = false;
</script>

<div class="content">
	<h1>Inicia Sesión</h1>
	<form
		method="POST"
		action="?/signin"
		use:enhance={() => {
			loading = true;
			return async ({ result }) => {
				if (result.status === 200) {
					const u = result.data.user;
					user.update(() => result.data.user);
					localStorage.setItem('user', JSON.stringify(result.data.user));
					if (u.role === 'professor') goto('/professor');
					else if (u.role === 'student') goto('/student');
					else if (u.role === 'admin') goto('/admin');
				}
				form = result.data;
				loading = false;
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
		<button disabled={loading} type="submit">Iniciar Sesión</button>
	</form>

	{#if form?.error}
		<p class="error">{form.error}</p>
	{/if}

	{#if loading}
		<p class="loading">Iniciando sesión...</p>
	{:else}
		<span
			>¿No tienes cuenta?

			<a href="/signup">Registrate</a>
		</span>
	{/if}
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
	a {
		color: var(--primary);
		text-decoration: none;
		font-weight: 600;
	}

	button {
		margin: 1.5em 0 0.5em 0;
	}

	a:hover {
		color: var(--hover);
	}

	a:active {
		color: var(--active);
	}

	.error {
		color: red;
	}

	.loading {
		color: green;
	}
</style>
