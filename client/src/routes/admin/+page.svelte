<script>
	import { enhance } from '$app/forms';
	export let data;
	let facultyName = '';
	let error = '';
</script>

<div class="page">
	<h1>{error}</h1>
	<nav>
		<ul class="breadcrumb">
			<li><a href="/admin">facultades</a></li>
			<li><span class="/">►</span></li>
			<li><a href="/admin">facultades</a></li>
		</ul>
	</nav>

	<section>
		<form
			method="POST"
			action="?/add"
			use:enhance={() => {
				return async ({ result }) => {
					if (result.status !== 200) {
						console.log(error);
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
						data?.faculties.some((s) => s.name === facultyName.toUpperCase())}
					type="submit">Agregar</button
				>
			</label>
		</form>
	</section>

	<section>
		<ul class="faculties">
			{#each data?.faculties as f, i}
				<li>
					<a href={`/admin/${f.id}`} class="faculty">{f.name}</a>
				</li>
			{/each}
		</ul>
	</section>
</div>

<style>
	.page {
		max-width: 1000px;
		/*border: solid 1px;*/
		width: 100%;
		padding: 0 1em;
	}

	.breadcrumb {
		display: flex;
		list-style: none;
		padding: 0;
		margin: 0;
	}

	a {
		font-weight: 700;
	}

	nav {
		margin: 10px;
		padding: 6px;
		border: solid 1px var(--border);
		border-radius: 8px;
	}

	.faculties {
		display: flex;
		flex-direction: column;
		flex-wrap: wrap;
		gap: 40px;
		list-style: none;
	}

	ul {
		padding: 0;
		margin: 0;
	}

	.faculty {
		border: solid 1px var(--border);
		padding: 1em;
		border-radius: 12px;
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
