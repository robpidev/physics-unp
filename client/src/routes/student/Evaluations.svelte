<script>
	import { onMount } from 'svelte';
	let average = 0;
	export let evaluations;

	onMount(() => {
		const tw = evaluations.tests[0].weight / 100;
		const pw = evaluations.tests[1].weight / 100;
		let test = 0;
		let totalTests = 0;
		let practice = 0;
		let totalPractice = 0;

		evaluations.evaluations.forEach((score) => {
			if (score.ev_type === 'test') {
				test += score.score;
				totalTests += 1;
			} else if (score.ev_type === 'practice') {
				practice += score.score;
				totalPractice += 1;
			}
		});
		average = (test / totalTests) * tw + (practice / totalPractice) * pw;
		average = Math.round(average * 100) / 100;

		console.log(average);
	});
</script>

<div class="title">
	<span class="course">Evaluaciones: {evaluations?.name}</span>
	<span class="average" style="color: {average >= 10.5 ? 'green' : 'red'}">Promedio: {average}</span
	>
</div>
<hr />
<div class="details">
	<span class="entry">Entrada ({evaluations.tests[0].weight}%)</span>
	<span class="prac">Informes ({evaluations.tests[1].weight}%)</span>
	<div></div>
	{#each [1, 2, 3, 4, 5] as i}
		<span class="p{i} head">Práctica {i}</span>
	{/each}

	{#each evaluations.evaluations as score}
		<button
			class="cell"
			style="
        grid-area: {score.ev_type}{score.number};
        color: {score.score >= 10.5 ? 'green' : 'red'};
      "
		>
			{score.score}
		</button>
	{/each}
</div>

<style>
	.details {
		display: grid;
		justify-items: center;
		gap: 1px;
		grid-template-areas:
			'v    p1        p2        p3        p4        p5'
			'test test1     test2     test3     test4     test5'
			'prac practice1 practice2 practice3 practice4 practice5';

		border-radius: 3px;
		padding: 1px;
		overflow-x: auto;
		margin-bottom: 1em;
		margin-top: 1em;
	}

	.type {
		justify-self: start;
		font-weight: 600;
	}
	.entry,
	.prac {
		width: 100%;
		font-weight: bold;
		justify-self: start;
		font-weight: 600;
		background: var(--color-200);
		border-radius: 3px;
		padding: 0 0.5em;
	}

	.entry {
		grid-area: test;
	}

	.prac {
		grid-area: prac;
	}

	.cell {
		color: black;
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

	.title {
		display: flex;
	}

	.course {
		flex-grow: 1;
	}
</style>
