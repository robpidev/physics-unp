<script>
	export let data;
	import { enhance } from '$app/forms';
	import { page } from '$app/stores';
	let course = '';
</script>

<section class="courses">
	<form method="post" action="?/add" use:enhance>
		<span>Agreagar: </span>
		<input placeholder="Nombre del curso" type="text" bind:value={course} name="course" />
		<input placeholder="plazas" name="places" type="number" min="0" max="256" />
		<button
			type="submit"
			disabled={data.courses.some((s) => {
				let resp = s.name.toUpperCase() == course.trim().toUpperCase() || course == '';
				return resp;
			})}>Agregar</button
		>
	</form>

	<hr />

	<ul>
		<li class="head">
			<span>Curso</span>
			<span>Inscritos</span>
		</li>
		{#each data.courses as course, id}
			<li class="course">
				<a
					data-sveltekit-preload-data="false"
					href="/admin/{$page.params.faculty}/{$page.params.school}/{course.id}">{course.name}</a
				>

				<div class="course-info">
					<span class="enrolled">{course.enrolled}</span>
					/
					<span class="places">{course.places}</span>
				</div>
			</li>
		{/each}
	</ul>
</section>

<style>
	section {
		padding: 1.5em;
	}

	ul {
		list-style: none;
		padding: 0;
	}

	.course {
		margin-bottom: 3px;
		background: var(--bg);
		padding: 0.5em 1em;
		border-radius: 5px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.course-info {
		min-width: 40px;
		display: flex;
		justify-content: space-between;
	}

	.enrolled {
		color: var(--primary);
		font-weight: 600;
	}

	.places {
		color: green;
		font-weight: 600;
		margin-right: 5px;
	}

	.head {
		display: flex;
		justify-content: space-between;
		padding: 0.2em 0.5em;
	}
</style>
