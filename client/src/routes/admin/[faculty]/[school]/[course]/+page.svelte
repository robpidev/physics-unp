<script>
	import Details from './Details.svelte';
	import Ponderate from './Ponderate.svelte';
	import Info from './Info.svelte';
	import Professors from './Professors.svelte';
	import AddStudent from './AddStudent.svelte';

	let search;
	let student = null;

	export let data;
	export let form;

	function average(scores) {
		const tw = data.course.tests[0].weight / 100;
		const pw = data.course.tests[1].weight / 100;
		let test = 0;
		let totalTests = 0;
		let practice = 0;
		let totalPractice = 0;

		scores.forEach((score) => {
			if (score.ev_type === 'test') {
				test += score.score;
				totalTests += 1;
			} else if (score.ev_type === 'practice') {
				practice += score.score;
				totalPractice += 1;
			}
		});

		const average = (test / totalTests) * tw + (practice / totalPractice) * pw;
		return Math.round(average * 100) / 100;
	}
</script>

<Info course={data.course} />

<section>
	<Professors professorsAssigneds={data.course?.professors} />
</section>

{#if student != null}
	<section>
		<Details student={data.evaluations[student]} {form} />
	</section>
{/if}

<section>
	{#if data?.evaluations.length > 0}
		<h2>Evaluaciones</h2>
		<Ponderate tests={data.course.tests} />

		<hr />

		<label>
			Buscar:
			<input bind:value={search} type="text" placeholder="Código o nombre" />
		</label>

		<hr />
		<div class="table">
			<span class="head">Código</span>
			<span class="head">Nombre</span>
			<span class="head">Promedio</span>
			<span class="head">Detalles</span>

			{#each data.evaluations as ev, id}
				{#if search == undefined || ev.name.includes(search) || ev.id.includes(search) || search == ''}
					<span class="cell center">{ev.id}</span>
					<span class="cell left">{ev.name}</span>
					<span class="cell right">{average(ev.scores)}</span>
					<span class="cell">
						<button on:click={() => (student = id)} class="details">Ver</button>
					</span>
					<!-- content here -->
				{/if}
			{/each}
		</div>
	{:else}
		<p>No hay alumnos registrados</p>
	{/if}
	<hr />
	<AddStudent {form} />
</section>

<style>
	.table {
		width: 100%;
		display: grid;
		grid-template-columns: 1fr 2fr 0.5fr 0.5fr;
		gap: 1px;
		overflow-x: auto;
	}

	.table .head {
		background: var(--color-200);
	}
	.table span {
		border-radius: 3px;
		text-align: center;
	}

	.table .cell {
		background: var(--bg);
		padding: 5px;
	}

	.left {
		display: flex;
		text-justify: left;
	}

	.right {
		display: flex;
		justify-content: right;
	}

	section {
		background: white;
		border-radius: 8px;
		padding: 1em;
		margin-bottom: 1em;
	}

	.details {
		background: var(--bg);
		color: var(--primary);
		padding: 0;
	}

	.details:hover {
		color: var(--color-600);
	}

	.details:active {
		color: var(--color-700);
	}
</style>
