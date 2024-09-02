<script>
	import { user } from '$lib/stores';
</script>

<div>
	<h1>Hola</h1>
	{#if user == null}
		Puedes
		<a href="/signin">Iniciar sesión</a>
		<span>O</span>
		<a href="/signup">Registrate</a>
	{:else}
		Pudes ingresar a
		<a href="/{$user.role}">Cursos</a>
	{/if}
</div>

<style>
	a {
		font-weight: 600;
	}
</style>
