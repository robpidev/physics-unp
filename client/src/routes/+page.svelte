<script>
	import { user } from '$lib/stores';
</script>

<div>
	<h1>Hola</h1>
	{#if $user}
		Pudes ingresar a
		<a href="/{$user?.role}">Cursos</a>
	{:else}
		Puedes
		<a href="/signin">Iniciar sesión</a>
		<span>O</span>
		<a href="/signup">Registrate</a>
	{/if}
</div>

<style>
	a {
		font-weight: 600;
	}
</style>
