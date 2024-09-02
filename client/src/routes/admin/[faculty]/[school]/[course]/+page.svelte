<script>
	import AssignButton from './AssignButton.svelte';
	import DeleteButton from './DeleteButton.svelte';
	import Details from './Details.svelte';
	import Ponderate from './Ponderate.svelte';
	let professors = [];
	async function getProfessors() {
		const url = '/api/professors';
		const resp = await fetch(url);
		professors = await resp.json();
	}

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

<section>
	{#if data.professors.length > 0}
		<h2>Profesores del curso</h2>
		{#each data.professors as professor}
			<ul>
				<li class="professor">
					<div class="info">
						<span class="name">{professor.names}</span>
						<span class="lastname">{professor.last_name1} {professor.last_name2}</span>
					</div>

					<div class="buttons">
						<span>{professor.role}</span>
						<DeleteButton user_id={professor.id} />
					</div>
				</li>
			</ul>
		{/each}
	{:else}
		<h2>No hay profesores asignados al curso</h2>
	{/if}
	{#if professors.length > 0}
		<h3>Eliga el profesor que quiere asignar al curso</h3>
		{#each professors as professor}
			<ul>
				{#if !data.professors.some((p) => p.id == professor.id)}
					<!-- content here -->
					<li class="professor">
						<div class="info">
							<span class="name">{professor.names}</span>
							<span class="lastname">{professor.last_name1} {professor.last_name2}</span>
						</div>
						<div class="buttons">
							<AssignButton role="practice" user_id={professor.id}>Práctica</AssignButton>
							<AssignButton role="theory" user_id={professor.id}>Teoría</AssignButton>
						</div>
					</li>
				{/if}
			</ul>
		{/each}
	{:else}
		<button on:click={getProfessors}>Asignar profesor</button>
	{/if}
</section>

{#if student != null}
	<section>
		<Details student={data.evaluations[student]} {form} />
	</section>
{/if}

<section>
	{#if data.evaluations.length > 0}
		<h2>Evaluaciones</h2>
		<Ponderate tests={data.course.tests} />

		<hr />
		<div class="table">
			<span class="head">Código</span>
			<span class="head">Nombre</span>
			<span class="head">Promedio</span>
			<span class="head">Detalles</span>

			{#each data.evaluations as ev, id}
				<span class="cell center">{ev.id}</span>
				<span class="cell left">{ev.name}</span>
				<span class="cell right">{average(ev.scores)}</span>
				<span class="cell">
					<button on:click={() => (student = id)} class="details">Detalles</button>
				</span>
			{/each}
		</div>
	{/if}
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
	ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}
	.professor {
		gap: 1em;
		display: flex;
		justify-content: space-between;
		border: solid 1px var(--border);
		padding: 0.5em 1em;
		margin: 5px;
		align-items: center;
		border-radius: 8px;
		flex-wrap: wrap;
	}

	.buttons {
		display: flex;
		gap: 5px;
	}

	.buttons span {
		width: 4em;
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
