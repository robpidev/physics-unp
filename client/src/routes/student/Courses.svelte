<script>
	import { enhance } from '$app/forms';
	import Groups from './Groups.svelte';
	let courses;
	let selected = [];

	async function coursesAvilables() {
		const resp = await fetch('/student/avilables');
		courses = await resp.json();
	}
</script>

{#if courses}
	<form action="?/enroll" method="post" use:enhance>
		<ul class="courses">
			{#if courses.length > 0}
				<p>Elige un curso:</p>
				{#each courses as course}
					<label>
						<input
							type="radio"
							name="course_id"
							required
							value={course.id}
							disabled={course.enrolled == course.places}
						/>
						<span class="name">{course.name}</span>
						<div class="info">
							<span>Inscritos: </span>
							<span class="enrolled">{course.enrolled}</span>
							/ <span class="places">{course.places}</span>
						</div>
					</label>
				{/each}
			{:else}
				<p>No hay cursos disponibles</p>
			{/if}
		</ul>

		<input type="text" name="ocupated_groups" bind:value={selected} hidden />
		<Groups bind:selected />
		<button disabled={selected.length == 0} type="submit">Inscribirme</button>
	</form>
{:else}
	<!-- else content here -->
	<p>No hay cursos matriculados</p>
	<button on:click={coursesAvilables}>Inscribirme en un curso</button>
{/if}

<style>
	ul {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		gap: 5px;
		flex-direction: column;
	}

	.info {
		display: flex;
		align-items: center;
		gap: 0.2em;
	}

	.enrolled {
		width: 2em;
		text-align: right;
		font-weight: 600;
		color: var(--primary);
	}

	.places {
		width: 1.5em;
		text-align: right;
		color: green;
		font-weight: 600;
	}

	button {
		max-width: 200px;
	}

	label {
		display: flex;
		align-items: center;
		border-radius: 5px;
		padding: 0em 0.5em;
		background: var(--bg);
	}

	input:focus {
		outline: none;
	}

	.name {
		flex-grow: 1;
	}

	label:hover {
		background: var(--color-200);
	}

	button[type='submit'] {
		margin-top: 1em;
	}
</style>
