<script>
	import { enhance } from '$app/forms';
	export let data;
	let facultyName = '';
	let error = '';
</script>

<h1>{error}</h1>

<section>
	<form
		method="POST"
		action="?/add"
		use:enhance={() => {
			return async ({ result }) => {
				if (result.status !== 200) {
					// console.log(error);
					error = result.data.error;
				} else {
					facultyName = '';
					error = '';
				}
			};
		}}
	>
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
</section>

<section>
	<ul class="faculties">
		{#each data?.faculties as f, i}
			<li class="faculty">
				<a href={`/admin/${f.id}`}>{f.name}</a>
			</li>
		{/each}
	</ul>
</section>

<style>
	a {
		font-weight: 700;
	}

	.faculties {
		display: flex;
		flex-direction: column;
		flex-wrap: wrap;
		gap: 10px;
		list-style: none;
	}

	.faculty {
		width: 100%;
		border: solid 1px var(--border);
		border-radius: 8px;
		padding: 0.5em 1em;
	}

	ul {
		padding: 0;
		margin: 0;
	}

	section {
		padding: 1em;
	}

	span {
		color: var(--primary);
		margin: 5px;
	}

	form {
		border: solid 1px var(--border);
		border-radius: 8px;
		padding: 4px 0px;
		position: sticky;
		top: 1px;
	}

	label > span {
		font-weight: 600;
	}
</style>
