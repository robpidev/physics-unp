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

	<hr />
	<span class="advice">Las clases inician lunes 4 de noviembre.</span>
</div>

<style>
	a {
		font-weight: 600;
	}

	div {
		padding: 1em;
	}

	.advice {
		display: block;
		color: green;
		font-weight: bold;
		font-size: 1.5rem;
	}
</style>
