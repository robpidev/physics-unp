<script>
	import { onMount } from 'svelte';
	import { enhance } from '$app/forms';
	let usr;

	export let data;
	onMount(() => {
		usr = JSON.parse(localStorage.getItem('user'));
	});
	//const user_suscribe = user.subscribe((u) => (usr = u));
	//onDestroy(user_suscribe);
	let courses = null;
</script>

<div class="page">
	<div class="info">
		<span class="name">
			{usr?.names}
			{usr?.last_name1}
			{usr?.last_name2}
		</span>
		<span class="code">{usr?.id}</span>
	</div>

	<section>
		<h2>Cursos matrículados</h2>
		<hr />

		{#if data?.courses != null}
			<ul class="courses">
				{#each data?.courses as course}
					<li class="course">
						<span class="course-name">{course?.name}</span>
						<button>Ver notas</button>
					</li>
				{/each}
			</ul>
		{:else if courses === null}
			<span>No hay curso matriculado</span>
			<form
				method="POST"
				action="?/courses"
				use:enhance={() => {
					return async ({ result }) => {
						courses = result.data.courses;
					};
				}}
			>
				<button>Inscribirme</button>
			</form>
		{:else}
			<span class="title">Escoge un curso</span>
			<form class="courses" action="?/enroll" method="POST" use:enhance>
				{#each courses as course, id}
					<label>
						<input type="radio" name="course_id" value={courses[id].id} required />
						<span>{course.name}</span>
					</label>
				{/each}

				{#if courses.length === 0}
					<span>No hay cursos disponibles</span>
				{:else}
					<button type="submit" disabled={courses.length === 0}>Inscribirme</button>
				{/if}
			</form>
		{/if}
	</section>
</div>

<style>
	.info {
		display: flex;
		width: 100%;
		align-items: center;
		justify-content: space-between;
	}
	section {
		width: 100%;
		padding: 1em;
	}

	.page {
		width: 100%;
		max-width: 1000px;
	}

	.name {
		font-size: 1.3em;
		font-weight: 700;
	}

	ul {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 1em;
	}

	.course {
		display: flex;
		background: var(--bg);
		justify-content: space-between;
		align-items: center;
		padding: 0 0.5em;
		border-radius: 5px;
	}

	.courses {
		display: flex;
		flex-direction: column;
		gap: 1em;
		position: relative;
	}

	.title {
		color: var(--primary);
		font-weight: 700;
	}

	button {
		background: inherit;
		color: var(--primary);
	}

	button:hover {
		color: var(--color-600);
	}

	button:active {
		color: var(--color-700);
	}
</style>
