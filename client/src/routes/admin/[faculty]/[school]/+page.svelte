<script>
	export let data;
	import { enhance } from '$app/forms';
	import { page } from '$app/stores';
	import { breadcrum } from '$lib/stores.js';
	let course = '';

	let course_delete = '';
	let delete_id = -1;
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
					class={delete_id == id ? 'course-delete' : ''}
					data-sveltekit-preload-data="false"
					on:click={() => {
						breadcrum.update((path) => [...path, { name: course.name, url: course.id }]);
					}}
					href="/admin/{$page.params.faculty}/{$page.params.school}/{course.id}">{course.name}</a
				>

				<div class="course-info">
					<span class="enrolled">{course.enrolled}</span>
					/
					<span class="places">{course.places}</span>

					<button on:click={() => (delete_id >= 0 ? (delete_id = -1) : (delete_id = id))}>
						{delete_id == id ? 'Cancelar' : 'Eliminar'}
					</button>
				</div>
			</li>
		{/each}
	</ul>

	{#if delete_id >= 0}
		<hr />
		<div>
			<form
				method="POST"
				class="delete"
				action="?/delete"
				use:enhance={() => {
					return async ({ update }) => {
						await update();
						delete_id = -1;
					};
				}}
			>
				<input
					bind:value={course_delete}
					type="text"
					placeholder="Para completar la eliminación ingrese el nombre del curso"
				/>
				<input type="text" name="course_id" value={data.courses[delete_id].id} hidden />
				<button on:click={() => (delete_id = -1)}>Cancelar</button>
				<button
					class="btn-delete"
					type="submit"
					disabled={data.courses[delete_id].name != course_delete}
				>
					Eliminar
				</button>
			</form>
		</div>
	{/if}
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
		border-radius: 5px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.course-info {
		min-width: 40px;
		margin-right: 1em;
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

	a {
		padding: 0.5em 1em;
		width: 100%;
	}

	.delete {
		width: 100%;
		display: flex;
		gap: 0.5em;
	}

	.delete input {
		flex: 1;
	}

	.btn-delete {
		background: red;
	}

	.btn-delete:hover {
		background: darkred;
	}

	.btn-delete:disabled {
		background: red;
		opacity: 0.5;
	}

	.course-delete {
		background: red;
		color: white;
		border-radius: 5px 0 0 5px;
	}

	.course-info button {
		color: red;
		background: initial;
		padding: 0 2em 0 1em;
		height: 0;
		margin: 0;
		width: 8em;
	}

	.course-info button:hover {
		text-decoration: underline;
	}
</style>
