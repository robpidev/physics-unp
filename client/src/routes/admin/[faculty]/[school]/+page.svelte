<script>
	export let data;
	import { enhance } from '$app/forms';
	let course = '';
</script>

<section class="add">
	<form method="post" action="?/add" use:enhance>
		<label>
			<span>Nombre del curso: </span>
			<input type="text" bind:value={course} name="course" />

			<button
				type="submit"
				disabled={data.schools.some((s) => {
					let resp = s.name.toUpperCase() == course.trim().toUpperCase();
					console.log(resp);
					return resp;
				})}>Agregar</button
			>
		</label>
	</form>
</section>

<section class="courses">
	<ul>
		{#each data.schools as school, id}
			<li><a href="/admin">{school.name}</a></li>
		{/each}
	</ul>
</section>
