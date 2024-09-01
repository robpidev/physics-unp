<script>
	export let data;
	import { enhance } from '$app/forms';
	import { page } from '$app/stores';
	let course = '';
</script>

<section class="add">
	<form method="post" action="?/add" use:enhance>
		<label>
			<span>Nombre del curso: </span>
			<input type="text" bind:value={course} name="course" />

			<button
				type="submit"
				disabled={data.courses.some((s) => {
					let resp = s.name.toUpperCase() == course.trim().toUpperCase() || course == '';
					return resp;
				})}>Agregar</button
			>
		</label>
	</form>
</section>

<section class="courses">
	<ul class="courses">
		{#each data.courses as course, id}
			<li class="course">
				<a
					data-sveltekit-preload-data="false"
					href="/admin/{$page.params.faculty}/{$page.params.school}/{course.id}">{course.name}</a
				>
			</li>
		{/each}
	</ul>
</section>

<style>
	.courses {
		list-style: none;
	}
	.course {
		border: solid 1px var(--border);
		margin-bottom: 10px;
		padding: 0.5em 1em;
		border-radius: 8px;
	}
</style>
