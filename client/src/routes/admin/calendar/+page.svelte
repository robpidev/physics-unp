<script>
	export let data;
	import Delete from './Delete.svelte';
	import Add from './Add.svelte';

	export let form;

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
				<span>Para</span>
				<span>Desde</span>
				<span>Hasta</span>
				<div>Eliminar</div>
			</li>
			{#each data.calendar as cal}
				<li class="row">
					<span class="col">{cal.todo}</span>
					<span class="col right">{dateHandle(cal.start) + ', ' + timeHandle(cal.start)}</span>
					<span class="col right">{dateHandle(cal.end) + ', ' + timeHandle(cal.end)}</span>
					<Delete id={cal.id} />
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
