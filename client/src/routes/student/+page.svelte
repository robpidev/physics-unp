<script>
	import { onMount } from 'svelte';
	import { enhance } from '$app/forms';
	let usr;
	onMount(() => {
		usr = JSON.parse(localStorage.getItem('user'));
	});
	//const user_suscribe = user.subscribe((u) => (usr = u));
	//onDestroy(user_suscribe);
	export let data;
	let courses = null;
</script>

<div class="page">
	<div class="student">
		<span class="name">
			{usr?.names}
			{usr?.last_name1}
			{usr?.last_name2}
		</span>
		<span class="code">{usr?.id}</span>
	</div>
	<div class="course">
		{#if data.course != null}
			<span>Curso matriculado: </span>
			<span>{data.course?.name}</span>
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

				<button type="submit">Inscribirme</button>
			</form>
		{/if}
	</div>
</div>

<style>
	.page {
		margin: 1em;
		display: flex;
		flex-wrap: wrap;
		gap: 20px;
		/*background: #f9f;*/
	}

	.name {
		font-size: 1.3em;
		font-weight: 700;
	}

	.student,
	.course {
		height: min-content;
		border: solid 1px var(--border);
		box-shadow:
			0 4px 6px -1px rgb(0 0 0 / 0.1),
			0 2px 4px -2px rgb(0 0 0 / 0.1);
		display: flex;
		padding: 1em;
		border-radius: 12px;
		flex-grow: 1;
		flex-direction: column;
		min-width: 240px;
		gap: 1em;
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
		max-width: min-content;
		margin: 1em auto;
	}
</style>
