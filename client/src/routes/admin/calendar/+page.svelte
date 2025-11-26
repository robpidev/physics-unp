<script>
	import Delete from './Delete.svelte';
	import Add from './Add.svelte';

	let { data, form } = $props();

	const dateHandle = (dateTime) => {
		const date = new Date(dateTime);

		return date.toLocaleDateString('es-ES', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit',
			timeZone: 'America/Lima'
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
	<h2>Horarios de registro</h2>
	<hr />
	<Add {form} />
	<hr />
	{#if data?.calendar}
		<ul class="calendar">
			<li class="row head">
				<span class="col">Para</span>
				<span class="col">Desde</span>
				<span class="col">Hasta</span>
				<div class="col">Eliminar</div>
			</li>
			{#each data.calendar as cal}
				<li class="row info">
					<span class="col todo">{cal.todo}</span>
					<span class="col right">{dateHandle(cal.start) + ', ' + timeHandle(cal.start)}</span>
					<span class="col right">{dateHandle(cal.end) + ', ' + timeHandle(cal.end)}</span>
					<Delete class="col" id={cal.id} />
				</li>
			{/each}
			<li></li>
		</ul>
	{:else}
		<p>No hay horarios establecidos</p>
	{/if}
</section>

<style>
	section {
		padding: 1em;
	}
	.calendar {
		display: flex;
		flex-direction: column;
		gap: 2px;
		list-style: none;
		padding: 0;
	}

	.row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0 5px;
		flex-wrap: wrap;
	}

	.row > span:nth-child(1) {
		width: 5em;
	}

	ul .info {
		border-radius: 4px;
		background: var(--bg);
	}

	.head {
		border-bottom: solid 1px var(--border);
		margin-bottom: 2px;
	}

	.col {
		min-width: 150px;
		flex-grow: 1;
	}

	.todo {
		background: var(--green);
	}
</style>
