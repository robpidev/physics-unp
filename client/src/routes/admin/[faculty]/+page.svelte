<script>
	import { enhance } from '$app/forms';
	import { page } from '$app/stores';
	import { breadcrum } from '$lib/stores';
	export let data;
	let name = '';
	let error = '';
</script>

<section class="add">
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
	<br />
	<span class="error">{error}</span>
</section>

<section>
	<ul class="schools">
		{#each data.data as school, id}
			<li class="school">
				<a
					on:click={() => breadcrum.update((url) => url.push({ path: '/otro', name: '/hola' }))}
					href="{$page.params.faculty}/{school.id}">{school.name}</a
				>
			</li>
		{/each}
	</ul>
</section>

<style>
	.schools {
		list-style: none;
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.school {
		border: 1px solid var(--border);
		width: 100%;
		padding: 0.5em 1em;
		border-radius: 8px;
	}

	.add {
		padding: 0 1em;
	}
</style>
