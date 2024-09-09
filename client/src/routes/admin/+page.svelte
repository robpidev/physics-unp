<script>
	import { enhance } from '$app/forms';

	export let data;
	let facultyName = '';
	let error = '';

	const dateHandle = (dateTime) => {
		const date = new Date(dateTime);

		return date.toLocaleDateString('es-ES', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit'
		});
	};

	const timeHandle = (dateTime) => {
		const date = new Date(dateTime);
		return date.toLocaleTimeString('es-ES', {
			hour: '2-digit',
			minute: '2-digit'
		});
	};
</script>

<section>
	<h2>Horarios</h2>
	<hr />
	{#if data?.calendar}
		<ul class="calendar">
			<li class="row head">
				<span>Para</span>
				<span>Desde</span>
				<span>Hasta</span>
				<div>Eliminar</div>
			</li>
			{#each data.calendar as cal}
				<li class="row">
					<span class="col">{cal.todo}</span>
					<span class="col right">{dateHandle(cal.from) + ', ' + timeHandle(cal.from)}</span>
					<span class="col right">{dateHandle(cal.to) + ', ' + timeHandle(cal.to)}</span>
					<span>Eliminar</span>
				</li>
			{/each}
			<li></li>
		</ul>
	{:else}
		<p>No hay horarios establecidos</p>
	{/if}
</section>

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
				<a href={`/admin/${f.id}`}>{f.name}</a>
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

	.calendar {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.row {
		border-radius: 4px;
		display: flex;
		justify-content: space-between;
		padding: 5px;
		flex-wrap: wrap;
	}

	.row > span:nth-child(1) {
		width: 5em;
	}

	.head {
		border: solid 1px;
	}
</style>
