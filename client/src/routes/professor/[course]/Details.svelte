<script>
	import { enhance } from '$app/forms';
	export let student;
	export let form;

	// Se puede mejorar
	export let available;

	let newScore = '';
	let id;
	let ev_type = '';
	let number = '';
	let updating = false;
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

<p>{available}</p>

<hr />
<div class="details">
	<span class="entry">Entrada</span>
	<span class="prac">Informes</span>
	<div></div>
	{#each [1, 2, 3, 4, 5] as i}
		<span class="p{i} head">Práctica {i}</span>
	{/each}

	{#each student.scores as score}
		<button
			on:click={() => {
				newScore = score.score;
				id = score.id;
				ev_type = score.ev_type;
				number = score.number;
			}}
			active={score.ev_type == ev_type && score.number == number}
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

{#if available}
	<hr />

	<div class="scores">
		{#if id != null}
			<form
				method="post"
				action="?/update_score"
				use:enhance={() => {
					updating = true;
					return async ({ update }) => {
						await update();
						updating = false;
						id = null;
						number = null;
						ev_type = null;
						newScore = null;
					};
				}}
			>
				<label>
					Nota:
					<input type="number" name="score" bind:value={newScore} />
				</label>
				<input type="text" name="ev_id" hidden bind:value={id} />
				<input type="number" name="number" hidden bind:value={number} />
				{#if updating}
					<input type="submit" value="Actualizando..." disabled />
				{:else}
					<input
						type="submit"
						value="Actualizar"
						disabled={newScore < 0 || newScore > 20 || newScore == undefined}
					/>
					<button
						class="cancel"
						on:click={() => {
							newScore = '';
							id = null;
							number = null;
						}}>Nueva Nota</button
					>
				{/if}
			</form>
		{:else}
			<form
				method="post"
				class="new_score"
				action="?/add_score"
				use:enhance={() => {
					updating = true;
					return async ({ update }) => {
						await update();
						updating = false;
						number = null;
						ev_type = null;
						newScore = null;
					};
				}}
			>
				<label>
					Nota: <input type="number" name="score" bind:value={newScore} required />
				</label>
				<label>
					Práctica: <input type="number" name="number" bind:value={number} required />
				</label>
				<label>
					<input type="radio" name="ev_type" bind:group={ev_type} value="test" required />
					Entrada
				</label>
				<label>
					<input type="radio" name="ev_type" bind:group={ev_type} value="practice" required />
					Informes
				</label>
				<input type="number" name="student_id" hidden bind:value={student.id} />
				{#if updating}
					<input type="submit" value="Actualizando..." disabled />
				{:else}
					<input
						type="submit"
						value="Agregar"
						disabled={student.scores.some((s) => {
							if (s.number == number && s.ev_type == ev_type) {
								return true;
							}
							return false;
						}) ||
							newScore < 0 ||
							newScore > 20 ||
							newScore == undefined ||
							number == undefined}
					/>
				{/if}
			</form>
		{/if}
	</div>

	<p class="error">
		{form?.error ?? ''}
	</p>

	<!-- content here -->
{/if}

<style>
	.new_score {
		display: flex;
		gap: 1em;
		flex-wrap: wrap;
	}

	.new_score input {
		margin-left: 0.5em;
	}
	.new_score label {
		display: flex;
		align-items: center;
	}
	span {
		min-width: 90px;
	}

	.scores input[type='number'] {
		width: 3em;
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

	button[active='true'] {
		background: var(--color-400);
		color: white;
	}

	input[type='number']::-webkit-inner-spin-button,
	input[type='number']::-webkit-outer-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	.cancel {
		color: red;
		background: initial;
	}

	.cancel:hover {
		text-decoration: underline;
	}

	.error {
		color: red;
	}
</style>
