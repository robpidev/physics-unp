<script>
	import { enhance } from '$app/forms';
	import { breadcrum } from '$lib/stores.js';
	import { onMount } from 'svelte';

	export let data;
	let facultyName = '';
	let error = '';

	onMount(() => {
		breadcrum.update(() => []);
	});
</script>

<section>
	<h2>Facultades</h2>
	<form method="POST" action="?/add" use:enhance>
		<label>
			<span>Nombre de la facultad: </span>
			<input on:keydown={() => (error = '')} type="text" name="name" bind:value={facultyName} />
			<button
				disabled={facultyName === '' ||
					error !== '' ||
					data?.faculties.some((s) => s.name === facultyName.toUpperCase().trim())}
				type="submit">Agregar</button
			>
		</label>
	</form>

	<hr />
	<ul class="faculties">
		{#each data?.faculties as f}
			<li class="faculty">
				<a
					on:click={() => breadcrum.update(() => [{ name: f.name, url: f.id }])}
					href={`/admin/${f.id}`}>{f.name}</a
				>
			</li>
		{/each}
	</ul>
</section>

<style>
	a {
		font-weight: 600;
	}

	.faculties {
		display: flex;
		flex-direction: column;
		gap: 10px;
		list-style: none;
	}

	.faculty {
		border-radius: 8px;
		padding: 0;
	}

	ul {
		padding: 0;
		margin: 0;
		list-style: none;
	}

	section {
		padding: 1em;
	}

	a {
		color: var(--primary);
		margin: 5px;
	}

	form {
		border-radius: 8px;
		padding: 4px 0px;
		position: sticky;
		top: 1px;
	}

	label > span {
		font-weight: 600;
	}
</style>
