<script>
	export let student;
</script>

<h3>Detalles de evaluaciones</h3>

<div class="student">
	<div class="codigo">
		<span class="label">Código: </span>
		<span class="data">{student.id}</span>
	</div>
	<div class="codigo">
		<span class="label">Nombre: </span>
		<span class="data">{student.name}</span>
	</div>
</div>

<div class="details">
	<span class="entry">Entrada</span>
	<span class="prac">Informes</span>
	<div></div>
	{#each [1, 2, 3, 4, 5] as i}
		<span class="p{i} head">Práctica {i}</span>
	{/each}

	{#each student.scores as score}
		<div class="cell" style="grid-area: {score.ev_type}{score.number}">
			{score.score}
		</div>
	{/each}
</div>

<style>
	span {
		min-width: 90px;
	}

	.student {
		display: flex;
		gap: 1em;
		margin-bottom: 5px;
		flex-wrap: wrap;
	}

	.label {
		padding: 0 5px;
		border-radius: 3px;
	}

	.details {
		display: grid;
		justify-items: center;
		gap: 1px;
		grid-template-areas:
			'v    p1        p2        p3        p4        p5'
			'test test1     test2     test3     test4     test5'
			'prac practice1 practice2 practice3 practice4 practice5';

		border: solid 1px var(--border);
		border-radius: 3px;
		padding: 1px;
		overflow-x: auto;
		margin-bottom: 1em;
	}

	.type {
		justify-self: start;
		font-weight: 600;
	}
	.entry,
	.prac {
		font-weight: bold;
		justify-self: start;
		font-weight: 600;
		background: var(--color-200);
		border-radius: 3px;
	}

	.entry {
		grid-area: test;
	}

	.prac {
		grid-area: prac;
	}

	.cell {
		display: flex;
		justify-content: right;
		padding: 0 0.5em;
		width: 100%;
		background: var(--bg);
		border-radius: 3px;
		user-select: none;
	}

	.head {
		display: flex;
		justify-content: center;
		border-radius: 3px;
		width: 100%;
		font-weight: 600;
		background: var(--color-200);
		user-select: none;
	}
</style>
