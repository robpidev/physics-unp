<script>
	import { enhance } from '$app/forms';
	import { page } from '$app/stores';
	import { breadcrum } from '$lib/stores';
	let { data } = $props();
	let name = $state('');
	let error = '';
</script>

<section class="add">
	<h2>Escuelas</h2>
	<form method="post" action="?/add" use:enhance>
		<label>
			<span>Nombre de la escuela: </span>
			<input bind:value={name} type="text" name="school" />
		</label>
		<button
			type="submit"
			disabled={name.length === 0 ||
				data?.data.some((s) => {
					return s.name === name.toUpperCase().trim();
				})}>Agregar</button
		>
	</form>
	<span class="error">{error}</span>

	<hr />
	<ul class="schools">
		{#each data.data as school}
			<li class="school">
				<a
					onclick={() =>
						breadcrum.update((path) => [...path, { name: school.name, url: school.id }])}
					href="{$page.params.faculty}/{school.id}">{school.name}</a
				>
			</li>
		{/each}
	</ul>
</section>

<style>
	section {
		padding: 1em 0;
	}
	.schools {
		list-style: none;
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.school {
		border-radius: 8px;
	}

	section {
		padding: 0.5em 1em 1em 1em;
	}

	ul {
		padding: 0;
	}

	a {
		display: block;
		background: var(--bg);
		font-weight: 600;
		border-radius: 5px;
		padding: 0.3em 1em;
	}
</style>
